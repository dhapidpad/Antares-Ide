# Workflow: Design system audit and UI polish

1. Read `docs/DESIGN-SYSTEM.md`, `docs/PRODUCT-SPEC.md`, `docs/STATUS.md` and `.agent/rules/11-design-system.md`.
2. Capture current screenshots at supported window sizes; identify visual inconsistencies with exact file/component paths.
3. Inventory theme keys, colors, typography, spacing, radius, icon packs and motion usage; do not start with arbitrary restyling.
4. Map existing controls to semantic tokens and shared components; identify missing states and accessibility gaps.
5. Propose a bounded change list and acceptance checklist; preserve editor-first layout and Floem architecture.
6. Implement tokens/components before migrating feature panels; do not mass-replace colors or icons.
7. Test Dark/Light, density modes, keyboard-only, focus, reduced motion, narrow layout, loading/error/empty states.
8. Verify no UI-thread blocking, stream-induced rebuilds or focus theft.
9. Run targeted checks, `git diff --check`; attach screenshots and report unresolved visual debt.
