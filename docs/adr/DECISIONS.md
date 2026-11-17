# Antares Architecture Decision Records (ADRs)

> Single Source of Truth for foundational architectural, process, security, and integration decisions in Antares IDE.
> Aligned with `docs/PRODUCT-SPEC.md` §23 and `docs/ROADMAP.md` Phase 2 Gate.

---

## Master Decision Ledger

| ID | Title | Status | Date | Scope |
|---|---|---|---|---|
| **ADR-001** | Crate and Folder Naming: Internal Upstream vs Antares Facade | **APPROVED** | 2026-11-17 | Architecture / Rebrand |
| **ADR-002** | Agent Runtime Process Boundary and ACP stdio Transport | **APPROVED** | 2026-11-17 | Runtime / Security |
| **ADR-003** | Multi-Agent Orchestration Engine, Workflow DAG and State Persistence | **APPROVED** | 2026-11-17 | Multi-Agent / Core |
| **ADR-004** | Provider Abstraction Trait and Local/Hosted Model Discovery | **APPROVED** | 2026-11-17 | AI Providers |
| **ADR-005** | Generic MCP Client Architecture, Transports and Server Trust Model | **APPROVED** | 2026-11-17 | MCP / Extensions |
| **ADR-006** | Plugin and Extension API, Manifest Format and Sandbox Strategy | **APPROVED** | 2026-11-17 | Plugins / WASI |
| **ADR-007** | Local API v1 Authentication, Loopback Binding and Scoped SSE | **APPROVED** | 2026-11-17 | Integrations / API |
| **ADR-008** | Hybrid Git Architecture, Credential Helpers and Worktree Isolation | **APPROVED** | 2026-11-17 | Git / VCS |
| **ADR-009** | Semantic IconRegistry, Licensing and Custom Icon Pack Specification | **APPROVED** | 2026-11-17 | UI / Design System |
| **ADR-010** | Discord RPC IPC Architecture, Opt-in Lifecycle and Privacy Isolation | **APPROVED** | 2026-11-17 | Integrations |
| **ADR-011** | Settings and Data Directory Migration from Lapce with Rollback | **APPROVED** | 2026-11-17 | Migration / Identity |
| **ADR-012** | Performance Budgets, Process Budgets and CI Regression Gates | **APPROVED** | 2026-11-17 | Performance / CI |

---

### ADR-001: Crate and Folder Naming — Internal Upstream vs Antares Facade
- **Status**: Approved
- **Context**: Forking `lapce/lapce` presents a maintenance challenge: bulk-renaming every internal struct, module, and crate from `lapce` to `antares` creates massive merge conflicts when synchronizing upstream updates.
- **Decision**:
  1. Retain upstream crate names (`lapce-app`, `lapce-core`, `lapce-proxy`, `lapce-rpc`) and internal module structures to minimize fork divergence.
  2. Implement an Antares facade layer: all user-facing strings (window title, About dialog, menus, CLI binaries, application metadata) present strictly as **Antares IDE**.
  3. All new AI, agent, orchestration, context, and diff logic reside in dedicated `antares-*` crates (e.g. `antares-agent`) or isolated submodules (`lapce-app/src/antares/`).
- **Consequences**: Easy tracking of upstream commits; zero name collision; clear ownership boundary.

---

### ADR-002: Agent Runtime Process Boundary and ACP stdio Transport
- **Status**: Approved
- **Context**: The AI agent runtime (Octomind) must execute reliably without crashing the editor UI, freezing the render thread, or leaking unparsed protocol data.
- **Decision**:
  1. Spawn `octomind acp` as an external child process managed by `antares-agent::ProcessSupervisor`.
  2. Reserve stdout exclusively for JSON-RPC 2.0 Agent Client Protocol (ACP) framing.
  3. Drain stderr independently to a bounded (500 lines) diagnostic ring buffer.
  4. Use dedicated non-blocking worker threads for stdin write and stdout read operations; UI thread communicates via bounded lock-free channels.
  5. Pin Octomind release (0.72.0) with SHA-256 verification.
- **Consequences**: Subprocess crashes do not affect editor stability; responsive UI; strictly typed protocol boundary.

---

### ADR-003: Multi-Agent Orchestration Engine, Workflow DAG and State Persistence
- **Status**: Approved
- **Context**: Users require multi-agent collaborative workflows (e.g. Planner → Coder → Reviewer) executing in sequence or parallel without race conditions or corrupted workspace state.
- **Decision**:
  1. Represent multi-agent workflows as Directed Acyclic Graphs (DAGs) supporting sequential, fan-out/fan-in parallel, and supervisor-worker patterns.
  2. Execute workflow state transitions using a pure deterministic reducer (`antares-agent::PanelState` / `WorkflowState`).
  3. Enforce file conflict ownership: parallel agents must operate on isolated Git worktrees.
  4. Persist workflow execution traces and checkpoints via Write-Ahead Logging (WAL) for pause, resume, and crash recovery.
- **Consequences**: Deterministic multi-agent execution; reproducible traces; no concurrent file write collisions.

---

### ADR-004: Provider Abstraction Trait and Local/Hosted Model Discovery
- **Status**: Approved
- **Context**: Antares must support diverse LLM providers (Google Gemini, Anthropic, OpenAI, local Ollama, vLLM, custom endpoints) without coupling UI to specific SDKs.
- **Decision**:
  1. Define a unified `Provider` trait in `antares-agent` abstracting chat completion, streaming, token tracking, and capability discovery.
  2. Implement provider adapters for Hosted (Gemini 3.8 Flash, Claude, OpenAI), Local (Ollama, Llama.cpp), and Custom OpenAI-compatible endpoints.
  3. Store API credentials securely in OS keyrings (never plaintext in git or project files).
- **Consequences**: Easy addition of new model providers; offline support via local models; robust secret safety.

---

### ADR-005: Generic MCP Client Architecture, Transports and Server Trust Model
- **Status**: Approved
- **Context**: Model Context Protocol (MCP) servers extend agent capabilities but introduce security risks if allowed silent access to files or network.
- **Decision**:
  1. Strictly distinguish Antares-hosted internal workspace tools (`antares-workspace`: read-only files, patch proposal, command proposal) from external third-party MCP servers.
  2. Support stdio and SSE transports for external MCP servers.
  3. Enforce explicit user approval before executing any MCP tool that writes files, runs commands, or makes external network calls.
  4. Apply timeouts and output size budgeting (max 100KB per response) to all MCP tool calls.
- **Consequences**: Zero silent execution; defense-in-depth against malicious MCP servers; predictable resource usage.

---

### ADR-006: Plugin and Extension API, Manifest Format and Sandbox Strategy
- **Status**: Approved
- **Context**: Extensibility is required for language packs, themes, and tools, while preserving editor performance and memory safety.
- **Decision**:
  1. Retain and build upon Lapce's WebAssembly (WASI) plugin host powered by `wasmtime`.
  2. Define versioned plugin manifests (`volt.toml`) declaring required permissions (network, filesystem roots).
  3. Isolate plugins in individual memory sandboxes; plugin crashes cannot crash the host process.
- **Consequences**: Safe cross-platform plugin execution; native performance; no dynamic C-ABI vulnerability.

---

### ADR-007: Local API v1 Authentication, Loopback Binding and Scoped SSE
- **Status**: Approved
- **Context**: External tools and CLI scripts may interact with Antares, requiring a programmatic API that cannot be hijacked by malicious local processes.
- **Decision**:
  1. Bind the Local API HTTP/WebSocket server exclusively to loopback (`127.0.0.1` / `::1`).
  2. Require high-entropy Bearer token authentication generated per editor session.
  3. Enforce granular authorization scopes (`read`, `run`, `approve`, `workspace-write`).
  4. Stream real-time editor and agent events via Server-Sent Events (SSE).
- **Consequences**: Safe local scripting; protection against cross-site request forgery and unauthorized local access.

---

### ADR-008: Hybrid Git Architecture, Credential Helpers and Worktree Isolation
- **Status**: Approved
- **Context**: Git operations must be fast, interactive, and completely safe against accidental data loss or force-push.
- **Decision**:
  1. Use `git2` (libgit2) for fast in-process status queries, diff computation, index staging, and blame.
  2. Delegate authentication, network operations (fetch, pull, push), and credential helper negotiation to the system Git CLI.
  3. Isolate multi-agent branches using Git worktrees (`git worktree add`).
  4. Enforce safety checks: disable force-push by default; require explicit confirmation dialogs for destructive actions (hard reset, branch deletion).
- **Consequences**: Snappy UI; seamless integration with user SSH keys and credential managers; no accidental data overwrites.

---

### ADR-009: Semantic IconRegistry, Licensing and Custom Icon Pack Specification
- **Status**: Approved
- **Context**: Lapce's icon system mixes multiple icon sets without consistent semantic keys or documented licensing.
- **Decision**:
  1. Implement a unified `IconRegistry` mapping semantic enum keys (e.g. `Icon::AgentOrchestrator`, `Icon::DiffAdded`) to vector SVG assets.
  2. Use permissible open-source icon fonts: Lucide UI (ISC), Codicons (Creative Commons), and Simple Icons (CC0).
  3. Support custom user-installable icon packs via structured JSON/TOML metadata manifests.
- **Consequences**: Cohesive visual design; license compliance; full user theme customization.

---

### ADR-010: Discord RPC IPC Architecture, Opt-in Lifecycle and Privacy Isolation
- **Status**: Approved
- **Context**: Discord Rich Presence provides community engagement but poses serious privacy risks if code content, file names, or private repositories are leaked.
- **Decision**:
  1. Discord RPC is strictly **disabled by default (opt-in only)**.
  2. When enabled, display only high-level status (e.g. "Editing Rust project", elapsed time). Never broadcast repository names, file paths, branch names, code snippets, or agent prompts.
  3. Execute Discord IPC on a background thread with exponential backoff reconnect; IPC disconnection or daemon absence has zero impact on editor responsiveness.
- **Consequences**: Total privacy protection; zero editor stutter from external daemon state.

---

### ADR-011: Settings and Data Directory Migration from Lapce with Rollback
- **Status**: Approved
- **Context**: Existing Lapce users upgrading to Antares must not lose their settings, themes, keymaps, or extensions.
- **Decision**:
  1. Store Antares configuration and data under `dev/antares/Antares-{branch}`.
  2. On first run, if Antares config is empty and legacy Lapce configuration (`dev/lapce/Lapce-{branch}`) exists, execute automated non-destructive import (`Directory::migrate_directory_content`).
  3. Never delete or mutate the original Lapce directory; user can roll back at any time.
  4. Ensure migration is idempotent: existing Antares files are never overwritten.
- **Consequences**: Frictionless user upgrade; zero data loss; clean uninstall/rollback safety.

---

### ADR-012: Performance Budgets, Process Budgets and CI Regression Gates
- **Status**: Approved
- **Context**: Antares must preserve the blazing speed and low memory footprint of Lapce while hosting complex AI agent capabilities.
- **Decision**:
  1. **UI Frame Budget**: Target 60 fps (<=16.6ms per frame); zero synchronous I/O or token parsing on the Floem render thread.
  2. **Memory Budget**: Baseline idle editor RSS <= 100 MB.
  3. **Transcript Budget**: Bounded chat windows with message windowing and virtualization to prevent memory bloat during 10,000+ token sessions.
  4. **CI Regression Gate**: Fast crate checks must complete in <= 60 seconds; full workspace checks in <= 6 minutes.
- **Consequences**: Fast startup; lightweight idle footprint; instant keyboard responsiveness even under heavy agent generation.
