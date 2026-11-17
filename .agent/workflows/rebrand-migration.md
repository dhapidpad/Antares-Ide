# Workflow: Rebrand Lapce identity to Antares safely

1. Read `docs/BRANDING-MIGRATION.md` and `.agent/rules/02-upstream.md`.
2. Run case-insensitive inventory of Lapce references; classify each hit before editing.
3. Preserve LICENSE, NOTICE, copyright, upstream source references and compatibility readers.
4. Separate user-facing strings from crate/module names, executable identity, bundle IDs and data paths.
5. Design versioned, idempotent, backup-first migration with dry-run and rollback.
6. Update packaging per OS and test clean install plus upgrade from old data directories.
7. Check URLs, CLI, protocol handlers, update feed, icons, tests, snapshots and docs links.
8. Do not delete old data or rename internal crates without ADR and compatibility evidence.
