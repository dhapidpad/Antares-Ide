# Documentation and cleanup rules

- Canonical docs are indexed in `docs/README.md`; roadmap only in `docs/ROADMAP.md`; status only in `docs/STATUS.md`.
- Do not create another master plan, duplicate roadmap, or competing prompt. Update canonical docs instead.
- Before deleting docs, search inbound links and move only still-relevant operational/legal/upstream content to canonical locations.
- Never delete LICENSE, NOTICE, copyright, third-party license, upstream provenance, migration compatibility or test fixture docs as generic cleanup.
- Dead-code removal requires evidence across cfg targets, macros, dynamic registration, build scripts, serialization and downstream APIs.
- Remove small batches, run tests, inspect diff, and record measured performance/size change.
