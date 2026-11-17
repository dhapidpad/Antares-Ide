#!/usr/bin/env bash
# Antares: gated commit -> push -> watch GitHub Actions for the EXACT pushed SHA.
#
# Safety properties (see .agent/rules/03-ci-loop.md, 08-github-automation.md, 14-git-safety.md):
#   * only branches named agy/*; never main/master; never force-push
#   * origin must not be lapce/lapce and must not embed credentials
#   * secret/forbidden-file scan before commit and before push (matches are never printed)
#   * local gate must pass before anything is pushed (unless --no-local-gate, reported NOT VERIFIED)
#   * the watched run must belong to the pushed commit SHA
#   * stops after 5 failed CI attempts on the same branch (blocker, not blind looping)
#
# Exit codes: 0 CI green | 1 CI failed | 2 precondition/gate/push failure
#             3 no matching run / timeout | 4 attempt limit reached (record blocker)
set -euo pipefail

MESSAGE=""
NO_LOCAL=0
DRY_RUN=0
WATCH_ONLY=0
RESET_ATTEMPTS=0
MAX_ATTEMPTS=5
FIND_TIMEOUT=180
WATCH_TIMEOUT=2400
WORKFLOW_PREFIX="Antares"
COMMIT_RE='^(feat|fix|refactor|test|docs|ci|build|chore)\((agent|protocol|context|mcp|diff|policy|ui|ci|deps|rebrand)\): .+'

usage() {
  cat <<'EOF'
Usage: scripts/antares-push-and-watch.sh [options]

  --commit "type(scope): summary"   commit all pending changes first (Conventional Commits, <=72 chars)
  --dry-run                         run checks and scans only; no commit, no push
  --watch-only                      do not commit/push; watch the run for the current HEAD
  --no-local-gate                   skip local gate (result is reported NOT VERIFIED)
  --reset-attempts                  clear the failed-attempt counter for this branch
  --max-attempts N                  failed CI attempts allowed per branch (default 5)
  --find-timeout SEC                wait for the run to appear (default 180)
  --watch-timeout SEC               max time to watch a run (default 2400)
  -h, --help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit) MESSAGE="${2:?--commit needs a message}"; shift 2 ;;
    --dry-run) DRY_RUN=1; shift ;;
    --watch-only) WATCH_ONLY=1; shift ;;
    --no-local-gate) NO_LOCAL=1; shift ;;
    --reset-attempts) RESET_ATTEMPTS=1; shift ;;
    --max-attempts) MAX_ATTEMPTS="${2:?}"; shift 2 ;;
    --find-timeout) FIND_TIMEOUT="${2:?}"; shift 2 ;;
    --watch-timeout) WATCH_TIMEOUT="${2:?}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown option: $1" >&2; usage >&2; exit 2 ;;
  esac
done

die() { printf 'ERROR: %s\n' "$1" >&2; exit "${2:-2}"; }
info() { printf '\n==> %s\n' "$*"; }

ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || die "Run inside the Antares Git repository."
cd "$ROOT"
command -v gh >/dev/null 2>&1 || die "GitHub CLI (gh) is required. Install it and run: gh auth login"
gh auth status >/dev/null 2>&1 || die "gh is not authenticated. Run: gh auth login (never paste a token into chat or files)."

# ---------- branch / remote safety ----------
BRANCH="$(git branch --show-current)"
[[ -n "$BRANCH" ]] || die "Detached HEAD. Switch to an agy/<phase>-<topic> branch."
case "$BRANCH" in
  main|master) die "Refusing to push '$BRANCH'. Use agy/<phase>-<topic>." ;;
  agy/*) ;;
  *) die "Branch '$BRANCH' does not match agy/<phase>-<topic>." ;;
esac

ORIGIN_URL="$(git remote get-url origin 2>/dev/null)" || die "No 'origin' remote."
if [[ "$ORIGIN_URL" =~ ://[^/@]+:[^@]*@ || "$ORIGIN_URL" =~ (ghp_|github_pat_) ]]; then
  die "origin URL embeds credentials. Remove them; use 'gh auth setup-git' instead."
fi
SLUG="$(printf '%s' "$ORIGIN_URL" | sed -E 's#^(git@[^:]+:|ssh://git@[^/]+/|https?://([^@/]+@)?[^/]+/)##; s#\.git$##')"
SLUG_LC="$(printf '%s' "$SLUG" | tr '[:upper:]' '[:lower:]')"
[[ "$SLUG_LC" != "lapce/lapce" ]] || die "origin is lapce/lapce. Push only to your Antares fork."
[[ "$SLUG" == */* ]] || die "Cannot derive owner/repo from origin."
PARENT="$(gh api "repos/$SLUG" --jq '.parent.full_name // "none"' 2>/dev/null || echo unknown)"
if [[ "$PARENT" != "lapce/lapce" ]]; then
  printf 'WARN: fork parent of %s is "%s" (expected lapce/lapce). Continuing.\n' "$SLUG" "$PARENT" >&2
fi
printf 'Repository: %s\nBranch:     %s\n' "$SLUG" "$BRANCH"

STATE_DIR="$ROOT/target/antares-ci"
mkdir -p "$STATE_DIR"
BRANCH_KEY="$(printf '%s' "$BRANCH" | tr '/ ' '__')"
ATTEMPTS_FILE="$STATE_DIR/attempts-$BRANCH_KEY"
[[ "$RESET_ATTEMPTS" == "1" ]] && rm -f "$ATTEMPTS_FILE"
ATTEMPTS=0
[[ -f "$ATTEMPTS_FILE" ]] && ATTEMPTS="$(cat "$ATTEMPTS_FILE" 2>/dev/null || echo 0)"
if (( ATTEMPTS >= MAX_ATTEMPTS )) && [[ "$WATCH_ONLY" == "0" ]]; then
  echo "BLOCKER: $ATTEMPTS failed CI attempts on $BRANCH. Stop editing blindly." >&2
  echo "Record the exact blocker and evidence in docs/STATUS.md, then ask the user. (--reset-attempts after a new root-cause hypothesis is approved.)" >&2
  exit 4
fi

# ---------- scans ----------
FORBIDDEN_NAMES='(^|/)\.env($|\.)|\.pem$|\.p12$|\.key$|(^|/)id_(rsa|ed25519)|(^|/)credentials|octomind-[0-9][^/]*\.tar\.gz$|(^|/)octomind$|session[^/]*\.(db|sqlite)$'
SECRET_RE='ghp_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{20,}|gho_[A-Za-z0-9]{30,}|AKIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{35}|-----BEGIN [A-Z ]*PRIVATE KEY-----|sk-[A-Za-z0-9]{32,}|xox[baprs]-[A-Za-z0-9-]{10,}'

scan_content() {  # stdin: unified diff; prints only counts, never matches
  local n
  n="$(grep -E '^\+' | grep -Ecs "$SECRET_RE" || true)"
  [[ "${n:-0}" == "0" ]]
}

# ---------- commit (optional) ----------
if [[ "$WATCH_ONLY" == "0" ]]; then
  STAGED_BY_DRY_RUN=0
  if [[ "$DRY_RUN" == "1" && -z "$MESSAGE" ]]; then MESSAGE="chore(ci): dry run"; STAGED_BY_DRY_RUN=1; fi
  if [[ -n "$(git status --porcelain)" ]]; then
    [[ -n "$MESSAGE" ]] || die "Uncommitted changes present. Re-run with --commit \"type(scope): summary\" or commit manually."
    (( ${#MESSAGE} <= 72 )) || die "Commit subject is ${#MESSAGE} chars (limit 72)."
    [[ "$MESSAGE" =~ $COMMIT_RE ]] || die "Commit message must match: type(scope): summary (types/scopes in AGENTS.antares.md §8)."
    info "Staging changes"
    git add -A
    if git diff --cached --name-only | grep -Eq "$FORBIDDEN_NAMES"; then
      git reset -q
      die "Staged files include forbidden names (env/keys/credentials/binaries/session DBs). Unstaged everything."
    fi
    if ! git diff --cached -U0 | scan_content; then
      git reset -q
      die "Possible secret detected in staged diff (content not shown). Unstaged everything; remove it and retry."
    fi
    git diff --cached --check || { git reset -q; die "git diff --check failed (whitespace)."; }
    git diff --cached --stat
  elif [[ -n "$MESSAGE" ]]; then
    echo "Working tree clean; --commit ignored."
  fi

  # ---------- local gate ----------
  if [[ "$NO_LOCAL" == "1" ]]; then
    echo "NOT VERIFIED: local gate skipped (--no-local-gate)."
  else
    info "Local gate"
    bash scripts/scaffold-check.sh
    if command -v cargo >/dev/null 2>&1; then
      bash scripts/antares-ci.sh fast
    else
      echo "NOT RUN: cargo not installed locally; Rust checks will run only in GitHub Actions."
    fi
  fi

  if [[ "$DRY_RUN" == "1" ]]; then
    [[ "$STAGED_BY_DRY_RUN" == "1" ]] && git reset -q
    info "Dry run complete: nothing committed or pushed."
    exit 0
  fi

  if [[ -n "$(git diff --cached --name-only)" ]]; then
    info "Committing"
    git commit -q -m "$MESSAGE"
  fi

  # ---------- pre-push scan of every unpushed commit ----------
  if git rev-parse --verify -q '@{u}' >/dev/null 2>&1; then
    RANGE="@{u}..HEAD"
  else
    BASE="$(git merge-base HEAD origin/main 2>/dev/null || git hash-object -t tree /dev/null)"
    RANGE="$BASE..HEAD"
  fi
  if git diff --name-only "$RANGE" | grep -Eq "$FORBIDDEN_NAMES"; then
    die "Unpushed commits contain forbidden file names. Not pushing."
  fi
  if ! git diff -U0 "$RANGE" | scan_content; then
    die "Possible secret in unpushed commits (content not shown). Not pushing."
  fi

  # ---------- push (never force) ----------
  info "Pushing $BRANCH to origin (no force)"
  git push --set-upstream origin "$BRANCH" || die "Push rejected. Do NOT force-push; fetch, inspect divergence and rebase/merge on the feature branch."
fi

SHA="$(git rev-parse HEAD)"
REMOTE_SHA="$(git ls-remote origin "refs/heads/$BRANCH" | awk '{print $1}')"
[[ "$REMOTE_SHA" == "$SHA" ]] || die "origin/$BRANCH ($REMOTE_SHA) is not HEAD ($SHA). Push first."
printf 'Pushed commit: %s\n' "$SHA"

# ---------- find runs for EXACT SHA ----------
info "Waiting for Actions runs on $SHA"
RUN_IDS=()
deadline=$((SECONDS + FIND_TIMEOUT))
while (( SECONDS < deadline )); do
  mapfile -t RUN_IDS < <(gh run list --repo "$SLUG" --branch "$BRANCH" --commit "$SHA" --limit 30 \
    --json databaseId,workflowName,headSha \
    --jq ".[] | select(.headSha==\"$SHA\") | select(.workflowName|startswith(\"$WORKFLOW_PREFIX\")) | .databaseId" 2>/dev/null || true)
  (( ${#RUN_IDS[@]} > 0 )) && break
  sleep 6
done
if (( ${#RUN_IDS[@]} == 0 )); then
  echo "No '$WORKFLOW_PREFIX*' workflow run found for $SHA within ${FIND_TIMEOUT}s." >&2
  echo "Check that Actions are enabled on the fork: https://github.com/$SLUG/actions" >&2
  echo "ANTARES_CI_RESULT sha=$SHA run=none conclusion=not-found"
  exit 3
fi

# ---------- watch ----------
OVERALL=0
for RUN_ID in "${RUN_IDS[@]}"; do
  info "Watching run $RUN_ID"
  if timeout "$WATCH_TIMEOUT" gh run watch "$RUN_ID" --repo "$SLUG" --exit-status --interval 15; then
    :
  else
    rc=$?
    [[ $rc -eq 124 ]] && echo "Timed out after ${WATCH_TIMEOUT}s watching run $RUN_ID." >&2
  fi
  META="$(gh run view "$RUN_ID" --repo "$SLUG" --json conclusion,status,url,headSha,workflowName \
    --jq '[.conclusion,.status,.url,.headSha,.workflowName]|@tsv')"
  IFS=$'\t' read -r CONCLUSION STATUS URL HSHA WNAME <<<"$META"
  [[ "$HSHA" == "$SHA" ]] || die "Run $RUN_ID belongs to $HSHA, not $SHA. Refusing to report it."
  printf 'ANTARES_CI_RESULT sha=%s run=%s workflow="%s" status=%s conclusion=%s url=%s\n' \
    "$SHA" "$RUN_ID" "$WNAME" "$STATUS" "${CONCLUSION:-none}" "$URL"
  if [[ "$CONCLUSION" != "success" ]]; then
    OVERALL=1
    LOG="$STATE_DIR/${SHA:0:12}-$RUN_ID-failed.log"
    gh run view "$RUN_ID" --repo "$SLUG" --log-failed >"$LOG" 2>&1 || true
    printf '\n--- failed-job log saved to %s (last 80 lines) ---\n' "target/antares-ci/$(basename "$LOG")"
    tail -n 80 "$LOG" || true
  fi
done

if (( OVERALL == 0 )); then
  rm -f "$ATTEMPTS_FILE"
  printf '\nCI GREEN for %s. Record the run URL + SHA in docs/STATUS.md (evidence rule).\n' "$SHA"
  exit 0
fi

ATTEMPTS=$((ATTEMPTS + 1))
printf '%s' "$ATTEMPTS" >"$ATTEMPTS_FILE"
printf '\nCI FAILED for %s (failed attempt %s/%s on %s).\n' "$SHA" "$ATTEMPTS" "$MAX_ATTEMPTS" "$BRANCH" >&2
echo "Next: follow .agent/workflows/fix-ci.md — find the FIRST causal error in the saved log, fix the root cause, then re-run this script." >&2
exit 1
