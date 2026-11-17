# Antares Architecture

## Product boundary
Antares là lớp sản phẩm trên nền editor Lapce. UI desktop hiện dùng Rust/Floem theo source tree; không giả định có webview/React runtime trong core.

## Ownership
- Upstream editor: buffer, undo/redo, syntax, LSP, terminal, proxy, workspace, panel infrastructure, core commands.
- Antares UI: `lapce-app/src/antares/`; Floem view/adapter chỉ nằm ở lớp UI.
- Antares agent domain: `antares-agent`; protocol/session/context/policy/patch boundaries tách module hoặc crate khi có nhu cầu thực tế.
- Runtime: external agent sidecar qua adapter ACP; không vendor nguyên repo runtime.
- MCP: phân biệt Antares-hosted tools và generic MCP client/server integration; không gọi hai thứ là một.

## Data flow
`Editor state → bounded context snapshot → provider/agent runtime → typed events → reducer/state → Floem view`.

Patch flow: `agent proposal → path/hash validation → diff preview → explicit user approval → editor buffer apply → undo-capable update`.

Command flow: `agent request → policy/risk check → approval → allowlisted execution → bounded output`.

## Process boundaries
- UI process không chạy blocking process/network/file scan.
- Agent/MCP helpers có supervisor, timeout, cancellation, bounded stdout/stderr và shutdown.
- Local API loopback-only mặc định, token auth và scopes.
- Discord RPC chỉ start khi opt-in.

## Compatibility
- ACP, MCP, provider API và plugin manifests đều versioned/adapter-backed.
- Không dùng protocol field khi chưa xác minh với pinned docs/source.
- Lapce upstream names được giữ trong internal API khi cần để giảm fork divergence; branding user-facing phải là Antares.

## Required ADRs
Xem `adr/DECISIONS.md`; thay đổi process boundary, provider abstraction, workflow schema, Git backend, identity migration hoặc plugin sandbox phải có ADR.
