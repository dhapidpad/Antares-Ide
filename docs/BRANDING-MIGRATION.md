# Lapce → Antares Branding & Migration

## Product-facing identity
All user-facing title, menus, About, onboarding, installer, app metadata, CLI help, notifications and docs should say Antares. Do not bulk replace identifiers.

## Classify each occurrence
- `PRODUCT_SURFACE`: rename to Antares.
- `IDENTITY`: executable, bundle ID, data dirs, protocol identity; migrate with versioned code.
- `INTERNAL_UPSTREAM`: keep when changing breaks compatibility or causes unnecessary fork divergence.
- `UPSTREAM_PROVENANCE`: preserve license, copyright, attribution and source history.
- `COMPAT_LEGACY`: read old Lapce data during migration; do not expose old name in UI.
- `FALSE_POSITIVE`: leave unchanged.

## Migration requirements
- Versioned, idempotent, dry-run capable, backup-first and rollback-aware.
- Import settings, keymaps, themes, snippets, extensions, workspace state and recent projects.
- Never delete old data automatically.
- Preserve workspace files and Git metadata.
- Redact secrets from logs; use OS credential storage.
- Test clean install, upgrade, interrupted migration, duplicate data, Unicode paths and each supported OS.

## Search inventory
`rg -n -i --hidden --glob '!target/**' --glob '!.git/**' 'lapce' .` is inventory only. Classify every hit before editing. Keep LICENSE, NOTICE, copyright headers and upstream references.
