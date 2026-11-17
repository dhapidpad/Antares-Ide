# Antares Documentation

Đây là mục lục tài liệu chuẩn. Tài liệu ngoài danh sách này không được tự nhận là roadmap/spec chính thức.

## Product & engineering
- [Product Specification](PRODUCT-SPEC.md) — yêu cầu sản phẩm tổng thể, UX/UI, agents, MCP, Git, integrations, security và phases.
- [Design System](DESIGN-SYSTEM.md) — quy tắc UI/UX, layout, màu sắc, typography, motion, icon và accessibility.
- [Architecture](ARCHITECTURE.md) — ranh giới module, luồng dữ liệu và process model.
- [Roadmap](ROADMAP.md) — thứ tự triển khai và phase gates duy nhất.
- [Status](STATUS.md) — trạng thái đã kiểm chứng, không phải wish-list.
- [Security](SECURITY.md) — threat model và permission policy.
- [Performance](PERFORMANCE.md) — budgets, profiling và cleanup.
- [Extensibility](EXTENSIBILITY.md) — agents, workflows, skills, providers, MCP, plugins và local API.
- [Git Integration](GIT-INTEGRATION.md) — Git UX, commands và safety.
- [Branding Migration](BRANDING-MIGRATION.md) — Lapce → Antares, compatibility và data migration.

## Operations
- [Guides](guides/) — build, install và release.
- [Audit](audit/) — baseline và kết quả kiểm kê.
- [ADR](adr/) — quyết định kiến trúc.
- [Reference](reference/) — upstream provenance, patch log và nguồn API.

## Agent execution
- [Rules](../.agent/rules/) — quy tắc luôn áp dụng.
- [Workflows](../.agent/workflows/) — quy trình tác vụ có thể chạy lặp lại.

**Quy tắc:** PRODUCT-SPEC mô tả điều cần có; ARCHITECTURE mô tả hệ thống thực tế; ROADMAP mô tả thứ tự; STATUS chỉ ghi việc có bằng chứng. Không trộn bốn loại nội dung này.
