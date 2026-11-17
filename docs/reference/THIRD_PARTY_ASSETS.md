# Antares Third-Party Assets & Icon Registry Ledger

> Canonical asset and license ledger for all vector, icon, and font assets bundled with or supported by Antares IDE.
> Mandated by **ADR-009: Semantic IconRegistry, Licensing and Custom Icon Pack Specification** and **PRODUCT-SPEC §5.4**.

---

## 1. Bundled Icon Sets

| Asset Pack | Author / Maintainer | Upstream Source | License | Location in Tree | Category |
|---|---|---|---|---|---|
| **Codicons** | Microsoft Corporation | [microsoft/vscode-codicons](https://github.com/microsoft/vscode-codicons) | Creative Commons Attribution 4.0 International (CC-BY 4.0) | `icons/codicons/` | Primary Editor, Status & Nav Icons |
| **Lucide UI** | Lucide Contributors | [lucide-icons/lucide](https://github.com/lucide-icons/lucide) | ISC License | Runtime extension / specification | Workbench & Tooling Glyphs |
| **Simple Icons** | Simple Icons Collaborators | [simple-icons/simple-icons](https://github.com/simple-icons/simple-icons) | CC0 1.0 Universal (Public Domain) | Brand / Provider references | Provider & Social Marks |

---

## 2. Icon Registry Namespace Architecture

All icons in Antares must be referenced via `SemanticIcon` enum keys or semantic identifier strings (`namespace.action`). Components must not resolve raw glyph names directly without fallback support.

```
app.*       -> Application brand mark, titlebar, system tray
nav.*       -> Primary navigation icons (Explorer, Search, Git, Debug, Extensions, Agents, MCP, Settings)
file.*      -> File types and directory icons (File, Folder, Binary, Code, Media, Archive, Symlink)
editor.*    -> Editor controls (Split, Fold, Unsaved, Format, Refactor, Rename, CodeAction)
git.*       -> Source control operations (Branch, Commit, Pull, Push, Merge, Rebase, Diff)
agent.*     -> AI Agent roles and actions (Planner, Coder, Reviewer, Tester, Researcher, Tool, Approval)
mcp.*       -> Model Context Protocol state (Server, Connected, Disconnected, Resource, Tool, Prompt)
status.*    -> Workbench status indicators (Success, Info, Warning, Error, Loading, Offline)
social.*    -> Integrations (Discord, GitHub, GitLab)
custom.*    -> User-installed custom icon packs
```

---

## 3. Asset Safety & Sanitization Policy

Per **ADR-009** and **PRODUCT-SPEC §5.3**, imported and custom SVG assets must pass strict verification before rendering:
1. **No Executable Scripts**: Any `<script>` or `</script>` tags are strictly rejected.
2. **No Embedded Objects**: `<foreignObject>` tags are rejected to avoid browser rendering vulnerabilities.
3. **No Inline Event Handlers**: Attributes like `onload`, `onerror`, `onclick`, and `onmouseover` are stripped or rejected.
4. **No Remote URIs**: References with `javascript:` or remote network `href="http..."` are disallowed.
5. **Deterministic Fallback**: If an icon key is missing or an asset fails validation, `IconRegistry` falls back to standard semantic fallbacks (e.g. `file.svg` or `circle-filled.svg`), ensuring zero application crashes or blank tofu boxes.
