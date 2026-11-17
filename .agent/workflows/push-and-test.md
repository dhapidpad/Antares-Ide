# Workflow: push to GitHub and test with Actions

1. Work only on `agy/<phase>-<topic>`; `origin` must be the personal Antares fork (never `lapce/lapce`). One-time: `gh auth login` and `gh auth setup-git`; never paste a token.
2. Dry run first: `bash scripts/antares-push-and-watch.sh --dry-run` (scaffold check, whitespace, secret scan, local gate; nothing is pushed).
3. Review `git status` and the staged diff. Write a Conventional Commit (`type(scope): summary`, <=72 chars).
4. Push and test: `bash scripts/antares-push-and-watch.sh --commit "type(scope): summary"` (or `make -f Makefile.antares antares-push MSG="..."`). It scans for secrets, runs the gate, pushes without force, finds the run for the exact SHA, and watches it.
5. Read the last line `ANTARES_CI_RESULT sha=... run=... conclusion=... url=...`. Only `conclusion=success` for the pushed SHA counts as green.
6. Exit 1 (red): open the saved log in `target/antares-ci/`, find the FIRST causal error, follow `.agent/workflows/fix-ci.md`, fix the root cause, run again. Exit 3: Actions disabled or no run; tell the user. Exit 4: five failed attempts; stop, write the blocker to `docs/STATUS.md`, ask the user.
7. On green, record SHA + run URL in `docs/STATUS.md` (and `docs/reference/PATCHES.md` if upstream files changed). Commit docs separately and push the same way.
8. Already pushed manually? Use `--watch-only`. Never force-push, never merge to `main` or open a PR to Lapce automatically.
