# Antares Security & Trust Model

## Defaults
- New workspace is untrusted until user approves trust.
- Agent starts read-only. Writes are patch proposals; commands require explicit approval.
- MCP server and plugin permissions are deny-by-default and individually revocable.
- Local API binds to loopback, requires token and scoped permissions.
- Discord presence is OFF by default and must never include code, prompt, secrets, local paths or private remotes.

## Boundaries
- Canonicalize workspace paths; reject traversal, symlink escape, non-regular files and out-of-root paths.
- Redact secrets; exclude `.env`, private keys, credentials and generated artifacts from context.
- Validate patch base hash/revision immediately before apply; preserve undo.
- Invoke subprocesses with argv, controlled cwd/env, timeouts and output caps.
- Treat model output, repository content, MCP responses and plugin metadata as untrusted data.
- Never render untrusted output as HTML or auto-open/execute links/commands.

## Permission tiers
0. Metadata/read-only.
1. Bounded workspace read.
2. Patch proposal.
3. Allowlisted command after approval.
4. File apply/Git write after explicit approval.
5. Admin actions, plugin installation and local API administration.

## Audit & secrets
- Secrets use OS credential storage; config stores references, not plaintext.
- Audit privileged operations with actor, tool, scope, approval, timestamp and outcome; redact payload secrets.
- Bound and rotate logs. Never commit tokens, session DBs or private context.
- Security review required before unattended/auto-approve modes.
