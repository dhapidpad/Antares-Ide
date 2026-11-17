# Antares Git Integration

## Required UX
Clone, open/init repo, fetch, pull, push, sync, branch create/switch/rename/delete, stage/unstage file or hunk, commit/amend, stash, cherry-pick, revert, merge/rebase, abort, conflict resolution, log/graph, blame, file history, worktrees, tags/remotes and open remote URL. Provider integrations (GitHub/GitLab/Gitea/Forgejo/Bitbucket) remain optional adapters.

## Safety
- Show remote, branch, outgoing commits and changed files before push.
- Force push is blocked by default and requires explicit typed/clear confirmation.
- Destructive discard/clean/delete operations show exact scope and preview.
- Credentials go through OS credential helper/keychain; never embed tokens in URLs/logs.
- Clone never runs post-clone scripts/tasks automatically.
- Git operations are cancellable where backend supports it; report partial results honestly.

## Command palette
Every common operation has a stable command ID, category, description, icon, shortcut, risk level and permission requirement. Commands from plugins/MCP are namespaced and display their source/trust.
