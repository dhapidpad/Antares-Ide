# Workflow: Maintain canonical documentation

1. Read `docs/README.md` and `.agent/rules/15-docs-and-cleanup.md`.
2. Identify the canonical document for the change; update it instead of creating a duplicate master file.
3. Keep product requirements, architecture, roadmap and status separate.
4. Update links, root agent pointers and relevant workflows/rules together.
5. Move still-useful legacy material to `guides/`, `adr/`, `audit/` or `reference/`; delete only verified duplicates.
6. Preserve legal, license, provenance, changelog and test fixture files.
7. Run link/reference search and `git diff --check`.
