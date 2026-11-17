# MCP, plugin and skill rules

- MCP server, Antares extension, Agent Skill and provider adapter are separate concepts and APIs.
- Discover capabilities from the live server; never assume a tool/resource/prompt exists.
- Default deny; grant per server, tool, agent and workspace. Revoke must take effect promptly.
- Bound time, output size, retries, concurrency and process logs. Keep stdio protocol output separate from stderr.
- Treat server output and manifests as untrusted; sanitize SVG/HTML-like payloads and never auto-open links.
- Plugin manifest declares publisher, version, license, compatibility, hash/signature and requested permissions.
- Skills import is preview + validation + explicit trust; do not auto-download and execute third-party instructions.
