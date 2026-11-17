# ANTARES IDE — ULTIMATE PRODUCT, UX/UI, AGENT, MCP, GIT & REBRAND MASTER SPEC
> **Tài liệu triển khai chuẩn duy nhất (Single Source of Truth)**  
> Phiên bản: 2.0 · Ngày rà soát: 2026-11-17 · Phạm vi: AntaresIDE fork trên nền mã nguồn Lapce/Floem  
> Trạng thái: **SPECIFICATION / AUDIT REQUIRED** — không được hiểu là các tính năng dưới đây đã được triển khai.

---

## 0. Mục tiêu và nguyên tắc không thỏa hiệp

Xây dựng Antares thành một IDE desktop hoàn chỉnh, đẹp, nhanh, có thể tùy biến sâu, lấy editor làm trung tâm nhưng có hệ sinh thái AI Agent đa tác nhân, workflow, skills, MCP, plugin, local API, Git/GitHub, command palette và Discord Rich Presence.

### 0.1 Kết quả người dùng phải cảm nhận
- Mở app nhanh, ít RAM/CPU khi không làm việc; không chạy agent/server/indexer nền nếu người dùng chưa bật.
- Mọi chức năng quan trọng có thể tìm thấy qua UI, Command Palette và shortcut.
- UI thống nhất: một hệ token, icon language, spacing, typography, motion và trạng thái.
- AI minh bạch: đang dùng agent/model/provider nào, quyền gì, context nào được gửi, tool nào chạy, chi phí/token bao nhiêu.
- Nhiều agent cộng tác được nhưng không giẫm chân nhau; mọi thay đổi code đều có diff và quyền phê duyệt.
- MCP/plugin/API mở rộng được nhưng mặc định an toàn, cô lập, có timeout, log và nút ngắt.
- Git thao tác nhanh nhưng không được tự push/force-push/xóa branch hoặc ghi đè dữ liệu khi chưa có xác nhận.
- Dấu vết Lapce được xử lý có kế hoạch: bề mặt sản phẩm là Antares; phần upstream/license/API được giữ khi cần để bảo toàn nguồn gốc, tương thích và nghĩa vụ pháp lý.

### 0.2 Quy tắc triển khai
1. Không làm redesign trước khi có baseline screenshot, benchmark và test.
2. Không đổi hàng loạt `Lapce` → `Antares` bằng replace-all.
3. Không xóa code chỉ vì `rg` không thấy tham chiếu; Rust macro, feature, build script, dynamic loading, FFI, platform cfg và upstream patch có thể dùng gián tiếp.
4. Không kéo thêm framework UI thứ hai hoặc nhúng webview cho phần editor native nếu chưa có ADR chứng minh lợi ích.
5. Không chạy model, MCP server, Discord RPC hay Git polling liên tục khi tính năng tương ứng tắt.
6. Mọi feature phải có: owner, UX, state model, command IDs, settings, telemetry/privacy stance, tests, docs, rollback.
7. Không ghi “done/verified” nếu không có log lệnh, CI URL hoặc bằng chứng tái lập.

---

## 1. Đọc source snapshot — hiện trạng đã xác nhận

### 1.1 Cấu trúc thực tế quan sát được
- Root Cargo package/bin vẫn tên `lapce`; binary mặc định là `lapce`.
- Workspace members: `lapce-app`, `lapce-proxy`, `lapce-rpc`, `lapce-core`, `antares-agent`.
- Các thư mục upstream `lapce-app/`, `lapce-core/`, `lapce-proxy/`, `lapce-rpc/` vẫn được giữ nguyên.
- Antares UI hiện nằm trong `lapce-app/src/antares/{data.rs,panel.rs,mod.rs}`.
- Agent crate có các module: `audit`, `capabilities`, `codec`, `context`, `error`, `health`, `mcp`, `metrics`, `onboarding`, `patch`, `persistence`, `protocol`, `recovery`, `release`, `session`, `supervisor`, `ui_state`.
- `.agent/rules/`, `.agent/workflows/`, `.agents/guardrails.toml` đã tồn tại.
- Docs hiện có nhiều tài liệu kiến trúc/roadmap/prompt/trạng thái; tài liệu UX/UI trước đã được hợp nhất vào PRODUCT-SPEC và DESIGN-SYSTEM.
- Icon tree có `icons/lapce/` và `icons/codicons/`; tổng số file trong `icons/` snapshot được đếm là 160. Không được tự kết luận số icon khả dụng vì cần parse manifest/font glyph và license.
- Có `lapce.spec`, `Makefile.antares`, các tài liệu upstream provenance trong `docs/reference/` và nhiều references Lapce khác.

### 1.2 Mâu thuẫn cần xử lý trước
`docs/STATUS.md` tuyên bố P0–P8 hoàn thành, CI 61 tests xanh và “Not done: None”. Nhưng `lapce-app/src/antares/data.rs` trong snapshot vẫn để comment placeholder ở side effects `LaunchProcess`, `SendPrompt`, `SendCancel`, `NotifyError`. Do đó:
- Không được coi status file là bằng chứng implementation end-to-end.
- Phải tái chạy CI/test trên commit hiện tại và cập nhật status theo từng capability.
- Cần phân biệt domain reducer đã có với runtime/process wiring thực sự.
- Panel phải được kiểm tra toàn bộ phần còn lại của file; snapshot đang cho thấy header/transcript/tool activity, chưa đủ để khẳng định composer/session UX hoàn chỉnh.

### 1.3 Baseline bắt buộc
Tạo `docs/audit/BASELINE.md` gồm:
- commit SHA, branch, remotes (redact token), OS/toolchain, Cargo version;
- `cargo metadata --no-deps`, workspace member list, feature graph;
- `cargo fmt --check`, `cargo check`, test từng crate, clippy, deny;
- startup time, idle RSS, peak RSS khi mở project lớn, CPU idle, binary size;
- screenshot toàn bộ workbench ở Dark/Light và cửa sổ hẹp;
- danh sách TODO/FIXME/placeholder/`unimplemented!`/`todo!`/dead-code warnings;
- dependency tree, duplicate crates, build time, feature bloat;
- danh sách file được sửa và upstream provenance.

---

## 2. Chiến lược rebrand Lapce → Antares

### 2.1 Định nghĩa “đồng bộ toàn bộ”
Mục tiêu là mọi thứ người dùng nhìn thấy, cài đặt, chạy, tìm kiếm, lưu dữ liệu và tài liệu sản phẩm đều gọi là **Antares**. Tuy nhiên tên nội bộ upstream chỉ đổi khi đã chứng minh không phá API, update path, serialization, plugin compatibility, license hay khả năng sync upstream.

### 2.2 Phân loại mọi occurrence
Mỗi match `Lapce/lapce` phải gán một nhãn trong `docs/audit/REBRAND-INVENTORY.csv`:
- `PRODUCT_SURFACE`: tên app, title, menu, About, onboarding, notifications, docs user-facing → đổi Antares.
- `IDENTITY`: executable, bundle ID, desktop ID, app data/config/cache dirs, protocol identifiers → đổi có migration.
- `INTERNAL_UPSTREAM`: crate/module/API/type/config key giữ để giảm fork divergence; có thể alias/wrapper Antares.
- `UPSTREAM_PROVENANCE`: copyright, license, attribution, source URL, changelog lịch sử → giữ chính xác, không xóa.
- `COMPAT_LEGACY`: tên cũ dùng đọc dữ liệu/cấu hình cũ → giữ trong migration reader, không hiện ra UI.
- `FALSE_POSITIVE`: từ ngữ không liên quan → không sửa.

### 2.3 Bề mặt phải quét
- Root Cargo package, `default-run`, `[[bin]]`, package names, workspace metadata, `Cargo.lock`.
- `lapce-app/core/proxy/rpc` folder/crate/imports và public Rust symbols.
- `lapce.spec`, Makefile, shell/PowerShell scripts, CI workflows, devcontainer.
- Windows exe/product name, installer, registry, file associations, protocol handlers.
- macOS `.app`, bundle identifier, entitlements, signing, update feed.
- Linux desktop entry, AppStream/metainfo, icon theme, RPM/DEB/AppImage/Flatpak.
- App title, window title, About, splash, onboarding, error strings, logs, telemetry names.
- URLs, homepage, docs links, issue templates, badges, release artifacts.
- Icon file names, SVG metadata/title, app logo, tray icon, notification icon.
- User data dirs, cache, logs, settings, workspace state, plugin/extension dirs.
- CLI names/help, proxy process names, environment variables, IPC endpoints.
- MCP server display names và user-facing tool descriptions.
- Tests/fixtures/snapshots/golden files.
- `README`, `CHANGELOG`, `NOTICE`, `THIRD_PARTY_LICENSES`.

### 2.4 Migration không mất dữ liệu
- Tạo `IdentityMigration` có version, dry-run, backup và idempotency.
- Đọc thư mục cấu hình Antares trước; nếu không tồn tại, import từ thư mục Lapce cũ.
- Không xóa thư mục cũ sau import; chỉ đề xuất dọn khi user xác nhận và backup đã kiểm tra.
- Migrate settings, keymaps, themes, snippets, extensions, workspace state, recent projects, sessions.
- Giữ nguyên workspace files, source code và Git metadata.
- Không copy credential/token plaintext vào log; secrets chuyển qua OS credential store nếu có.
- Test upgrade N-1 → N, interrupted migration, rollback, duplicate state, Unicode path, Windows path.
- Cung cấp màn hình “Imported from previous installation” với danh sách nội dung và nút undo/backup.

### 2.5 Lệnh kiểm kê
```bash
rg -n -i --hidden --glob '!target/**' --glob '!.git/**' 'lapce' .
rg -n 'TODO|FIXME|XXX|todo!|unimplemented!|placeholder|not implemented' .
cargo metadata --format-version 1 --no-deps
cargo tree -e features
```
Không dùng kết quả `rg` làm lệnh replace-all. Mỗi thay đổi cần review theo nhóm file.

---

## 3. Antares product architecture — các khu vực sản phẩm

### 3.1 Navigation cấp một
Activity Bar bên trái, thứ tự mặc định:
1. Explorer
2. Search
3. Source Control
4. Run & Debug
5. Extensions
6. **Agents** (tab mới, icon riêng)
7. MCP & Integrations
8. Testing
9. Outline / Symbols
10. Settings / Manage

Người dùng được kéo-thả đổi thứ tự, hide/show, pin/favorite. Lưu theo profile và workspace; có Restore Defaults.

### 3.2 Workbench layout
- Titlebar: logo Antares, workspace/project, command center, layout controls, window actions.
- Activity Bar: 48 px mặc định.
- Primary Sidebar: 260–320 px, resize 200–520 px.
- Editor: vùng trung tâm, tabs, breadcrumbs, split groups.
- Secondary Sidebar: 320–420 px, mặc định chứa Agents khi mở.
- Bottom Panel: Terminal, Problems, Output, Debug Console, Ports, Tasks, Git/History.
- Status Bar: branch, sync, diagnostics, language, encoding, line/column, formatter, agent state, MCP state.
- Panel có thể move, dock left/right/bottom, maximize, hide.
- Mỗi layout có preset: Coding, Agentic, Review, Debug, Minimal, Presentation.
- Không để AI dock che editor hoặc tự giành focus.

### 3.3 Global command center
Một command surface hỗ trợ:
- commands/actions;
- files/symbols/workspaces;
- recent commands;
- agent actions/workflows;
- Git actions;
- MCP tools (chỉ hiển thị tools đã được cấp quyền);
- settings search;
- extension/skill search.
Kết quả có icon, category, shortcut, description, source, permission hint. Fuzzy search, recent ranking, keyboard-first.

---

## 4. Design language: “Antares Aurora”

### 4.1 Hướng thẩm mỹ
Premium developer workstation, tối giản có chiều sâu, không biến IDE thành dashboard rối:
- Dark-first nhưng Light theme hoàn chỉnh.
- Nền graphite/navy trung tính, surface phân lớp nhẹ.
- Accent cyan/ice blue kết hợp violet có kiểm soát.
- Gradient chỉ dành cho brand mark, active agent, onboarding hero và điểm nhấn; không tô gradient toàn bộ panel.
- Border tinh tế, shadow rất nhẹ, không dùng glass blur nặng trên mọi component.
- Typography rõ, code font monospace; không dùng quá nhiều font.
- Icon nét đồng nhất, stroke/filled có quy tắc.

### 4.2 Token Dark — giá trị đề xuất
| Token | Hex | Dùng cho |
|---|---:|---|
| `bg.canvas` | `#0B0F17` | nền chính |
| `bg.sidebar` | `#101722` | sidebar |
| `bg.panel` | `#151E2B` | panel |
| `bg.elevated` | `#1B2635` | popover/menu |
| `bg.hover` | `#202D3E` | hover |
| `bg.active` | `#26364A` | selected |
| `border.subtle` | `#263446` | divider |
| `border.strong` | `#3A4B61` | focus boundary |
| `text.primary` | `#E8F0FA` | nội dung chính |
| `text.secondary` | `#A7B5C8` | phụ |
| `text.muted` | `#718198` | metadata |
| `accent.cyan` | `#65D8F2` | primary accent |
| `accent.blue` | `#6C9EFF` | links/selection |
| `accent.violet` | `#A78BFA` | AI/agent |
| `success` | `#45C995` | thành công |
| `warning` | `#F2B96B` | cảnh báo |
| `danger` | `#F07886` | lỗi/nguy hiểm |
| `info` | `#70B7F5` | thông tin |

### 4.3 Token Light — giá trị đề xuất
| Token | Hex | Dùng cho |
|---|---:|---|
| `bg.canvas` | `#F5F7FB` | nền chính |
| `bg.sidebar` | `#EEF2F8` | sidebar |
| `bg.panel` | `#FFFFFF` | panel |
| `bg.elevated` | `#FFFFFF` | popover |
| `bg.hover` | `#E8EEF7` | hover |
| `bg.active` | `#DCE8F8` | selected |
| `border.subtle` | `#DCE3ED` | divider |
| `border.strong` | `#AEBBCD` | focus boundary |
| `text.primary` | `#182334` | nội dung chính |
| `text.secondary` | `#46566C` | phụ |
| `text.muted` | `#718096` | metadata |
| `accent.cyan` | `#087F9B` | primary accent |
| `accent.blue` | `#326DCE` | links/selection |
| `accent.violet` | `#7651C9` | AI/agent |
| `success` | `#13865F` | thành công |
| `warning` | `#9A5B08` | cảnh báo |
| `danger` | `#C33D51` | lỗi/nguy hiểm |
| `info` | `#246EA9` | thông tin |

Các giá trị trên là seed token; phải đo contrast WCAG 2.2 AA cho text/controls trước khi release. Không hardcode màu trong component.

### 4.4 Layout, typography, density
- Spacing base 4 px; token: 2, 4, 6, 8, 12, 16, 20, 24, 32, 40.
- Radius: 4 (small), 7 (control), 10 (card), 14 (dialog), 18 (hero); tránh bo tròn mọi thứ quá mức.
- Font UI: system stack ưu tiên native; tùy chọn Inter/Geist nếu license và bundling được duyệt.
- Code: JetBrains Mono / Cascadia Code / Fira Code / system monospace; user chọn font cài trên máy.
- Text sizes: 11 metadata, 12 compact, 13 default, 14 section, 16 heading, 20 page title.
- Density: Compact / Comfortable / Spacious; scale spacing và row height đồng bộ.
- Editor line height 1.45–1.7, ligatures off mặc định, minimap configurable.

### 4.5 Component contract
Mọi component phải có states:
`default`, `hover`, `pressed`, `focused`, `selected`, `disabled`, `loading`, `success`, `warning`, `error`, `empty`.
- Tooltip delay 500–700 ms, không bật tooltip khi thao tác keyboard liên tục.
- Focus ring 2 px, tương phản đủ, không chỉ dựa vào màu.
- Hit target tối thiểu 28 px trong IDE dense; menu/action quan trọng 32 px.
- Không có icon-only button không tooltip/accessible label.
- Skeleton chỉ cho nội dung tải bất đồng bộ; không fake progress.
- Empty state phải có CTA đúng ngữ cảnh.
- Toast không che composer, diff hoặc menu; có severity và action.

---

## 5. Icon system — đầy đủ, đồng bộ và custom sâu

### 5.1 Kiến trúc icon
Tạo `IconRegistry` với semantic IDs, không để component gọi trực tiếp tên glyph rải rác:
- `app.*`: logo, product, tray.
- `nav.*`: explorer, search, git, debug, extensions, agents, MCP, tests, settings.
- `file.*`: file/folder theo language/type.
- `editor.*`: format, refactor, rename, code action, split, pin.
- `git.*`: branch, commit, pull, push, merge, rebase, stash, conflict, changed.
- `agent.*`: planner, coder, reviewer, tester, researcher, security, terminal, tool, approval.
- `mcp.*`: server, connected, disconnected, resource, prompt, tool, warning.
- `status.*`: success, info, warning, error, loading, offline.
- `social.*`: Discord, GitHub, GitLab và provider logos (chỉ dùng theo brand guidelines).
- `custom.*`: icon do user/imported theme cung cấp.

### 5.2 Nguồn icon cần đánh giá
- Codicons: phù hợp ngữ nghĩa IDE, kiểm tra license và glyph coverage hiện có.
- Lucide: icon UI phổ thông, stroke đồng bộ.
- Material Symbols: tùy chọn pack bổ sung, cần mapping/weight thống nhất.
- Simple Icons: logo brand/provider; không dùng thay icon hành động.
- Icon pack cộng đồng: chỉ cho phép khi có license rõ ràng, nguồn và version pin.
Không trộn 4 bộ icon trong cùng một toolbar. Chọn một bộ UI chính + Codicons cho ngữ nghĩa editor + Simple Icons cho brand.

### 5.3 Icon customization
- Chọn icon pack global/workspace.
- Scale 12/14/16/18/20/24 px; stroke weight theo pack.
- Filled/outline mode; animation chỉ cho trạng thái hoạt động.
- File association icon themes.
- Import SVG/PNG (SVG phải sanitize, chặn script/external href/foreignObject; raster giới hạn kích thước).
- Preview grid, search, mapping, reset.
- Fallback khi thiếu glyph; không render ô vuông.
- Đồng bộ theme/icon pack theo profile export/import.
- Cache raster/vector có giới hạn; không parse lại mọi icon mỗi frame.

### 5.4 Asset policy
Mỗi icon asset cần `source`, `license`, `version`, `author`, `hash`, `category`; tạo `THIRD_PARTY_ASSETS.md`. Không tải icon ngẫu nhiên từ web vào source mà không kiểm tra license.

---

## 6. Tab Agents — trung tâm đa Agent

### 6.1 Mục tiêu
Tab **Agents** là nơi tạo, chạy, phối hợp, giám sát và đánh giá nhiều agent/workflow. Không chỉ là chat panel. Có thể chạy một agent đơn, một nhóm agent, hoặc workflow dạng graph.

### 6.2 Layout Agents tab
**Header**
- Workspace selector, session selector, New Session, New Workflow.
- Provider/model selector; runtime/local/remote badge.
- Run/Pause/Stop; global budget indicator.
- Connection and permission status.

**Left rail — Sessions**
- Search sessions, filter by workspace/provider/status/agent.
- Pinned, recent, archived.
- Session state: idle/running/waiting approval/failed/completed.
- Branch/worktree association.

**Center — Workflow canvas / conversation**
- Toggle Chat / Board / Graph.
- Workflow graph: nodes, edges, dependencies, retries, conditional branches.
- Live execution timeline.
- Conversation stream grouped by agent.
- Task cards with owner, status, files, elapsed time, token/cost.

**Right inspector**
- Agent config, model, role, tools, MCP servers, skill set, context, permissions.
- Input/output schema.
- Retry policy, timeout, concurrency, budget.
- Trace/logs and artifacts.

**Bottom drawer**
- Tool calls, terminal, logs, diffs, test output, audit events.

### 6.3 Agent catalog mặc định
| Agent | Trách nhiệm | Quyền mặc định |
|---|---|---|
| Orchestrator | phân rã task, giao việc, tổng hợp | đọc context; không tự apply patch |
| Planner/Architect | kiến trúc, plan, ADR | read-only |
| UI/UX Designer | layout, tokens, usability, accessibility | read-only + proposal |
| Implementer | triển khai code theo task | patch proposal |
| Reviewer | review diff, logic, style | read-only |
| Test Engineer | test, repro, coverage | chạy test allowlist |
| Security Auditor | threat model, secret/path/dependency audit | read-only |
| Performance Engineer | benchmark/profile | chạy benchmark allowlist |
| Documentation Agent | docs, changelog, migration | docs patch |
| Git Agent | branch/status/commit/PR draft | đọc; commit/push cần approve |
| MCP Integrator | cấu hình/test MCP | không tự trust server |
| Research Agent | tra cứu docs/API/release notes | network theo policy |
| Refactor Agent | dọn code theo phạm vi nhỏ | proposal-only |
| Release Agent | release checklist/package | không publish nếu chưa duyệt |

### 6.4 Multi-agent workflow
Bắt buộc hỗ trợ:
- Sequential: A → B → C.
- Parallel fan-out/fan-in: nhiều agent độc lập, aggregator tổng hợp.
- Planner → Implementer → Reviewer → Tester.
- Debate/consensus: nhiều reviewer độc lập, có tiêu chí chấm.
- Supervisor-worker: orchestrator quản lý worker.
- Conditional branch: theo test/status/approval.
- Retry with backoff; max retries; circuit breaker.
- Human-in-the-loop gate.
- Sub-workflow/reusable workflow.
- Cancel propagation và cleanup.
- Pause/resume/recover sau crash.
- Max concurrency, per-agent budget, global budget.
- Conflict avoidance qua isolated Git worktree/branch hoặc file ownership lock.
- Merge queue và conflict resolution có diff review.

### 6.5 Workflow editor UX
- Canvas snap-to-grid, zoom, pan, minimap, auto-layout.
- Node palette: Agent, Prompt, Skill, MCP Tool, Condition, Approval, Parallel, Join, Delay, Retry, Git, Test, Output.
- Node config forms generated from typed schema.
- Validate graph trước run: cycle, missing input, invalid provider, permission mismatch, unreachable node.
- Save as reusable `.antares/workflows/*.yaml` hoặc JSON versioned schema.
- Import/export, duplicate, template gallery.
- Dry-run/plan preview.
- Version history and diff.
- Workflow run timeline; replay only deterministic recorded events, không giả lập external side effects.
- Node states: queued/running/waiting/blocked/succeeded/failed/cancelled/skipped.

### 6.6 Agent isolation & resource governance
- Agent process tách biệt theo runtime adapter.
- Per-agent working directory/worktree.
- CPU/memory/time/output limits.
- Tool allowlist và denylist.
- No unrestricted shell by default.
- Không share mutable context object giữa agent; message passing có schema.
- Context provenance: file/selection/diagnostic/user prompt, timestamp, hash.
- Redact secrets trước khi đưa vào prompt/log.
- Bounded transcript, streaming backpressure, output truncation rõ ràng.
- Agent không được tự cấp quyền cho agent khác.

---

## 7. Provider, Local API, runtime adapters

### 7.1 Provider interface
Chuẩn hóa trait/API nội bộ cho:
- `discover_models`
- `validate_credentials`
- `start_session`
- `send_turn`
- `stream_events`
- `cancel_turn`
- `list_capabilities`
- `health_check`
- `usage_metrics`
- `close_session`
Provider adapter không được leak type riêng vào UI/domain.

### 7.2 Nhóm provider
- Hosted API: OpenAI-compatible, Anthropic-compatible, Gemini/Google, các provider chính thức khác khi API được xác minh.
- Local runtime: Ollama, llama.cpp server, LM Studio, vLLM, LocalAI, endpoint OpenAI-compatible.
- Agent runtime/protocol: ACP-compatible agent process (Octomind hoặc runtime khác) qua adapter riêng.
- Custom endpoint: base URL, auth method, model ID, headers allowlist, timeout, TLS policy.
Không hardcode model cụ thể trong UI. Không khẳng định tương thích chỉ vì cùng format OpenAI.

### 7.3 Local API & control plane
- Localhost-only mặc định (`127.0.0.1`), port random/ephemeral; không bind `0.0.0.0` nếu user không chủ động.
- Token auth bắt buộc; token lưu OS keychain/credential store.
- API versioning `/api/v1`.
- Endpoints mục tiêu: health, capabilities, providers, agents, sessions, workflows, runs, approvals, tools, MCP servers, settings subset, events (SSE/WebSocket).
- OpenAPI schema export và generated client.
- Rate limit, request size limit, CORS deny-by-default, origin validation, CSRF protection nếu có browser surface.
- Local API permission scopes: `read`, `run`, `approve`, `workspace-write`, `git-write`, `admin`.
- Audit every privileged action; revoke token; rotate token.
- Không expose secret, raw env, arbitrary filesystem hoặc shell endpoint.

### 7.4 Provider UX
- Provider cards: name, type, local/remote, status, latency, model count.
- Add/Edit/Test/Disable/Delete; test không gửi source code.
- Credential status “Configured” không show secret.
- Per-workspace default, per-agent override, fallback chain.
- Model capability badges: tools, vision, context length, streaming, JSON schema; chỉ hiển thị khi provider trả metadata hoặc verified manifest.
- Connection failure có actionable error, retry, docs link.
- Model routing policy: cost, latency, capability, privacy, local-first.
- Offline mode và provider health cache.

---

## 8. MCP Hub & plugin ecosystem

### 8.1 MCP Hub
- Server catalog, installed servers, workspace-only servers, disabled servers.
- Add from stdio command, HTTP transport, supported remote endpoint.
- Server identity, publisher/source, version, trust status, transport, health.
- Capabilities: tools/resources/prompts; schema viewer; tool search.
- Per-server enable/disable; per-tool allow/deny; agent allowlist.
- Tool invocation confirmation theo risk; read-only tools có thể policy allow.
- Timeout, cancellation, retry safe policy, output cap, rate limits.
- stderr/stdout separation với stdio; không để logs làm hỏng JSON-RPC stream.
- Environment variables/secrets qua secret references, không plaintext trong config export.
- Server process lifecycle, restart/backoff, health monitor chỉ khi active.
- MCP protocol version/capability negotiation; unknown capability graceful fallback.
- Log tool name, server, duration, status, approval ID; redact arguments/results nhạy cảm.

### 8.2 Plugin manager
Phân biệt rõ:
1. MCP server: tool/resource/prompt integration.
2. Antares extension/plugin: UI contribution, commands, views, language/theme.
3. Agent skill: chỉ dẫn/workflow capability cho agent.
4. Provider adapter: kết nối model/runtime.
Không coi các khái niệm này là một loại plugin chung.

Plugin manifest cần:
- id/name/version/publisher/description/license;
- engine compatibility;
- permissions/scopes;
- contribution points;
- dependencies/conflicts;
- entrypoint/hash/signature;
- update channel;
- uninstall/rollback metadata.
Cài đặt: marketplace hoặc file local đã xác minh; preview permissions trước khi install.
Không tự động tải skill từ Claude/Cloud/website và thực thi. Import chỉ sau scan, preview, trust confirmation.

### 8.3 Tương thích
- MCP SDK/protocol versions phải pin và test contract.
- Capability detection, không giả định tất cả server hỗ trợ mọi feature.
- ACP adapter và MCP adapter tách biệt.
- Có adapter layer để runtime khác nhau không làm rối domain.
- Compatibility matrix theo OS, transport, server version, feature.
- Unknown field được xử lý forward-compatible khi schema cho phép.
- Contract tests với fake server, malformed response, timeout, disconnect, restart.

### 8.4 Security tiers
- Tier 0: metadata/read-only, không workspace access.
- Tier 1: đọc file/workspace bounded.
- Tier 2: đề xuất patch; không apply.
- Tier 3: chạy command allowlist sau approve.
- Tier 4: ghi file/commit/push; approve từng hành động.
- Tier 5: admin/local API/plugin install; yêu cầu xác thực rõ ràng.
Không có “trust all” mặc định.

---

## 9. Git & remote collaboration — thao tác nhanh

### 9.1 Source Control view
- Changes: staged, unstaged, untracked, conflicted.
- Inline diff, side-by-side diff, word diff, file history.
- Stage/unstage file/hunk/selection.
- Discard file/hunk với confirm; không có discard all một click không xác nhận.
- Commit message editor, amend, sign commit (GPG/SSH nếu configured).
- Branch tree, tags, remotes, stash, worktrees.
- Graph: commits, authors, refs, ahead/behind.
- Blame/annotate, file history, line history.
- Merge conflict editor 3-way (ours/theirs/base/result).
- Git status trong status bar.

### 9.2 Git Quick Actions
Command Palette và toolbar:
- Clone repository…
- Open repository…
- Initialize repository…
- Fetch
- Pull (rebase/merge theo preference)
- Push
- Sync
- Create/switch/rename/delete branch
- Checkout commit/tag
- Stage/unstage
- Commit/amend
- Stash push/pop/apply/drop
- Cherry-pick
- Revert commit
- Merge/rebase
- Abort merge/rebase/cherry-pick
- Resolve conflicts
- Open remote URL
- Create PR/MR (qua provider integration)
- Copy commit hash/branch/remote URL
- Open terminal at repo root
- Create worktree / remove worktree
- Git clean preview (không clean trực tiếp nếu chưa confirm)
Mọi action destructive phải hiển thị scope và preview.

### 9.3 Clone UX
- URL input hỗ trợ HTTPS/SSH, provider URL parsing.
- Folder destination picker, name preview, shallow clone option, branch selection.
- Credential helper/OS keychain; không hỏi paste token vào URL.
- Clone progress, cancel, retry, network diagnostics.
- Sau clone: Open project, trust workspace, scan language tools có consent.
- Không tự chạy post-clone scripts/tasks.

### 9.4 Push safety
- Hiển thị remote, branch, commits/files sẽ push.
- Detect force push/non-fast-forward.
- Force push chỉ qua explicit dialog, mặc định block.
- Không lưu token trong remote URL/log.
- Auto-fetch không quá thường xuyên; configurable.
- Commit signing state và upstream tracking rõ ràng.

### 9.5 Git provider integrations
GitHub/GitLab/Forgejo/Gitea/Bitbucket qua provider interface. OAuth/device flow hoặc credential helper. PR/MR create/review, checks/status, issues chỉ bật khi plugin/connector có quyền. Core Git vẫn hoạt động offline.

---

## 10. Quick Commands & command system

### 10.1 Command registry
Mỗi command có:
`id`, `title`, `category`, `description`, `icon_id`, `keybinding`, `when`, `risk_level`, `required_permissions`, `handler`, `telemetry_policy`.
- ID ổn định; label dịch được.
- Command có thể đến từ core, plugin, skill, workflow, MCP action nhưng phải namespace.
- Collision detection, disable command contribution, permission check.
- Command Palette hiển thị nguồn và trust state.

### 10.2 Nhóm lệnh cần có
**Workbench:** New Window, Open Folder, Save All, Toggle Sidebar, Toggle Panel, Focus Editor, Zen Mode, Layout presets.

**Editor:** Go to File, Go to Symbol, Go to Line, Find/Replace, Format, Rename Symbol, Code Actions, Quick Fix, Fold/Unfold, Multi-cursor, Split Editor, Move Editor, Reopen Closed Editor.

**Project:** Add Folder, Workspace Settings, Trust Workspace, Project Tasks, Recent Projects.

**Terminal/Tasks:** New Terminal, Split Terminal, Run Task, Stop Task, Select Default Shell, Clear Terminal.

**Git:** toàn bộ quick actions tại mục 9.2.

**Agents:** New Agent Session, Run Workflow, Pause/Resume/Cancel Run, Add Context, Review Pending Patch, Switch Provider, Open Agent Logs, Export Session.

**MCP:** Add Server, Test Server, Enable/Disable Server, Inspect Tools, Revoke Permission, Restart Server.

**Extensions/Skills:** Browse, Install, Disable, Update, Inspect Permissions, Create Skill, Validate Skill.

**Diagnostics:** Developer: Show Logs, Open Logs Folder, Run Doctor, Copy Environment Info (redacted), Reset UI Layout, Safe Mode.

### 10.3 Shortcut design
- Không trùng shortcut nền tảng OS hoặc editor phổ biến.
- Context-aware `when` clause.
- Chord support, conflict resolver, import/export keymap.
- Shortcut discoverability trong menus/tooltips.
- Có searchable Keyboard Shortcuts editor.

---

## 11. Discord Rich Presence (RPC)

### 11.1 Mục tiêu và mặc định
Tùy chọn, mặc định OFF. Không gửi nội dung source code, tên file nhạy cảm, prompt, branch private, URL repo hoặc đường dẫn local.
Presence chỉ hiển thị trạng thái tổng quát: “Editing”, “Debugging”, “Reviewing changes”, “Running tests”, “Working with Antares Agents”.
- Toggle global và per-workspace.
- Privacy mode: Off / Generic / Detailed (Detailed vẫn không gửi code).
- Preview trước khi enable.
- Pause when idle / when locked / when Do Not Disturb.
- Reconnect/backoff; không spam update.
- Discord app/client ID cấu hình từ build env; không nhúng secret.
- Disconnect/revoke và clear presence.
- Discord unavailable không ảnh hưởng startup/editor.
- Không truyền prompt, code, tokens, path hoặc repo remote.
- Kiểm tra Discord Rich Presence SDK/IPC policy hiện hành trước implementation; không tự viết RPC protocol dựa trên blog cũ.

### 11.2 UI
Settings → Integrations → Discord:
- Enable Rich Presence
- Show current activity
- Show language (optional)
- Show elapsed time
- Hide when private workspace
- Preview card
- Connection status / Reconnect / Disconnect

---

## 12. Settings & customization center

### 12.1 Settings information architecture
- Appearance: themes, colors, typography, density, icon pack, motion, layout.
- Editor: font, tabs, minimap, guides, wrapping, suggestions, formatting.
- Workbench: panels, activity bar, status bar, window, startup.
- Keybindings: editor, workbench, agent, git.
- Agents: catalog, orchestration, default policies, context.
- Providers: hosted/local/custom API.
- MCP: servers, permissions, transports.
- Plugins: marketplace, installed, updates, permissions.
- Skills: sources, installed, trust, validation.
- Git: identity, signing, fetch/pull/push, providers, diff.
- Integrations: Discord, GitHub, GitLab, terminal, external tools.
- Privacy & Security: secret store, audit, workspace trust, local API.
- Performance: memory, indexing, cache, background jobs.
- Updates: stable/preview, release notes.
- Advanced: paths, logs, diagnostics, experimental features.

### 12.2 Theme editor
- Live preview; edit semantic tokens, not raw component styles.
- Color picker + HEX/RGB/HSL; contrast warning.
- Import/export theme JSON; schema version.
- Per-syntax token colors; semantic diagnostics.
- Theme inheritance, diff, reset individual token.
- Preview editor, terminal, diff, chat, workflow, dialogs.
- Validate required tokens and contrast before apply.
- Theme switching no full app restart unless technically unavoidable.

### 12.3 Layout and icon customization
- Drag/drop panel positions and Activity Bar.
- Save named layouts; workspace profile override.
- Icon pack browser, per-category mapping, preview, fallback.
- Export/import profile package with manifest; no secrets.
- Sync settings opt-in, conflict resolution and local backup.

### 12.4 Advanced customization
- User CSS-like styling is **not** allowed to inject arbitrary code into native UI.
- Theme schema supports tokens and component variants.
- Plugin contributions must use supported extension points.
- Custom commands can be declarative or sandboxed; arbitrary scripts require explicit trust and permissions.

---

## 13. Performance: nhẹ, nhanh, không chạy thừa

### 13.1 Performance budgets (mục tiêu cần benchmark)
| Metric | Target |
|---|---:|
| Cold start on reference machine | ≤ 2.5 s |
| Idle CPU after settled | near 0%; no periodic busy loop |
| Idle memory | establish baseline; target no >10% regression per release |
| UI interaction p95 | ≤ 100 ms |
| Command palette open | ≤ 100 ms |
| Agent panel open (no runtime launch) | ≤ 100 ms |
| Theme switch | ≤ 250 ms |
| No frame stall during stream | no >100 ms UI-thread blocking |
| Background indexing | pause/throttle on battery/idle policy |
| Crash-free session | measured by release channel |

Targets are engineering goals, not claims about current performance. Record hardware/OS/project size and median/p95.

### 13.2 Lazy loading
- Agent runtime starts only when first agent action occurs.
- MCP server starts only when enabled and requested (unless explicit keep-alive).
- Discord RPC starts only when enabled.
- Extension marketplace network calls only when opened or scheduled by user.
- Git auto-fetch disabled or conservative by default.
- Heavy icon pack/theme previews load on demand.
- Large transcript uses virtualized/windowed list.
- Workflow canvas renders visible nodes only where feasible.
- Project indexing incremental, ignore-aware, cancellable, resource capped.
- Do not duplicate LSP/indexing already provided by editor.

### 13.3 Remove bloat safely
- Audit Cargo dependencies with `cargo tree`, `cargo tree -d`, `cargo bloat` (if installed), `cargo llvm-lines` (if installed).
- Check feature flags; disable unused defaults only after behavior tests.
- Remove duplicate libraries only when API compatibility and compile graph permit.
- Avoid adding async/runtime stacks if existing runtime/channel infrastructure suffices.
- Avoid bundling browser engine, model weights, icon fonts or language servers unnecessarily.
- No always-on polling; use event-driven notifications.
- Cache with TTL, size cap, eviction and invalidation.
- Keep logs bounded and rotate.
- Measure release binary size before/after every dependency change.

### 13.4 Memory and process lifecycle
- Central process supervisor for agent/MCP/helper subprocesses.
- Explicit ownership, cancellation token, child reaping, timeout.
- Kill orphan process on app exit; graceful shutdown first, force kill after deadline.
- Backpressure for streaming; cap stdout/stderr and transcript.
- No blocking file/network/process calls on UI thread.
- Avoid cloning large editor buffers; use snapshots/rope slices where supported.
- Release inactive session resources; preserve session state on disk.

---

## 14. Dead-code & legacy cleanup — quy trình an toàn

### 14.1 Inventory
Tạo:
- `docs/audit/DEAD-CODE-CANDIDATES.md`
- `docs/audit/DEPENDENCY-REVIEW.csv`
- `docs/audit/PLACEHOLDER-INVENTORY.md`
- `docs/audit/LEGACY-DOCS-MAP.md`

### 14.2 Candidate discovery
```bash
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo tree -d
cargo tree -e features
rg -n 'TODO|FIXME|XXX|todo!|unimplemented!|placeholder|stub' --glob '!target/**' .
rg -n 'dead_code|allow\\(dead_code\\)|cfg\\(' --glob '*.rs' .
```
Dùng `cargo machete` nếu khả dụng để tìm dependency không dùng; xác minh false positive. Có thể dùng `cargo udeps`/`cargo bloat` trong CI phụ, nhưng không bắt buộc developer cài toàn cục.

### 14.3 Không xóa tự động
Trước khi xóa:
1. Kiểm tra mọi feature/platform target.
2. Kiểm tra macro-generated refs, reflection, serde names, command registration, inventory/linkme, plugin loading.
3. Kiểm tra public API/semver và downstream consumers.
4. Kiểm tra build scripts, integration tests, fixtures, docs examples.
5. Xóa theo batch nhỏ, một commit/PR có rollback.
6. Chạy tests liên quan và full workspace CI.
7. Đo binary size/build time/RSS trước-sau.

### 14.4 Loại code cần ưu tiên
- Placeholder side effects chưa được nối runtime.
- Duplicate helpers/types không còn owner.
- Dead feature flags và dependency không dùng.
- Debug-only UI/log spam trong release.
- UI state không được đọc/ghi.
- Old migration code chỉ sau khi policy retention hết hạn và có backup.
- Unused icon assets sau khi xác minh không được tham chiếu bởi generated code/theme.
- Old docs trùng lặp hoặc mâu thuẫn sau khi đã migrate nội dung.

### 14.5 Không được dọn
- LICENSE, NOTICE, copyright headers, THIRD_PARTY_LICENSES.
- Upstream source cần thiết cho editor, syntax, LSP, terminal, RPC, platform support.
- Compatibility readers và migration tests.
- Generated files nếu build pipeline yêu cầu.
- Fixtures, golden snapshots, benchmark data đang được dùng.
- `lapce` references mang ý nghĩa provenance hoặc internal upstream contract cho đến khi có migration plan.

---

## 15. Cấu trúc Markdown chuẩn — không tạo tài liệu cạnh tranh

### 15.1 Single source of truth
- `docs/README.md`: index tài liệu.
- `docs/PRODUCT-SPEC.md`: yêu cầu sản phẩm và phạm vi tính năng.
- `docs/DESIGN-SYSTEM.md`: UI/UX, tokens, motion, icon và accessibility.
- `docs/ARCHITECTURE.md`: hệ thống và ranh giới module đang được thiết kế/triển khai.
- `docs/ROADMAP.md`: phase và gate duy nhất.
- `docs/STATUS.md`: tiến độ đã có bằng chứng.
- `docs/SECURITY.md`, `docs/PERFORMANCE.md`, `docs/EXTENSIBILITY.md`, `docs/GIT-INTEGRATION.md`, `docs/BRANDING-MIGRATION.md`: các chuyên đề.
- `docs/adr/`: quyết định kiến trúc. `docs/audit/`: baseline và audit. `docs/guides/`: hướng dẫn vận hành. `docs/reference/`: upstream provenance, patch log và API sources.

### 15.2 Quy tắc dọn tài liệu
- Không tạo master prompt, roadmap hoặc status thứ hai.
- Tài liệu cũ trùng lặp đã được loại khỏi docs root; nội dung vận hành còn cần thiết được chuyển vào `guides/`, `adr/`, `audit/` hoặc `reference/`.
- Giữ README, CHANGELOG, CONTRIBUTING, LICENSE/NOTICE, third-party licenses, GitHub templates và test fixture docs.
- Trước khi xóa bất kỳ tài liệu nào khác, tìm inbound links, chuyển nội dung cần thiết, cập nhật links và kiểm tra lại.
- `.agent/rules/` là quy tắc thực thi; `.agent/workflows/` là quy trình thao tác; chúng không thay thế product spec.

## 16. Security & trust model

- Workspace Trust: project chưa trust không chạy task/extension/agent script.
- Secret store: OS keychain/credential manager; file config chỉ lưu reference.
- Context privacy: preview file/path/selection trước khi gửi; exclude secrets, `.env`, key files, build artifacts, large/binary.
- Patch apply: hash check, path containment, symlink check, conflict detection, atomic write/rollback.
- Shell: argv-based invocation, avoid shell string concatenation; command allowlist; working directory containment; env scrub.
- MCP: server allowlist, permission tiers, approval and audit.
- Plugin: signature/hash, publisher, permission manifest, update rollback.
- Local API: loopback, token, scopes, rate limit, origin checks.
- Git: credential helper; never store token in remote URL; explicit destructive action approval.
- Discord: generic presence only; privacy default off.
- Audit logs append-only where feasible, bounded/rotated, redact secrets.
- Security review before enabling auto-approve or unattended agent mode.

---

## 17. Accessibility, localization & usability

- Keyboard-first, full tab order, visible focus.
- Screen reader labels for controls and icon-only actions.
- Contrast AA for text and controls.
- Do not encode status by color alone; icon + text/shape.
- Font scaling 80–160%; no clipping.
- Reduced motion, high contrast, color-blind safe status.
- Vietnamese and English UI localization baseline; avoid hardcoded user-facing strings.
- Date/time/number formatting locale-aware.
- Tooltips and errors explain recovery action.
- All destructive operations reversible where possible.

---

## 18. Testing & release gates

### 18.1 Test layers
- Unit: reducers, workflow graph validation, provider routing, permission evaluation, migration.
- Contract: ACP, MCP, provider adapters, plugin manifest, Local API.
- Integration: multi-agent run, pause/cancel/recovery, tool approval, patch apply/rollback.
- UI: keyboard navigation, focus, resize, theme, empty/loading/error, workflow canvas.
- Security: traversal, symlink escape, secret redaction, malicious MCP output, prompt injection, local API auth.
- Performance: startup, idle, large workspace, 10k transcript, concurrent agents, MCP flood.
- Packaging: Windows/macOS/Linux identifiers, icons, upgrade, uninstall, migration.
- Docs: links, command examples, schema examples, version consistency.

### 18.2 Release gate
- No P0/P1 security bugs.
- No fake controls or placeholder side effects on release paths.
- Full workspace tests and CI pass on supported OS matrix.
- No new clippy warnings.
- Baseline performance no unexplained regression.
- Migration tested with backup/rollback.
- License/assets audit complete.
- `STATUS.md` linked to exact commit and CI evidence.
- Feature flags documented; experimental features off by default.

---

## 19. Roadmap — phase order and completion criteria

### PHASE 0 — Reality audit & baseline
- [ ] Pin commit, branch, remotes, toolchain.
- [ ] Run fmt/check/test/clippy/deny; capture logs.
- [ ] Verify `docs/STATUS.md` claims.
- [ ] Inventory Lapce occurrences by category.
- [ ] Inventory all docs, dependencies, features, icons, TODOs.
- [ ] Capture UI screenshots and performance baseline.
**Gate:** audit reports evidence and risk; no code deletion yet.

### PHASE 1 — Brand identity & migration
- [ ] Product name, logo, title, menus, CLI, app metadata.
- [ ] Binary/package/installer identity plan.
- [ ] Data directory migration + backup/rollback.
- [ ] User-facing docs and links canonicalized.
- [ ] Legal/provenance preserved.
**Gate:** clean install + upgrade test across OS.

### PHASE 2 — Architecture and docs consolidation
- [ ] Canonical docs structure created.
- [ ] Old docs mapped, merged, archived with redirects/links.
- [ ] Architecture boundaries and ADRs approved.
- [ ] Status reflects verified implementation only.
**Gate:** no conflicting roadmap or duplicate master prompt.

### PHASE 3 — Design system & icon foundation
- [ ] Tokens Dark/Light, contrast test.
- [ ] Typography/spacing/radius/density.
- [ ] Component states and focus/accessibility.
- [ ] IconRegistry and pack licensing.
- [ ] Theme/icon import/export.
**Gate:** all existing panels render consistently.

### PHASE 4 — Workbench redesign
- [ ] Activity Bar, sidebars, editor, bottom panel, status bar.
- [ ] Resizable/dockable panels and named layouts.
- [ ] Command center and keyboard shortcuts.
- [ ] Empty/loading/error states.
**Gate:** screenshot review + keyboard-only smoke test.

### PHASE 5 — Agent runtime correctness
- [ ] Replace placeholder side effects with real supervisor bridge.
- [ ] Process lifecycle, streaming, cancellation, restart, recovery.
- [ ] Composer, session management, errors, retry, context picker.
- [ ] Patch review and explicit apply.
**Gate:** end-to-end agent prompt → stream → tool → diff → approve tested.

### PHASE 6 — Agents tab & multi-agent orchestration
- [ ] Agent catalog and role config.
- [ ] Workflow graph schema/editor.
- [ ] Sequential/parallel/supervisor/approval/retry.
- [ ] Isolated worktrees and file conflict management.
- [ ] Budgets, traces, pause/resume/recovery.
**Gate:** multi-agent workflow completes with deterministic audit trail.

### PHASE 7 — Provider & Local API
- [ ] Provider trait/adapters.
- [ ] Hosted/local/custom endpoint configuration.
- [ ] Model capability discovery and routing.
- [ ] Local API v1, auth, scopes, OpenAPI, SSE.
**Gate:** local endpoint cannot be accessed without token and scope.

### PHASE 8 — MCP Hub & plugins
- [ ] MCP server registry and lifecycle.
- [ ] Tools/resources/prompts capability UI.
- [ ] Permissions, approval, logs, timeout, output limits.
- [ ] Plugin manifest, install/update/uninstall/rollback.
- [ ] Skill import/validation/trust workflow.
**Gate:** malicious/invalid server tests pass; no silent tool execution.

### PHASE 9 — Git & collaboration
- [ ] Clone/open/init.
- [ ] Stage/commit/branch/fetch/pull/push.
- [ ] Diff/history/blame/conflict/worktree.
- [ ] Git provider integrations and PR/MR flow.
- [ ] Destructive action confirmations.
**Gate:** offline core Git works; credential and push safety tests pass.

### PHASE 10 — Integrations & Discord RPC
- [ ] Discord RPC opt-in, privacy preview, reconnect.
- [ ] GitHub/GitLab/terminal integrations.
- [ ] External API/plugin compatibility matrix.
**Gate:** Discord failure cannot affect editor; privacy tests pass.

### PHASE 11 — Customization, motion & polish
- [ ] Theme editor, icon packs, layout profiles.
- [ ] Motion tokens, reduced motion, performance.
- [ ] Settings search, export/import, reset.
- [ ] Localization and accessibility.
**Gate:** all themes/layouts tested across supported OS.

### PHASE 12 — Dead-code and dependency diet
- [ ] Candidate inventory reviewed by owner.
- [ ] Remove unused dependencies/features with evidence.
- [ ] Remove stale code/docs/assets only after link/reference checks.
- [ ] Benchmark before/after and rollback.
**Gate:** no behavior regression; measurable size/build/RSS improvement or justified maintenance gain.

### PHASE 13 — Release hardening
- [ ] Cross-platform packaging, signing, update/migration.
- [ ] Security audit, performance suite, crash recovery.
- [ ] Documentation and release notes.
- [ ] Stable/preview channel policy.
**Gate:** release checklist signed off.

---

## 20. Definition of Done — feature nào cũng phải đạt

- [ ] Có UI đúng design tokens, không hardcode màu/icon.
- [ ] Có command palette entry và shortcut nếu phù hợp.
- [ ] Có loading/empty/error/disabled/success states.
- [ ] Có keyboard navigation, focus và accessible label.
- [ ] Có settings và default policy rõ.
- [ ] Có unit/integration/contract tests.
- [ ] Có security permissions và audit nếu đụng tool/files/network.
- [ ] Có performance budget và cancellation.
- [ ] Có docs canonical và migration nếu đổi schema.
- [ ] Có log/test evidence gắn với commit.
- [ ] Không có fake button, mock success hoặc placeholder trên đường release.
- [ ] Có rollback hoặc recovery path cho thao tác phá hủy.

---

## 21. Master implementation prompt cho coding agents

Bạn là Principal Engineer/UX Architect của Antares IDE. Làm việc trực tiếp trên repository hiện tại, đọc code và tài liệu trước khi sửa.

**Bắt buộc:**
1. Đọc tài liệu này và `docs/audit/BASELINE.md`.
2. Chỉ thực hiện đúng một phase được giao; không tự nhảy phase.
3. Trước khi sửa, liệt kê file dự kiến thay đổi, API liên quan, rủi ro và test.
4. Không replace-all Lapce; phân loại theo BRANDING-MIGRATION.
5. Không xóa dead code/dependency/docs khi chưa có evidence và review.
6. Không thêm framework/dependency lớn nếu không có ADR, benchmark và lý do.
7. Không làm fake UI; action phải hoạt động end-to-end.
8. Mọi agent/MCP/plugin/local API/Git action tuân thủ permission model.
9. Tất cả patch code phải có test, format, clippy và cập nhật docs canonical.
10. Khi kết thúc, báo cáo: files changed, behavior added, tests run + exact results, limitations, risks, rollback, next phase.
11. Nếu không chạy được test, ghi **NOT VERIFIED**, không được ghi PASS.
12. Dừng khi phase gate chưa đạt; không tự tuyên bố hoàn tất.

**Phong cách UI:** Antares Aurora — premium, restrained, crisp, coherent, keyboard-first, performant. Tránh gradient/glass/animation phô trương. Mọi hiệu ứng phục vụ hierarchy, feedback và orientation.

**Ưu tiên:** correctness → security → performance → consistency → visual polish. Không hy sinh dữ liệu, source provenance hoặc ổn định để đổi tên cho nhanh.

---

## 22. Audit checklist đầu tiên cho repo hiện tại

- [ ] Kiểm tra lại `lapce-app/src/antares/data.rs`: side effects còn placeholder hay đã có runtime bridge.
- [ ] Kiểm tra đầy đủ `lapce-app/src/antares/panel.rs`: composer, send/cancel, session, errors, context, approvals.
- [ ] So khớp `docs/STATUS.md` với code và CI hiện tại.
- [ ] Kiểm tra `antares-agent/src/mcp.rs`: đây là host-owned workspace tools hay generic MCP client/server; không đánh đồng.
- [ ] Kiểm tra supervisor có process launch thật, ACP event loop, shutdown và recovery.
- [ ] Kiểm tra multi-agent orchestration: hiện có thật hay mới role catalog/supervisor scaffolding.
- [ ] Kiểm tra provider adapters: model API hosted/local/custom đã có đến đâu.
- [ ] Kiểm tra plugin extension API: hiện có hay chưa; MCP không thay thế plugin UI API.
- [ ] Kiểm tra Discord RPC chưa có trong scope code hiện tại hay đã có integration.
- [ ] Kiểm tra Git quick actions: phân biệt Git core UI hiện có và các flow clone/push/PR còn thiếu.
- [ ] Parse icon manifests/font maps; lập inventory theo semantic ID, license và usage.
- [ ] Xác định `lapce.spec`, package/bin names, desktop metadata, `homepage = lapce.dev` và identity migration.
- [ ] Audit toàn bộ Markdown, inbound links và prompt duplication.
- [ ] Đo dependency/build/binary/RSS trước khi cleanup.

---

## 23. Quyết định kiến trúc cần ADR trước khi code

1. `ADR-001`: mức độ đổi tên crate/folder nội bộ hay giữ upstream names + Antares facade.
2. `ADR-002`: agent runtime process boundary và ACP adapter strategy.
3. `ADR-003`: multi-agent orchestration engine, workflow schema và persistence.
4. `ADR-004`: provider abstraction và local model API compatibility.
5. `ADR-005`: MCP client architecture, transports và server trust model.
6. `ADR-006`: plugin/extension API và sandbox strategy.
7. `ADR-007`: Local API authentication, scopes, event transport.
8. `ADR-008`: Git backend (git2/CLI/hybrid), credential strategy và worktree model.
9. `ADR-009`: icon sources, licenses, semantic registry và custom pack format.
10. `ADR-010`: Discord RPC SDK/IPC and privacy.
11. `ADR-011`: settings/data migration and backward compatibility.
12. `ADR-012`: dependency budget, process budget and performance CI.

Không bắt đầu phase phụ thuộc ADR nếu quyết định đó chưa được ghi lại và review.

---

## 24. Quy tắc cập nhật tài liệu

- `PRODUCT-SPEC.md` mô tả yêu cầu sản phẩm; không báo tiến độ.
- `ROADMAP.md` mô tả phase/gate; không chứa checklist trùng lặp ở nhiều nơi.
- `STATUS.md` chỉ ghi trạng thái có bằng chứng.
- `ARCHITECTURE.md` mô tả hệ thống đang tồn tại, không mô tả wish-list.
- `SECURITY.md` mô tả threat model và controls đã triển khai; phần chưa làm ghi rõ gap.
- Mỗi phase hoàn thành cập nhật test evidence và ADR.
- Mỗi release xóa/archival docs chỉ khi link check pass.
- Tài liệu cũ được archive, không xóa lịch sử hoặc legal/provenance.

**Kết luận:** Antares cần được hoàn thiện theo một hệ thống thống nhất chứ không phải một đợt đổi skin. Rebrand, agent orchestration, MCP/plugin, Git, local API, Discord, icon packs, customization và performance phải cùng tuân theo một kiến trúc, một design system và một permission model. Làm theo phase, đo được, kiểm chứng được, rollback được.
