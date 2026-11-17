# Patches to Lapce upstream

The Antares IDE integration keeps modifications to upstream Lapce files isolated, minimal, and non-destructive.

| File | Change | Why | Re-apply hint | Status |
|---|---|---|---|---|
| root `Cargo.toml` | Add `antares-agent` to `[workspace].members` | Compile Antares sidecar boundary in same workspace | `scripts/bootstrap.sh` performs an idempotent members-array edit | Applied |
| root `Cargo.lock` | Record the new workspace package after `cargo check -p antares-agent` | Keep `--locked` CI reproducible | Let Cargo update it; never hand-edit lock data | Applied |
| root `Makefile` | Add `-include Makefile.antares` | Expose Antares local gate without replacing upstream targets | `scripts/bootstrap.sh` appends the include once | Applied |
| root `.gitignore` | Append Antares secret/local build ignores | Avoid committing credentials and generated output | `scripts/bootstrap.sh` appends missing lines only | Applied |
| `lapce-app/Cargo.toml` | Add `antares-agent = { path = "../antares-agent" }` | Expose agent domain state/reducer to Lapce UI | Add path dependency under `[dependencies]` | Applied |
| `lapce-app/src/lib.rs` | Add `pub mod antares;` | Expose Antares UI module to Lapce application | Add module declaration | Applied |
| `lapce-app/src/panel/kind.rs` | Add `PanelKind::Antares` | Register Antares AI panel type with icon and default position | Add enum variant and match arms | Applied |
| `lapce-app/src/panel/data.rs` | Add `PanelKind::Antares` to `default_panel_order` | Show Antares panel in `RightTop` position by default | Insert into `RightTop` panel vector | Applied |
| `lapce-app/src/panel/view.rs` | Add `PanelKind::Antares` view dispatch and tooltip | Render `antares_panel` when selected in panel picker | Add match arms in `panel_view` and `panel_picker` | Applied |
| `lapce-app/src/window_tab.rs` | Add `antares_data` field and `PanelKind::Antares` focus check | Wire reactive panel data into window tab lifecycle | Add field to `WindowTabData` and constructor | Applied |
| `lapce-app/src/app.rs` | Update window title and menu labels to Antares IDE | Synchronize user-facing application identity | Replace "Lapce" with "Antares IDE" in title and menu | Applied |
| `lapce-app/src/title.rs` | Update About menu item to "About Antares IDE" | Present cohesive Antares branding in window title bar | Update string in menu item | Applied |
| `lapce-app/src/command.rs` | Update ShowAbout command description | Display "About Antares IDE" in command palette | Update strum message | Applied |
| `lapce-app/src/about.rs` | Update About popup branding and GitHub link | Point to Antares repo and show Antares IDE title | Update label and const GITHUB | Applied |
| `extra/linux/dev.lapce.lapce.desktop` | Update desktop metadata name to Antares IDE | Linux desktop integration branding | Update Name and GenericName | Applied |
| `lapce-core/build.rs` | Update `NAME` constant from `Lapce-{branch}` to `Antares-{branch}` | Product identity and directory isolation for Antares IDE | Update format string in meta generator | Applied |
| `lapce-core/src/directory.rs` | Update project dirs to Antares with legacy Lapce migration | Automatic non-destructive migration from Lapce config/data | Add `migrate_directory_content` and fallback check | Applied |
| `defaults/dark-theme.toml` | Update theme name to "Antares Dark" & add Antares Aurora tokens | Antares Aurora dark design tokens per DESIGN-SYSTEM §3 | Update name and semantic base/ui color keys | Applied |
| `defaults/light-theme.toml` | Update theme name to "Antares Light" & add Antares Aurora tokens | Antares Aurora light design tokens per DESIGN-SYSTEM §3 | Update name and semantic base/ui color keys | Applied |
| `defaults/icon-theme.toml` | Update theme name to "Antares Codicons" & add agent/mcp icons | Antares Codicon icon pack with agent/mcp semantics | Update name and add semantic UI keys | Applied |
| `defaults/settings.toml` | Set default color-theme & icon-theme to Antares | Default out-of-the-box experience for Antares IDE | Update default theme setting strings | Applied |
| `lapce-app/src/config.rs` | Theme/icon alias mapping with safe `ui_svg` fallback | Backward compatibility for legacy Lapce configs & zero-panic SVG | Map legacy names to Antares in resolve_theme | Applied |
| `lapce-app/src/config/core.rs` | Update theme & icon config field descriptions | Product identity in settings documentation | Update field_names desc attributes | Applied |
| `lapce-app/src/config/icon.rs` | Add `IconNamespace`, `SemanticIcon`, `IconRegistry` | Unified semantic icon system & SVG sanitizer per ADR-009 | Add enums, registry struct, and unit tests | Applied |
| `lapce-app/src/config/svg.rs` | Safe fallback to `file.svg` on missing glyphs | Prevent runtime panics if glyph not found in codicons | Add `or_else` fallback to CODICONS_ICONS_DIR | Applied |
| `lapce-app/src/panel/kind.rs` | Use `LapceIcons::ANTARES_AGENT` for `PanelKind::Antares` | Semantic icon key for Antares Agent panel | Update icon mapping in `PanelKind::svg_name` | Applied |
