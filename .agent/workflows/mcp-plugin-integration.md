# Workflow: Integrate an MCP server, plugin or skill

1. Read `docs/EXTENSIBILITY.md`, `docs/SECURITY.md` and `.agent/rules/13-mcp-plugin-skills.md`.
2. Classify integration type: MCP server, Antares extension, Agent Skill or provider adapter. Do not combine contracts.
3. Pin protocol/SDK/version and inspect official docs/source for that exact version.
4. Define manifest, capabilities, scopes, trust level, transport, lifecycle and uninstall/rollback.
5. Implement discovery and health checks with timeouts, bounded output and cancellation.
6. Present requested permissions and tool schemas before enabling. Default deny.
7. Sanitize untrusted content; redact secrets; do not auto-open URLs or execute commands.
8. Add contract tests for malformed payloads, unsupported capabilities, disconnect, timeout, restart and revocation.
9. Verify UI states and audit events; update compatibility matrix.
