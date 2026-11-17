pub struct LapceIcons {}

impl LapceIcons {
    pub const ANTARES_AGENT: &'static str = "antares.agent";
    pub const ANTARES_MCP: &'static str = "antares.mcp";

    pub const WINDOW_CLOSE: &'static str = "window.close";
    pub const WINDOW_RESTORE: &'static str = "window.restore";
    pub const WINDOW_MAXIMIZE: &'static str = "window.maximize";
    pub const WINDOW_MINIMIZE: &'static str = "window.minimize";

    pub const LOGO: &'static str = "logo";
    pub const MENU: &'static str = "menu";
    pub const LINK: &'static str = "link";
    pub const ERROR: &'static str = "error";
    pub const ADD: &'static str = "add";
    pub const CLOSE: &'static str = "close";
    pub const REMOTE: &'static str = "remote";
    pub const PROBLEM: &'static str = "error";
    pub const DEBUG: &'static str = "debug";
    pub const DEBUG_ALT: &'static str = "debug_alt";
    pub const DEBUG_BREAKPOINT: &'static str = "debug_breakpoint";
    pub const DEBUG_SMALL: &'static str = "debug_small";
    pub const DEBUG_RESTART: &'static str = "debug_restart";
    pub const DEBUG_CONTINUE: &'static str = "debug_continue";
    pub const DEBUG_STEP_OVER: &'static str = "debug_step_over";
    pub const DEBUG_STEP_INTO: &'static str = "debug_step_into";
    pub const DEBUG_STEP_OUT: &'static str = "debug_step_out";
    pub const DEBUG_PAUSE: &'static str = "debug_pause";
    pub const DEBUG_STOP: &'static str = "debug_stop";
    pub const DEBUG_CONSOLE: &'static str = "debug_console";
    pub const DEBUG_DISCONNECT: &'static str = "debug_disconnect";
    pub const START: &'static str = "start";
    pub const RUN_ERRORS: &'static str = "run_errors";
    pub const UNSAVED: &'static str = "unsaved";
    pub const WARNING: &'static str = "warning";
    pub const TERMINAL: &'static str = "terminal";
    pub const SETTINGS: &'static str = "settings";
    pub const LIGHTBULB: &'static str = "lightbulb";
    pub const EXTENSIONS: &'static str = "extensions";
    pub const KEYBOARD: &'static str = "keyboard";
    pub const BREADCRUMB_SEPARATOR: &'static str = "breadcrumb_separator";
    pub const SYMBOL_COLOR: &'static str = "symbol_color";
    pub const TYPE_HIERARCHY: &'static str = "type_hierarchy";

    pub const FILE: &'static str = "file";
    pub const FILE_EXPLORER: &'static str = "file_explorer";
    pub const FILE_PICKER_UP: &'static str = "file_picker_up";

    pub const IMAGE_LOADING: &'static str = "image_loading";
    pub const IMAGE_ERROR: &'static str = "image_error";

    pub const SCM: &'static str = "scm.icon";
    pub const SCM_DIFF_MODIFIED: &'static str = "scm.diff.modified";
    pub const SCM_DIFF_ADDED: &'static str = "scm.diff.added";
    pub const SCM_DIFF_REMOVED: &'static str = "scm.diff.removed";
    pub const SCM_DIFF_RENAMED: &'static str = "scm.diff.renamed";
    pub const SCM_CHANGE_ADD: &'static str = "scm.change.add";
    pub const SCM_CHANGE_REMOVE: &'static str = "scm.change.remove";

    pub const FOLD: &'static str = "fold";
    pub const FOLD_UP: &'static str = "fold.up";
    pub const FOLD_DOWN: &'static str = "fold.down";

    pub const PALETTE_MENU: &'static str = "palette.menu";

    pub const DROPDOWN_ARROW: &'static str = "dropdown.arrow";

    pub const PANEL_FOLD_UP: &'static str = "panel.fold-up";
    pub const PANEL_FOLD_DOWN: &'static str = "panel.fold-down";

    pub const LOCATION_BACKWARD: &'static str = "location.backward";
    pub const LOCATION_FORWARD: &'static str = "location.forward";

    pub const ITEM_OPENED: &'static str = "item.opened";
    pub const ITEM_CLOSED: &'static str = "item.closed";

    pub const DIRECTORY_CLOSED: &'static str = "directory.closed";
    pub const DIRECTORY_OPENED: &'static str = "directory.opened";

    pub const PANEL_RESTORE: &'static str = "panel.restore";
    pub const PANEL_MAXIMISE: &'static str = "panel.maximise";

    pub const SPLIT_HORIZONTAL: &'static str = "split.horizontal";

    pub const TAB_PREVIOUS: &'static str = "tab.previous";
    pub const TAB_NEXT: &'static str = "tab.next";

    pub const SIDEBAR_LEFT: &'static str = "sidebar.left.on";
    pub const SIDEBAR_LEFT_OFF: &'static str = "sidebar.left.off";
    pub const SIDEBAR_RIGHT: &'static str = "sidebar.right.on";
    pub const SIDEBAR_RIGHT_OFF: &'static str = "sidebar.right.off";

    pub const LAYOUT_PANEL: &'static str = "layout.panel.on";
    pub const LAYOUT_PANEL_OFF: &'static str = "layout.panel.off";
    pub const LAYOUT: &'static str = "layout";

    pub const SEARCH: &'static str = "search.icon";
    pub const SEARCH_CLEAR: &'static str = "search.clear";
    pub const SEARCH_FORWARD: &'static str = "search.forward";
    pub const SEARCH_BACKWARD: &'static str = "search.backward";
    pub const SEARCH_CASE_SENSITIVE: &'static str = "search.case_sensitive";
    pub const SEARCH_WHOLE_WORD: &'static str = "search.whole_word";
    pub const SEARCH_REGEX: &'static str = "search.regex";
    pub const SEARCH_REPLACE: &'static str = "search.replace";
    pub const SEARCH_REPLACE_ALL: &'static str = "search.replace_all";

    pub const FILE_TYPE_CODE: &'static str = "file-code";
    pub const FILE_TYPE_MEDIA: &'static str = "file-media";
    pub const FILE_TYPE_BINARY: &'static str = "file-binary";
    pub const FILE_TYPE_ARCHIVE: &'static str = "file-zip";
    pub const FILE_TYPE_SUBMODULE: &'static str = "file-submodule";
    pub const FILE_TYPE_SYMLINK_FILE: &'static str = "file-symlink-file";
    pub const FILE_TYPE_SYMLINK_DIRECTORY: &'static str = "file-symlink-directory";

    pub const DOCUMENT_SYMBOL: &'static str = "document_symbol";

    pub const REFERENCES: &'static str = "references";

    pub const IMPLEMENTATION: &'static str = "implementation";

    pub const SYMBOL_KIND_ARRAY: &'static str = "symbol_kind.array";
    pub const SYMBOL_KIND_BOOLEAN: &'static str = "symbol_kind.boolean";
    pub const SYMBOL_KIND_CLASS: &'static str = "symbol_kind.class";
    pub const SYMBOL_KIND_CONSTANT: &'static str = "symbol_kind.constant";
    pub const SYMBOL_KIND_ENUM_MEMBER: &'static str = "symbol_kind.enum_member";
    pub const SYMBOL_KIND_ENUM: &'static str = "symbol_kind.enum";
    pub const SYMBOL_KIND_EVENT: &'static str = "symbol_kind.event";
    pub const SYMBOL_KIND_FIELD: &'static str = "symbol_kind.field";
    pub const SYMBOL_KIND_FILE: &'static str = "symbol_kind.file";
    pub const SYMBOL_KIND_FUNCTION: &'static str = "symbol_kind.function";
    pub const SYMBOL_KIND_INTERFACE: &'static str = "symbol_kind.interface";
    pub const SYMBOL_KIND_KEY: &'static str = "symbol_kind.key";
    pub const SYMBOL_KIND_METHOD: &'static str = "symbol_kind.method";
    pub const SYMBOL_KIND_NAMESPACE: &'static str = "symbol_kind.namespace";
    pub const SYMBOL_KIND_NUMBER: &'static str = "symbol_kind.number";
    pub const SYMBOL_KIND_OBJECT: &'static str = "symbol_kind.namespace";
    pub const SYMBOL_KIND_OPERATOR: &'static str = "symbol_kind.operator";
    pub const SYMBOL_KIND_PROPERTY: &'static str = "symbol_kind.property";
    pub const SYMBOL_KIND_STRING: &'static str = "symbol_kind.string";
    pub const SYMBOL_KIND_STRUCT: &'static str = "symbol_kind.struct";
    pub const SYMBOL_KIND_TYPE_PARAMETER: &'static str =
        "symbol_kind.type_parameter";
    pub const SYMBOL_KIND_VARIABLE: &'static str = "symbol_kind.variable";

    pub const COMPLETION_ITEM_KIND_CLASS: &'static str =
        "completion_item_kind.class";
    pub const COMPLETION_ITEM_KIND_CONSTANT: &'static str =
        "completion_item_kind.constant";
    pub const COMPLETION_ITEM_KIND_ENUM_MEMBER: &'static str =
        "completion_item_kind.enum_member";
    pub const COMPLETION_ITEM_KIND_ENUM: &'static str = "completion_item_kind.enum";
    pub const COMPLETION_ITEM_KIND_FIELD: &'static str =
        "completion_item_kind.field";
    pub const COMPLETION_ITEM_KIND_FUNCTION: &'static str =
        "completion_item_kind.function";
    pub const COMPLETION_ITEM_KIND_INTERFACE: &'static str =
        "completion_item_kind.interface";
    pub const COMPLETION_ITEM_KIND_KEYWORD: &'static str =
        "completion_item_kind.keyword";
    pub const COMPLETION_ITEM_KIND_METHOD: &'static str =
        "completion_item_kind.method";
    pub const COMPLETION_ITEM_KIND_MODULE: &'static str =
        "completion_item_kind.module";
    pub const COMPLETION_ITEM_KIND_PROPERTY: &'static str =
        "completion_item_kind.property";
    pub const COMPLETION_ITEM_KIND_SNIPPET: &'static str =
        "completion_item_kind.snippet";
    pub const COMPLETION_ITEM_KIND_STRING: &'static str =
        "completion_item_kind.string";
    pub const COMPLETION_ITEM_KIND_STRUCT: &'static str =
        "completion_item_kind.struct";
    pub const COMPLETION_ITEM_KIND_VARIABLE: &'static str =
        "completion_item_kind.variable";
}

use std::{borrow::Cow, collections::HashMap};

/// Semantic icon namespaces as specified in Antares Aurora Design System & ADR-009.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconNamespace {
    App,
    Nav,
    File,
    Editor,
    Git,
    Agent,
    Mcp,
    Status,
    Social,
    Custom,
}

impl IconNamespace {
    pub const fn prefix(&self) -> &'static str {
        match self {
            Self::App => "app",
            Self::Nav => "nav",
            Self::File => "file",
            Self::Editor => "editor",
            Self::Git => "git",
            Self::Agent => "agent",
            Self::Mcp => "mcp",
            Self::Status => "status",
            Self::Social => "social",
            Self::Custom => "custom",
        }
    }
}

/// Unified semantic icon definitions according to PRODUCT-SPEC §5.1.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticIcon {
    // app.*
    AppLogo,
    AppMenu,
    AppTray,
    // nav.*
    NavExplorer,
    NavSearch,
    NavGit,
    NavDebug,
    NavExtensions,
    NavAgents,
    NavMcp,
    NavTests,
    NavSettings,
    NavLayout,
    // file.*
    FileDefault,
    FileDirectoryClosed,
    FileDirectoryOpened,
    FileCode,
    FileMedia,
    FileBinary,
    FileArchive,
    FileSubmodule,
    FileSymlinkFile,
    FileSymlinkDirectory,
    // editor.*
    EditorFormat,
    EditorRefactor,
    EditorRename,
    EditorCodeAction,
    EditorSplitHorizontal,
    EditorSplitVertical,
    EditorFold,
    EditorFoldUp,
    EditorFoldDown,
    EditorUnsaved,
    // git.*
    GitBranch,
    GitCommit,
    GitPull,
    GitPush,
    GitMerge,
    GitRebase,
    GitStash,
    GitConflict,
    GitDiffModified,
    GitDiffAdded,
    GitDiffRemoved,
    GitDiffRenamed,
    // agent.*
    AgentPlanner,
    AgentCoder,
    AgentReviewer,
    AgentTester,
    AgentResearcher,
    AgentSecurity,
    AgentTerminal,
    AgentTool,
    AgentApproval,
    // mcp.*
    McpServer,
    McpConnected,
    McpDisconnected,
    McpResource,
    McpPrompt,
    McpTool,
    McpWarning,
    // status.*
    StatusSuccess,
    StatusInfo,
    StatusWarning,
    StatusError,
    StatusLoading,
    StatusOffline,
    // social.*
    SocialDiscord,
    SocialGithub,
    SocialGitlab,
    // custom.*
    Custom(String),
}

impl SemanticIcon {
    pub fn namespace(&self) -> IconNamespace {
        match self {
            Self::AppLogo | Self::AppMenu | Self::AppTray => IconNamespace::App,
            Self::NavExplorer
            | Self::NavSearch
            | Self::NavGit
            | Self::NavDebug
            | Self::NavExtensions
            | Self::NavAgents
            | Self::NavMcp
            | Self::NavTests
            | Self::NavSettings
            | Self::NavLayout => IconNamespace::Nav,
            Self::FileDefault
            | Self::FileDirectoryClosed
            | Self::FileDirectoryOpened
            | Self::FileCode
            | Self::FileMedia
            | Self::FileBinary
            | Self::FileArchive
            | Self::FileSubmodule
            | Self::FileSymlinkFile
            | Self::FileSymlinkDirectory => IconNamespace::File,
            Self::EditorFormat
            | Self::EditorRefactor
            | Self::EditorRename
            | Self::EditorCodeAction
            | Self::EditorSplitHorizontal
            | Self::EditorSplitVertical
            | Self::EditorFold
            | Self::EditorFoldUp
            | Self::EditorFoldDown
            | Self::EditorUnsaved => IconNamespace::Editor,
            Self::GitBranch
            | Self::GitCommit
            | Self::GitPull
            | Self::GitPush
            | Self::GitMerge
            | Self::GitRebase
            | Self::GitStash
            | Self::GitConflict
            | Self::GitDiffModified
            | Self::GitDiffAdded
            | Self::GitDiffRemoved
            | Self::GitDiffRenamed => IconNamespace::Git,
            Self::AgentPlanner
            | Self::AgentCoder
            | Self::AgentReviewer
            | Self::AgentTester
            | Self::AgentResearcher
            | Self::AgentSecurity
            | Self::AgentTerminal
            | Self::AgentTool
            | Self::AgentApproval => IconNamespace::Agent,
            Self::McpServer
            | Self::McpConnected
            | Self::McpDisconnected
            | Self::McpResource
            | Self::McpPrompt
            | Self::McpTool
            | Self::McpWarning => IconNamespace::Mcp,
            Self::StatusSuccess
            | Self::StatusInfo
            | Self::StatusWarning
            | Self::StatusError
            | Self::StatusLoading
            | Self::StatusOffline => IconNamespace::Status,
            Self::SocialDiscord | Self::SocialGithub | Self::SocialGitlab => {
                IconNamespace::Social
            }
            Self::Custom(_) => IconNamespace::Custom,
        }
    }

    pub fn canonical_key(&self) -> Cow<'static, str> {
        match self {
            Self::AppLogo => Cow::Borrowed("app.logo"),
            Self::AppMenu => Cow::Borrowed("app.menu"),
            Self::AppTray => Cow::Borrowed("app.tray"),
            Self::NavExplorer => Cow::Borrowed("nav.explorer"),
            Self::NavSearch => Cow::Borrowed("nav.search"),
            Self::NavGit => Cow::Borrowed("nav.git"),
            Self::NavDebug => Cow::Borrowed("nav.debug"),
            Self::NavExtensions => Cow::Borrowed("nav.extensions"),
            Self::NavAgents => Cow::Borrowed("nav.agents"),
            Self::NavMcp => Cow::Borrowed("nav.mcp"),
            Self::NavTests => Cow::Borrowed("nav.tests"),
            Self::NavSettings => Cow::Borrowed("nav.settings"),
            Self::NavLayout => Cow::Borrowed("nav.layout"),
            Self::FileDefault => Cow::Borrowed("file.default"),
            Self::FileDirectoryClosed => Cow::Borrowed("file.directory.closed"),
            Self::FileDirectoryOpened => Cow::Borrowed("file.directory.opened"),
            Self::FileCode => Cow::Borrowed("file.code"),
            Self::FileMedia => Cow::Borrowed("file.media"),
            Self::FileBinary => Cow::Borrowed("file.binary"),
            Self::FileArchive => Cow::Borrowed("file.archive"),
            Self::FileSubmodule => Cow::Borrowed("file.submodule"),
            Self::FileSymlinkFile => Cow::Borrowed("file.symlink.file"),
            Self::FileSymlinkDirectory => Cow::Borrowed("file.symlink.directory"),
            Self::EditorFormat => Cow::Borrowed("editor.format"),
            Self::EditorRefactor => Cow::Borrowed("editor.refactor"),
            Self::EditorRename => Cow::Borrowed("editor.rename"),
            Self::EditorCodeAction => Cow::Borrowed("editor.code_action"),
            Self::EditorSplitHorizontal => Cow::Borrowed("editor.split.horizontal"),
            Self::EditorSplitVertical => Cow::Borrowed("editor.split.vertical"),
            Self::EditorFold => Cow::Borrowed("editor.fold"),
            Self::EditorFoldUp => Cow::Borrowed("editor.fold.up"),
            Self::EditorFoldDown => Cow::Borrowed("editor.fold.down"),
            Self::EditorUnsaved => Cow::Borrowed("editor.unsaved"),
            Self::GitBranch => Cow::Borrowed("git.branch"),
            Self::GitCommit => Cow::Borrowed("git.commit"),
            Self::GitPull => Cow::Borrowed("git.pull"),
            Self::GitPush => Cow::Borrowed("git.push"),
            Self::GitMerge => Cow::Borrowed("git.merge"),
            Self::GitRebase => Cow::Borrowed("git.rebase"),
            Self::GitStash => Cow::Borrowed("git.stash"),
            Self::GitConflict => Cow::Borrowed("git.conflict"),
            Self::GitDiffModified => Cow::Borrowed("git.diff.modified"),
            Self::GitDiffAdded => Cow::Borrowed("git.diff.added"),
            Self::GitDiffRemoved => Cow::Borrowed("git.diff.removed"),
            Self::GitDiffRenamed => Cow::Borrowed("git.diff.renamed"),
            Self::AgentPlanner => Cow::Borrowed("agent.planner"),
            Self::AgentCoder => Cow::Borrowed("agent.coder"),
            Self::AgentReviewer => Cow::Borrowed("agent.reviewer"),
            Self::AgentTester => Cow::Borrowed("agent.tester"),
            Self::AgentResearcher => Cow::Borrowed("agent.researcher"),
            Self::AgentSecurity => Cow::Borrowed("agent.security"),
            Self::AgentTerminal => Cow::Borrowed("agent.terminal"),
            Self::AgentTool => Cow::Borrowed("agent.tool"),
            Self::AgentApproval => Cow::Borrowed("agent.approval"),
            Self::McpServer => Cow::Borrowed("mcp.server"),
            Self::McpConnected => Cow::Borrowed("mcp.connected"),
            Self::McpDisconnected => Cow::Borrowed("mcp.disconnected"),
            Self::McpResource => Cow::Borrowed("mcp.resource"),
            Self::McpPrompt => Cow::Borrowed("mcp.prompt"),
            Self::McpTool => Cow::Borrowed("mcp.tool"),
            Self::McpWarning => Cow::Borrowed("mcp.warning"),
            Self::StatusSuccess => Cow::Borrowed("status.success"),
            Self::StatusInfo => Cow::Borrowed("status.info"),
            Self::StatusWarning => Cow::Borrowed("status.warning"),
            Self::StatusError => Cow::Borrowed("status.error"),
            Self::StatusLoading => Cow::Borrowed("status.loading"),
            Self::StatusOffline => Cow::Borrowed("status.offline"),
            Self::SocialDiscord => Cow::Borrowed("social.discord"),
            Self::SocialGithub => Cow::Borrowed("social.github"),
            Self::SocialGitlab => Cow::Borrowed("social.gitlab"),
            Self::Custom(name) => Cow::Owned(format!("custom.{name}")),
        }
    }

    pub fn fallback_glyph(&self) -> &'static str {
        match self {
            Self::AppLogo => "lapce_logo.svg",
            Self::AppMenu => "menu.svg",
            Self::AppTray => "remote.svg",
            Self::NavExplorer => "files.svg",
            Self::NavSearch => "search.svg",
            Self::NavGit => "source-control.svg",
            Self::NavDebug => "debug.svg",
            Self::NavExtensions => "extensions.svg",
            Self::NavAgents => "lightbulb.svg",
            Self::NavMcp => "extensions.svg",
            Self::NavTests => "debug-start.svg",
            Self::NavSettings => "settings-gear.svg",
            Self::NavLayout => "layout.svg",
            Self::FileDefault => "file.svg",
            Self::FileDirectoryClosed => "folder.svg",
            Self::FileDirectoryOpened => "folder-opened.svg",
            Self::FileCode => "file-code.svg",
            Self::FileMedia => "file-media.svg",
            Self::FileBinary => "file-binary.svg",
            Self::FileArchive => "file-zip.svg",
            Self::FileSubmodule => "file-submodule.svg",
            Self::FileSymlinkFile => "file-symlink-file.svg",
            Self::FileSymlinkDirectory => "file-symlink-directory.svg",
            Self::EditorFormat => "file-code.svg",
            Self::EditorRefactor => "lightbulb-autofix.svg",
            Self::EditorRename => "edit.svg",
            Self::EditorCodeAction => "lightbulb.svg",
            Self::EditorSplitHorizontal => "split-horizontal.svg",
            Self::EditorSplitVertical => "split-vertical.svg",
            Self::EditorFold => "fold.svg",
            Self::EditorFoldUp => "fold-up.svg",
            Self::EditorFoldDown => "fold-down.svg",
            Self::EditorUnsaved => "circle-filled.svg",
            Self::GitBranch => "source-control.svg",
            Self::GitCommit => "git-commit.svg",
            Self::GitPull => "repo-pull.svg",
            Self::GitPush => "repo-push.svg",
            Self::GitMerge => "git-merge.svg",
            Self::GitRebase => "git-compare.svg",
            Self::GitStash => "history.svg",
            Self::GitConflict => "warning.svg",
            Self::GitDiffModified => "diff-modified.svg",
            Self::GitDiffAdded => "diff-added.svg",
            Self::GitDiffRemoved => "diff-removed.svg",
            Self::GitDiffRenamed => "diff-renamed.svg",
            Self::AgentPlanner => "lightbulb.svg",
            Self::AgentCoder => "file-code.svg",
            Self::AgentReviewer => "search.svg",
            Self::AgentTester => "debug-start.svg",
            Self::AgentResearcher => "search.svg",
            Self::AgentSecurity => "warning.svg",
            Self::AgentTerminal => "terminal.svg",
            Self::AgentTool => "settings-gear.svg",
            Self::AgentApproval => "circle-filled.svg",
            Self::McpServer => "extensions.svg",
            Self::McpConnected => "circle-filled.svg",
            Self::McpDisconnected => "error.svg",
            Self::McpResource => "file.svg",
            Self::McpPrompt => "lightbulb.svg",
            Self::McpTool => "settings-gear.svg",
            Self::McpWarning => "warning.svg",
            Self::StatusSuccess => "circle-filled.svg",
            Self::StatusInfo => "info.svg",
            Self::StatusWarning => "warning.svg",
            Self::StatusError => "error.svg",
            Self::StatusLoading => "refresh.svg",
            Self::StatusOffline => "close.svg",
            Self::SocialDiscord => "remote.svg",
            Self::SocialGithub => "source-control.svg",
            Self::SocialGitlab => "source-control.svg",
            Self::Custom(_) => "file.svg",
        }
    }
}

/// Central IconRegistry supporting semantic keys, fallback glyphs, custom packs, and SVG sanitization.
#[derive(Clone, Debug, Default)]
pub struct IconRegistry {
    custom_mappings: HashMap<String, String>,
}

impl IconRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, key: impl Into<String>, asset_path: impl Into<String>) {
        self.custom_mappings.insert(key.into(), asset_path.into());
    }

    pub fn resolve_semantic(&self, icon: &SemanticIcon) -> &str {
        let key = icon.canonical_key();
        if let Some(path) = self.custom_mappings.get(key.as_ref()) {
            path.as_str()
        } else {
            icon.fallback_glyph()
        }
    }

    pub fn resolve_key<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(path) = self.custom_mappings.get(key) {
            path.as_str()
        } else {
            Self::default_glyph_for_key(key).unwrap_or("file.svg")
        }
    }

    pub fn default_glyph_for_key(key: &str) -> Option<&'static str> {
        match key {
            "antares.agent" | "nav.agents" | "agent.planner" => Some("lightbulb.svg"),
            "antares.mcp" | "nav.mcp" | "mcp.server" => Some("extensions.svg"),
            "agent.coder" => Some("file-code.svg"),
            "agent.reviewer" | "agent.researcher" => Some("search.svg"),
            "agent.tester" | "nav.tests" => Some("debug-start.svg"),
            "agent.terminal" => Some("terminal.svg"),
            "agent.tool" | "mcp.tool" => Some("settings-gear.svg"),
            "status.success" | "mcp.connected" => Some("circle-filled.svg"),
            "status.warning" | "mcp.warning" => Some("warning.svg"),
            "status.error" | "mcp.disconnected" => Some("error.svg"),
            "status.info" => Some("info.svg"),
            "status.loading" => Some("refresh.svg"),
            "status.offline" => Some("close.svg"),
            "git.commit" => Some("git-commit.svg"),
            "git.merge" => Some("git-merge.svg"),
            "git.pull" => Some("repo-pull.svg"),
            "git.push" => Some("repo-push.svg"),
            _ => None,
        }
    }

    /// Sanitize SVG according to ADR-009 / PRODUCT-SPEC §5.3.
    /// Rejects executable script tags, event handlers, foreignObject and remote hrefs.
    pub fn sanitize_svg(content: &str) -> Result<String, &'static str> {
        let lower = content.to_lowercase();
        if lower.contains("<script") || lower.contains("</script") {
            return Err("SVG contains forbidden <script> element");
        }
        if lower.contains("<foreignobject") || lower.contains("</foreignobject") {
            return Err("SVG contains forbidden <foreignObject> element");
        }
        if lower.contains("javascript:") {
            return Err("SVG contains forbidden javascript: URI");
        }
        if lower.contains("onload=")
            || lower.contains("onerror=")
            || lower.contains("onclick=")
            || lower.contains("onmouseover=")
        {
            return Err("SVG contains forbidden inline event handler");
        }
        if lower.contains("href=\"http://")
            || lower.contains("href=\"https://")
            || lower.contains("href='http://")
            || lower.contains("href='https://")
        {
            return Err("SVG contains forbidden external href reference");
        }
        Ok(content.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_icon_namespaces() {
        assert_eq!(SemanticIcon::AppLogo.namespace(), IconNamespace::App);
        assert_eq!(SemanticIcon::NavAgents.namespace(), IconNamespace::Nav);
        assert_eq!(SemanticIcon::NavMcp.namespace(), IconNamespace::Nav);
        assert_eq!(SemanticIcon::NavLayout.namespace(), IconNamespace::Nav);
        assert_eq!(SemanticIcon::FileDefault.namespace(), IconNamespace::File);
        assert_eq!(SemanticIcon::EditorSplitHorizontal.namespace(), IconNamespace::Editor);
        assert_eq!(SemanticIcon::GitBranch.namespace(), IconNamespace::Git);
        assert_eq!(SemanticIcon::AgentPlanner.namespace(), IconNamespace::Agent);
        assert_eq!(SemanticIcon::McpServer.namespace(), IconNamespace::Mcp);
        assert_eq!(SemanticIcon::StatusSuccess.namespace(), IconNamespace::Status);
        assert_eq!(SemanticIcon::SocialDiscord.namespace(), IconNamespace::Social);
        assert_eq!(SemanticIcon::Custom("my-icon".into()).namespace(), IconNamespace::Custom);
    }

    #[test]
    fn test_semantic_icon_canonical_keys() {
        assert_eq!(SemanticIcon::AppLogo.canonical_key(), "app.logo");
        assert_eq!(SemanticIcon::NavAgents.canonical_key(), "nav.agents");
        assert_eq!(SemanticIcon::NavMcp.canonical_key(), "nav.mcp");
        assert_eq!(SemanticIcon::NavLayout.canonical_key(), "nav.layout");
        assert_eq!(LapceIcons::LAYOUT, "layout");
        assert_eq!(SemanticIcon::AgentPlanner.canonical_key(), "agent.planner");
        assert_eq!(SemanticIcon::AgentCoder.canonical_key(), "agent.coder");
        assert_eq!(SemanticIcon::StatusSuccess.canonical_key(), "status.success");
        assert_eq!(SemanticIcon::Custom("custom_tag".into()).canonical_key(), "custom.custom_tag");
    }

    #[test]
    fn test_icon_registry_default_fallbacks() {
        let registry = IconRegistry::new();
        assert_eq!(registry.resolve_semantic(&SemanticIcon::NavAgents), "lightbulb.svg");
        assert_eq!(registry.resolve_semantic(&SemanticIcon::NavMcp), "extensions.svg");
        assert_eq!(registry.resolve_semantic(&SemanticIcon::StatusSuccess), "circle-filled.svg");
        assert_eq!(registry.resolve_semantic(&SemanticIcon::StatusError), "error.svg");
        assert_eq!(registry.resolve_key("antares.agent"), "lightbulb.svg");
        assert_eq!(registry.resolve_key("unknown.key"), "file.svg");
    }

    #[test]
    fn test_icon_registry_custom_override() {
        let mut registry = IconRegistry::new();
        registry.register("nav.agents", "custom_antares_agent.svg");
        assert_eq!(registry.resolve_semantic(&SemanticIcon::NavAgents), "custom_antares_agent.svg");
        assert_eq!(registry.resolve_key("nav.agents"), "custom_antares_agent.svg");
    }

    #[test]
    fn test_icon_registry_svg_sanitizer() {
        let safe_svg = r#"<svg viewBox="0 0 16 16"><path d="M1 1h14v14H1z"/></svg>"#;
        assert!(IconRegistry::sanitize_svg(safe_svg).is_ok());

        let script_svg = r#"<svg><script>alert(1)</script></svg>"#;
        assert!(IconRegistry::sanitize_svg(script_svg).is_err());

        let onclick_svg = r#"<svg><path onclick="alert(1)"/></svg>"#;
        assert!(IconRegistry::sanitize_svg(onclick_svg).is_err());

        let foreign_svg = r#"<svg><foreignObject><div>test</div></foreignObject></svg>"#;
        assert!(IconRegistry::sanitize_svg(foreign_svg).is_err());

        let remote_href_svg = r#"<svg><a href="https://malicious.site/payload"></a></svg>"#;
        assert!(IconRegistry::sanitize_svg(remote_href_svg).is_err());
    }
}
