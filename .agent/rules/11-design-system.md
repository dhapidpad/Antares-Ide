# Antares Aurora design-system rules

- Read `docs/DESIGN-SYSTEM.md` before every UI change.
- Use semantic tokens, shared components and `IconRegistry`; no hardcoded hex colors, arbitrary spacing or raw glyphs in feature views.
- Preserve editor-first hierarchy and existing native Floem architecture; do not introduce a web UI stack without ADR.
- Implement all relevant states: default, hover, pressed, focus, selected, disabled, loading, empty, success and error.
- Every icon-only action needs tooltip and accessible label. Keyboard navigation and visible focus are mandatory.
- Dark and Light themes, Compact/Comfortable/Spacious density, reduced motion and narrow window behavior must be checked.
- Animation is purposeful, short and cancellable; no continuous decorative loops or expensive blur.
- Do not steal editor focus or rebuild the entire view on every streamed event.
- UI PR includes before/after screenshots and a manual smoke checklist when desktop is available.
