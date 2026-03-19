# 🗄️ Cơ Sở Dữ Liệu — Database Architecture

> Schema design, migration strategy, backup/restore, data integrity

## Tài Liệu

| File | Nội dung | Trạng thái |
|------|----------|:----------:|
| `erd-diagram.md` | Entity Relationship Diagram toàn bộ schema | ⬜ Cần viết |
| `migration-strategy.md` | Chiến lược migration (46 migrations hiện tại) | ⬜ Cần viết |
| `backup-restore.md` | Quy trình backup & khôi phục dữ liệu | ⬜ Cần viết |
| `data-integrity.md` | Quy tắc toàn vẹn dữ liệu (constraints, triggers) | ⬜ Cần viết |

## Hiện Trạng

- SQLite local (rusqlite) — 46 migrations
- 20+ bảng chính (products, customers, invoices, suppliers...)
- Hybrid migration: schema.sql base + incremental migrations
- Supplier debt resync mỗi lần startup
