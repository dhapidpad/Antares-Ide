use std::cmp::min;

use floem::{
    prelude::SignalTrack,
    reactive::{RwSignal, Scope, SignalGet, SignalUpdate, SignalWith},
};
use lapce_core::{
    selection::{SelRegion, Selection},
    word::WordCursor,
};
use lapce_xi_rope::{
    Cursor, Interval, Rope,
    find::{CaseMatching, find, is_multiline_regex},
};
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};

/// Maximum compiled size of a user supplied regex (protects against
/// pathological patterns).
const REGEX_SIZE_LIMIT: usize = 1_000_000;

/// Indicates what changed in the find state.
#[derive(PartialEq, Debug, Clone)]
pub enum FindProgress {
    /// Incremental find is done/not running.
    Ready,

    /// The find process just started.
    Started,

    /// Incremental find is in progress. Keeps tracked of already searched range.
    InProgress(Selection),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindStatus {
    /// Identifier for the current search query.
    id: usize,

    /// The current search query.
    chars: Option<String>,

    /// Whether the active search is case matching.
    case_sensitive: Option<bool>,

    /// Whether the search query is considered as regular expression.
    is_regex: Option<bool>,

    /// Query only matches whole words.
    whole_words: Option<bool>,

    /// Total number of matches.
    matches: usize,

    /// Line numbers which have find results.
    lines: Vec<usize>,
}

#[derive(Clone)]
pub struct FindSearchString {
    pub content: String,
    pub regex: Option<Regex>,
}

#[derive(Clone)]
pub struct Find {
    pub rev: RwSignal<u64>,
    /// If the find is shown
    pub visual: RwSignal<bool>,
    /// The currently active search string.
    pub search_string: RwSignal<Option<FindSearchString>>,
    /// The case matching setting for the currently active search.
    pub case_matching: RwSignal<CaseMatching>,
    /// Query matches only whole words.
    pub whole_words: RwSignal<bool>,
    /// The search query should be considered as regular expression.
    pub is_regex: RwSignal<bool>,
    /// replace editor is shown
    pub replace_active: RwSignal<bool>,
    /// replace editor is focused
    pub replace_focus: RwSignal<bool>,
    /// Triggered by changes in the search string
    pub triggered_by_changes: RwSignal<bool>,
}

// ───────────────────────── Scanner (internal) ─────────────────────────

/// What the scanner should do after a match was reported to the callback.
enum Scan {
    /// Keep searching after this match.
    Next,
    /// Stop the search.
    Stop,
    /// Continue searching from the given offset.
    Seek(usize),
}

/// One configured search over a rope. All the "find the next match" loops in
/// this file go through [`Scanner::scan`], so edge cases (empty matches,
/// whole-word filtering, UTF-8 boundaries) are handled in exactly one place.
struct Scanner<'a> {
    text: &'a Rope,
    search: &'a FindSearchString,
    case_matching: CaseMatching,
    whole_words: bool,
}

impl<'a> Scanner<'a> {
    /// Scans `from..to` and reports every accepted match to `on_match`.
    ///
    /// `cursor_text` is the rope the search cursor walks over. It is usually
    /// `self.text`, but callers that only need a prefix of the text can pass
    /// a cheaper sub-rope.
    fn scan(
        &self,
        cursor_text: &Rope,
        from: usize,
        to: usize,
        mut on_match: impl FnMut(usize, usize) -> Scan,
    ) {
        let text = self.text;
        let lines_from = |pos: usize| text.lines_raw(pos.min(to)..to);

        let mut find_cursor = Cursor::new(cursor_text, from);
        let mut raw_lines = lines_from(from);

        while let Some(start) = find(
            &mut find_cursor,
            &mut raw_lines,
            self.case_matching,
            &self.search.content,
            self.search.regex.as_ref(),
        ) {
            let end = find_cursor.pos();

            let accepted = !self.whole_words
                || Find::is_matching_whole_words(text, start, end);
            let flow = if accepted {
                on_match(start, end)
            } else {
                Scan::Next
            };

            match flow {
                Scan::Stop => break,
                Scan::Seek(pos) => {
                    find_cursor.set(pos);
                    raw_lines = lines_from(pos);
                    continue;
                }
                Scan::Next => {}
            }

            // An empty match (e.g. regex `a*`) does not advance the cursor.
            // Move it manually, otherwise the search would repeat forever.
            if start == end {
                if end + 1 >= text.len() {
                    break;
                }
                let next = text
                    .at_or_next_codepoint_boundary(end + 1)
                    .unwrap_or(end + 1);
                if next > to {
                    break;
                }
                find_cursor.set(next);
            }

            raw_lines = lines_from(find_cursor.pos());
        }
    }

    /// First match in `from..to`. If `after` is set, matches starting at or
    /// before that offset are skipped (so the current selection is not found
    /// again).
    fn first(
        &self,
        from: usize,
        to: usize,
        after: Option<usize>,
    ) -> Option<(usize, usize)> {
        let mut found = None;
        self.scan(self.text, from, to, |start, end| {
            if after.map_or(true, |a| start > a) {
                found = Some((start, end));
                Scan::Stop
            } else {
                Scan::Next
            }
        });
        found
    }

    /// Last match in `from..to` whose start offset satisfies `keep`.
    fn last(
        &self,
        from: usize,
        to: usize,
        keep: impl Fn(usize) -> bool,
    ) -> Option<(usize, usize)> {
        let mut found = None;
        self.scan(self.text, from, to, |start, end| {
            if keep(start) {
                found = Some((start, end));
            }
            Scan::Next
        });
        found
    }
}

// ───────────────────────── Find ─────────────────────────

impl Find {
    pub fn new(cx: Scope) -> Self {
        let find = Self {
            rev: cx.create_rw_signal(0),
            visual: cx.create_rw_signal(false),
            search_string: cx.create_rw_signal(None),
            case_matching: cx.create_rw_signal(CaseMatching::CaseInsensitive),
            whole_words: cx.create_rw_signal(false),
            is_regex: cx.create_rw_signal(false),
            replace_active: cx.create_rw_signal(false),
            replace_focus: cx.create_rw_signal(false),
            triggered_by_changes: cx.create_rw_signal(false),
        };

        // Rebuild the regex when the regex toggle or the case setting changes.
        // (The compiled regex bakes in case sensitivity, so it goes stale
        // otherwise.)
        {
            let find = find.clone();
            cx.create_effect(move |_| {
                find.is_regex.track();
                find.case_matching.track();
                let content = find.search_string.with_untracked(|s| {
                    s.as_ref().map(|s| s.content.clone()).unwrap_or_default()
                });
                if !content.is_empty() {
                    find.apply_search(&content, true);
                }
            });
        }

        // Bump the revision whenever anything that affects results changes.
        {
            let find = find.clone();
            cx.create_effect(move |_| {
                find.search_string.track();
                find.case_matching.track();
                find.whole_words.track();
                find.rev.update(|rev| {
                    *rev += 1;
                });
            });
        }

        find
    }

    /// Returns `true` if case sensitive, otherwise `false`
    pub fn case_sensitive(&self, tracked: bool) -> bool {
        let case_matching = if tracked {
            self.case_matching.get()
        } else {
            self.case_matching.get_untracked()
        };
        matches!(case_matching, CaseMatching::Exact)
    }

    /// Sets find case sensitivity.
    pub fn set_case_sensitive(&self, case_sensitive: bool) {
        if self.case_sensitive(false) == case_sensitive {
            return;
        }

        self.case_matching.set(if case_sensitive {
            CaseMatching::Exact
        } else {
            CaseMatching::CaseInsensitive
        });
    }

    /// Sets the active search string (shows the find bar if it is hidden).
    /// An empty string clears the search.
    pub fn set_find(&self, search_string: &str) {
        self.apply_search(search_string, false);
    }

    /// Shared implementation of [`Find::set_find`]. With `force`, a regex
    /// search is rebuilt even if the query text did not change.
    fn apply_search(&self, search_string: &str, force: bool) {
        if search_string.is_empty() {
            self.search_string.set(None);
            return;
        }

        if !self.visual.get_untracked() {
            self.visual.set(true);
        }

        let is_regex = self.is_regex.get_untracked();

        let unchanged = self.search_string.with_untracked(|search| {
            search.as_ref().is_some_and(|s| {
                s.content == search_string && s.regex.is_some() == is_regex
            })
        });

        // Only regex searches depend on the case setting at build time.
        if unchanged && !(force && is_regex) {
            return;
        }

        let regex = if is_regex {
            Self::build_regex(search_string, self.case_sensitive(false))
        } else {
            None
        };

        self.triggered_by_changes.set(true);
        self.search_string.set(Some(FindSearchString {
            content: search_string.to_string(),
            regex,
        }));
    }

    /// Builds a regex from untrusted user input. Returns `None` if the
    /// pattern is invalid or too large.
    fn build_regex(pattern: &str, case_sensitive: bool) -> Option<Regex> {
        RegexBuilder::new(pattern)
            .size_limit(REGEX_SIZE_LIMIT)
            .case_insensitive(!case_sensitive)
            .build()
            .ok()
    }

    /// Finds the next (or previous, if `reverse`) match relative to `offset`.
    /// With `wrap`, the search continues from the other end of the document.
    pub fn next(
        &self,
        text: &Rope,
        offset: usize,
        reverse: bool,
        wrap: bool,
    ) -> Option<(usize, usize)> {
        if !self.visual.get_untracked() {
            self.visual.set(true);
        }

        let case_matching = self.case_matching.get_untracked();
        let whole_words = self.whole_words.get_untracked();

        self.search_string.with_untracked(|search| {
            let scanner = Scanner {
                text,
                search: search.as_ref()?,
                case_matching,
                whole_words,
            };
            let len = text.len();

            if !reverse {
                scanner.first(offset, len, Some(offset)).or_else(|| {
                    if wrap {
                        scanner.first(0, offset, None)
                    } else {
                        None
                    }
                })
            } else {
                scanner.last(0, offset, |start| start < offset).or_else(|| {
                    if wrap {
                        scanner.last(offset, len, |start| start > offset)
                    } else {
                        None
                    }
                })
            }
        })
    }

    /// Checks if the start and end of a match is matching whole words.
    fn is_matching_whole_words(text: &Rope, start: usize, end: usize) -> bool {
        // An empty match is never a "word"; this also avoids `end - 1`
        // underflowing at offset 0.
        if start >= end {
            return false;
        }

        let mut word_end_cursor = WordCursor::new(text, end - 1);
        let mut word_start_cursor = WordCursor::new(text, start + 1);

        word_start_cursor.prev_code_boundary() == start
            && word_end_cursor.next_code_boundary() == end
    }

    /// Returns `true` if the search query is a multi-line regex.
    pub fn is_multiline_regex(&self) -> bool {
        self.search_string.with_untracked(|search| {
            search.as_ref().is_some_and(|search| {
                search.regex.is_some() && is_multiline_regex(&search.content)
            })
        })
    }

    /// Collects the occurrences of `search` in `start..end` into
    /// `occurrences`.
    ///
    /// With `include_slop` the region is widened by twice the query length
    /// on both sides (twice, because case matching may increase the length of
    /// an occurrence), so matches straddling the region edges are found.
    #[allow(clippy::too_many_arguments)]
    pub fn find(
        text: &Rope,
        search: &FindSearchString,
        start: usize,
        end: usize,
        case_matching: CaseMatching,
        whole_words: bool,
        include_slop: bool,
        occurrences: &mut Selection,
    ) {
        let slop = if include_slop {
            search.content.len() * 2
        } else {
            0
        };

        let expanded_start = start.saturating_sub(slop);
        let expanded_end = min(end.saturating_add(slop), text.len());
        let from = text
            .at_or_prev_codepoint_boundary(expanded_start)
            .unwrap_or(0);
        let to = text
            .at_or_next_codepoint_boundary(expanded_end)
            .unwrap_or_else(|| text.len());

        // The cursor only has to walk the text up to the end of the region.
        let mut to_cursor = Cursor::new(text, to);
        let _ = to_cursor.next_leaf();
        let sub_text = text.subseq(Interval::new(0, to_cursor.pos()));

        let scanner = Scanner {
            text,
            search,
            case_matching,
            whole_words,
        };

        scanner.scan(&sub_text, from, to, |start, end| {
            let region = SelRegion::new(start, end, None);
            let (_, kept_end) = occurrences.add_range_distinct(region);

            // Ambiguous results (e.g. "aba" in "ababa"): the occurrence
            // closer to the beginning of the file wins. Re-align the search
            // to the occurrence that was kept.
            if kept_end != end {
                Scan::Seek(kept_end)
            } else {
                Scan::Next
            }
        });
    }

    /// Execute the search on the provided text in the range provided by
    /// `start` and `end`.
    pub fn update_find(
        &self,
        text: &Rope,
        start: usize,
        end: usize,
        include_slop: bool,
        occurrences: &mut Selection,
    ) {
        let Some(search) = self.search_string.get_untracked() else {
            return;
        };

        Self::find(
            text,
            &search,
            start,
            end,
            self.case_matching.get_untracked(),
            self.whole_words.get_untracked(),
            include_slop,
            occurrences,
        );
    }
}

#[derive(Clone)]
pub struct FindResult {
    pub find_rev: RwSignal<u64>,
    pub progress: RwSignal<FindProgress>,
    pub occurrences: RwSignal<Selection>,
    pub search_string: RwSignal<Option<FindSearchString>>,
    pub case_matching: RwSignal<CaseMatching>,
    pub whole_words: RwSignal<bool>,
    pub is_regex: RwSignal<bool>,
}

impl FindResult {
    pub fn new(cx: Scope) -> Self {
        Self {
            find_rev: cx.create_rw_signal(0),
            progress: cx.create_rw_signal(FindProgress::Started),
            occurrences: cx.create_rw_signal(Selection::new()),
            search_string: cx.create_rw_signal(None),
            case_matching: cx.create_rw_signal(CaseMatching::Exact),
            whole_words: cx.create_rw_signal(false),
            is_regex: cx.create_rw_signal(false),
        }
    }

    pub fn reset(&self) {
        self.progress.set(FindProgress::Started);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn literal(s: &str) -> FindSearchString {
        FindSearchString {
            content: s.to_string(),
            regex: None,
        }
    }

    fn scanner<'a>(
        text: &'a Rope,
        search: &'a FindSearchString,
        whole_words: bool,
    ) -> Scanner<'a> {
        Scanner {
            text,
            search,
            case_matching: CaseMatching::Exact,
            whole_words,
        }
    }

    #[test]
    fn first_skips_current_match() {
        let text = Rope::from("foo bar foo");
        let search = literal("foo");
        let s = scanner(&text, &search, false);
        assert_eq!(s.first(0, text.len(), None), Some((0, 3)));
        assert_eq!(s.first(0, text.len(), Some(0)), Some((8, 11)));
    }

    #[test]
    fn last_respects_filter() {
        let text = Rope::from("foo bar foo");
        let search = literal("foo");
        let s = scanner(&text, &search, false);
        assert_eq!(s.last(0, text.len(), |start| start < 8), Some((0, 3)));
        assert_eq!(s.last(0, text.len(), |_| true), Some((8, 11)));
    }

    #[test]
    fn whole_words_rejects_partial_matches() {
        let text = Rope::from("foobar foo");
        let search = literal("foo");
        let s = scanner(&text, &search, true);
        assert_eq!(s.first(0, text.len(), None), Some((7, 10)));
    }

    #[test]
    fn whole_words_empty_match_does_not_panic() {
        let text = Rope::from("abc");
        assert!(!Find::is_matching_whole_words(&text, 0, 0));
    }
}