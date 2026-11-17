# Git integration safety rules

- Show repository, branch, remote and exact affected files/commits before write operations.
- Push, commit, branch deletion, clean, discard, reset, rebase and force-push must use explicit confirmation according to risk.
- Force-push is disabled by default; never force-push main.
- Credentials use OS credential helpers/keychain; never store tokens in remote URLs, logs or settings plaintext.
- Clone must not auto-run repository scripts/tasks.
- Review full staged diff before commit; never include secrets, generated outputs or local state.
