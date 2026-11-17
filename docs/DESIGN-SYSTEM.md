# Antares Aurora — Design System & UX Rules

> Canonical UI specification. Mọi panel, dialog, menu, agent view, MCP view và plugin contribution phải tuân thủ.

## 1. Design principles
1. **Editor-first:** editor là vùng làm việc chính; sidebar và AI không được chiếm focus ngoài ý muốn.
2. **Quiet premium:** tinh tế, rõ hierarchy, không neon/glass/gradient tràn lan.
3. **One system:** dùng semantic tokens, spacing scale, typography scale và IconRegistry; không hardcode style.
4. **Keyboard-first:** mọi thao tác chính có command và shortcut; chuột không phải con đường duy nhất.
5. **Honest states:** không giả tiến trình, không fake success, nút chưa hỗ trợ phải disabled và giải thích.
6. **Fast by default:** animation và rendering không được ảnh hưởng typing, scroll, selection hay stream.
7. **Accessible:** focus, contrast, labels, reduced motion và scale font là yêu cầu nền tảng.

## 2. Visual direction
- Brand direction: **Antares Aurora** — graphite/navy, ice-cyan, controlled violet.
- Dark theme mặc định; Light theme là theme hạng nhất, không phải bản đảo màu.
- Gradient chỉ dùng cho brand mark, hero, selected agent indicator và onboarding. Không dùng gradient cho mọi card/button.
- Surface phân lớp bằng tone/border; shadow rất nhẹ; hạn chế blur vì tốn GPU và khó đọc.
- Không dùng quá 2 accent nổi bật trong cùng một vùng UI.

## 3. Semantic color tokens

### Dark
| Token | Value | Usage |
|---|---|---|
| `canvas` | `#0B0F17` | app background |
| `sidebar` | `#101722` | sidebars |
| `panel` | `#151E2B` | panels |
| `elevated` | `#1B2635` | menus/popovers |
| `hover` | `#202D3E` | hover |
| `selected` | `#26364A` | selected rows |
| `border-subtle` | `#263446` | separators |
| `border-strong` | `#3A4B61` | boundaries |
| `text-primary` | `#E8F0FA` | primary text |
| `text-secondary` | `#A7B5C8` | secondary text |
| `text-muted` | `#718198` | metadata |
| `accent-cyan` | `#65D8F2` | primary accent |
| `accent-blue` | `#6C9EFF` | links/selection |
| `accent-violet` | `#A78BFA` | AI identity |
| `success` | `#45C995` | success |
| `warning` | `#F2B96B` | warning |
| `danger` | `#F07886` | destructive/error |

### Light
| Token | Value | Usage |
|---|---|---|
| `canvas` | `#F5F7FB` | app background |
| `sidebar` | `#EEF2F8` | sidebars |
| `panel` | `#FFFFFF` | panels |
| `elevated` | `#FFFFFF` | menus/popovers |
| `hover` | `#E8EEF7` | hover |
| `selected` | `#DCE8F8` | selected rows |
| `border-subtle` | `#DCE3ED` | separators |
| `border-strong` | `#AEBBCD` | boundaries |
| `text-primary` | `#182334` | primary text |
| `text-secondary` | `#46566C` | secondary text |
| `text-muted` | `#718096` | metadata |
| `accent-cyan` | `#087F9B` | primary accent |
| `accent-blue` | `#326DCE` | links/selection |
| `accent-violet` | `#7651C9` | AI identity |
| `success` | `#13865F` | success |
| `warning` | `#9A5B08` | warning |
| `danger` | `#C33D51` | destructive/error |

Các màu là seed values; phải chạy contrast audit WCAG 2.2 AA trước release. Màu trạng thái luôn đi cùng icon/label, không truyền đạt chỉ bằng màu.

## 4. Layout grid & sizing
- Spacing base 4 px; scale: 2, 4, 6, 8, 12, 16, 20, 24, 32, 40.
- Activity Bar: 48 px; icon center aligned.
- Primary Sidebar: 260–320 px mặc định, resize 200–520 px.
- Secondary Sidebar: 320–420 px mặc định, resize được.
- Bottom Panel: 180–280 px mặc định, có maximize/collapse.
- Editor tối thiểu 45% chiều ngang ở layout desktop tiêu chuẩn; panel không tự đẩy editor xuống kích thước không dùng được.
- Radius: 4 px compact, 7 px controls, 10 px cards, 14 px dialogs, 18 px hero.
- Density: Compact / Comfortable / Spacious; điều chỉnh row height, padding và list spacing đồng bộ.
- Layout presets: Coding, Agentic, Review, Debug, Minimal; user có thể save/restore.

## 5. Typography
- UI ưu tiên system font native; font custom phải có license và fallback.
- Code font user-selectable: JetBrains Mono, Cascadia Code, Fira Code hoặc monospace system.
- Scale: 11 metadata, 12 compact, 13 default, 14 section, 16 heading, 20 page title.
- Không dùng quá 3 weight trong một view.
- Code line height 1.45–1.7; ligatures configurable, default off.
- Không dùng uppercase dài cho labels; sentence case thống nhất.

## 6. Component anatomy & states
Mỗi reusable component phải hỗ trợ trạng thái phù hợp: `default`, `hover`, `pressed`, `focused`, `selected`, `disabled`, `loading`, `success`, `warning`, `error`, `empty`.
- Button hierarchy: Primary (một hành động chính/vùng), Secondary, Tertiary/Ghost, Destructive.
- Icon-only action bắt buộc tooltip và accessible label.
- Focus ring 2 px, tương phản rõ; không xóa outline nếu không có thay thế.
- Tooltip delay 500–700 ms; không bật khi người dùng đang điều hướng keyboard nhanh.
- Toast không che editor selection, agent composer, diff actions hoặc modal controls.
- Empty state có giải thích + CTA hợp ngữ cảnh; error state có retry/recovery khi có thể.
- Loading indicator chỉ dùng khi thật sự có async operation; progress chỉ hiển thị số liệu thật.

## 7. Workbench composition
- Titlebar: brand, workspace, command center, layout control, window actions.
- Activity Bar: Explorer, Search, Source Control, Run & Debug, Extensions, Agents, MCP, Testing, Settings.
- Primary Sidebar: nội dung navigation đang chọn.
- Editor: tabs, breadcrumbs, split groups; không nhồi card dashboard.
- Secondary Sidebar: agent/session inspector hoặc outline tùy layout.
- Bottom Panel: Terminal, Problems, Output, Debug Console, Ports, Tasks, Git history.
- Status Bar: branch, sync, diagnostics, language, encoding, cursor, formatter và trạng thái agent/MCP gọn.
- Panel drag/drop, resize, dock và reset phải hoạt động nhất quán.

## 8. Agents UX rules
- Phân biệt rõ agent identity, provider/model, runtime, role và permission.
- Mỗi run có trạng thái, elapsed time, token/cost nếu provider cung cấp, tool activity và cancel.
- Nội dung stream append vào message hiện tại; không dựng lại toàn bộ transcript mỗi token.
- Diff proposal phải có file list, hunk preview, base hash, accept/reject; không auto-apply.
- Tool call có risk label và approval gate theo policy.
- Workflow graph có node types rõ, validation trước run, status per node, trace và recovery.
- Không tự focus composer khi agent stream; người dùng giữ quyền điều hướng.

## 9. MCP / Plugin UX rules
- Phân biệt MCP server, Antares extension, Agent Skill và Provider adapter.
- Mỗi server/plugin hiển thị publisher/source, version, trust, permissions, enabled state.
- Hiển thị tool schema trước khi user cấp quyền; tool result là untrusted text.
- Permission review trước install/enable; revoke dễ tìm.
- Không render tool output dưới dạng HTML; không tự mở URL hoặc chạy command.

## 10. Motion system
- Motion purpose: orientation, feedback, hierarchy; không trang trí vô nghĩa.
- Micro interaction 100–160 ms; panel/overlay 160–220 ms; complex transition tối đa khoảng 260 ms.
- Easing: standard ease-out cho enter, ease-in cho exit, ease-in-out cho reposition.
- Không animate layout continuously trong lúc streaming; tránh blur, spring physics nặng và particle effects.
- Reduced Motion: tắt parallax, looping, scale/slide dài; thay bằng instant/fade ngắn.
- Không chuyển động icon liên tục; chỉ animate trạng thái loading/connected có ý nghĩa.
- Animation phải có cancellation và không block input.

## 11. Icon system
- Dùng semantic `IconRegistry`; component không gọi glyph raw rải rác.
- Một bộ UI chính (ví dụ Lucide) + Codicons cho semantics editor + Simple Icons chỉ cho brand.
- Không trộn style filled/outline tùy tiện trong một toolbar.
- Semantic namespaces: `app`, `nav`, `file`, `editor`, `git`, `agent`, `mcp`, `status`, `social`, `custom`.
- Custom packs phải có manifest, version, source, author, license, hash.
- SVG phải sanitize; cấm script, external href, foreignObject và oversized filters. Raster phải giới hạn kích thước.
- Missing icon có fallback, không hiển thị tofu/ô vuông.
- User có thể chọn pack, scale, stroke/filled mode, file association và import/export.

## 12. Responsive behavior & accessibility
- Sidebar có collapse; khi cửa sổ hẹp, panel chuyển sang overlay/drawer không che critical editor controls.
- Font scale 80–160% không clipping.
- Keyboard navigation đầy đủ; tab order logic; shortcut conflict resolver.
- Screen reader label cho icon-only controls và live status không spam.
- WCAG 2.2 AA contrast cho text/controls; focus visible; status không chỉ dựa màu.
- High contrast mode và reduced motion.

## 13. UI performance gates
- Không blocking I/O, process, parsing hoặc model calls trên UI thread.
- Virtualize transcript, file list và workflow nodes khi cần.
- Không rebuild whole panel on every token.
- Không global polling khi có event/watcher.
- Theme switch không restart toàn app trừ khi framework bắt buộc và có lý do.
- Test p95 interaction, frame stalls, startup, idle RSS/CPU và large workspace.

## 14. UI review checklist
- [ ] Visual hierarchy rõ; editor vẫn là trung tâm.
- [ ] Dark/Light, Compact/Comfortable/Spacious đều không vỡ layout.
- [ ] Không hardcode màu/spacing/font/icon.
- [ ] Mọi action có loading/success/error/cancel đúng thực tế.
- [ ] Focus, keyboard, tooltip và accessible label đầy đủ.
- [ ] Reduced motion hoạt động.
- [ ] Empty/error states có hướng xử lý.
- [ ] Không che editor hoặc steal focus.
- [ ] Không gây frame stall khi agent stream.
- [ ] Screenshot và manual smoke checklist được lưu trong PR.
