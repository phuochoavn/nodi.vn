# 🤖 Trợ Lý AI — Agentic Coding & Workflow Optimization

> AI agent strategy, prompt engineering, skills, multi-agent orchestration

---

## 🏆 Tinh Hoa Cần Làm — Tổng hợp từ Nghiên Cứu

### 🔴 Ưu tiên Cao nhất

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 1 | **10 Nguyên Tắc Vàng → TRO_LY_DIEU_HANH.md** | Nhúng 10 rules vào quy trình trợ lý | Nhỏ |
| 2 | **Ma trận Human/AI → TRO_LY_DIEU_HANH.md** | 4 hàng phân quyền rõ: UX, Infra, Security, Code | Nhỏ |
| 3 | **Anti-Patterns → TRO_LY_DIEU_HANH.md** | 4 lỗi prompt phổ biến + cách tránh | Nhỏ |

### 🟠 Ưu tiên Cao

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 4 | **Tạo thêm skills** | migration-sqlite, backup-restore, deploy-apk | TB |
| 5 | **Nâng cấp tauri-ipc-command skill** | So sánh NC mới vs skill hiện có, merge best of both | Nhỏ |
| 6 | **Workflow `/plan`** | Plan Mode trước mọi thay đổi lớn | Nhỏ |

### 🟡 Ưu tiên Trung bình

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 7 | **Git Worktree script** | Tự động tạo worktree cho multi-agent sessions | TB |
| 8 | **RAG docs structure** | Tạo docs/ ARCHITECTURE.md, STATE_MANAGEMENT.md | TB |
| 9 | **Token budget monitoring** | Track chi phí API, khi nào Opus vs Sonnet | Nhỏ |

---

### 💎 5 Nguyên tắc AI Agent

1. **"Founder = CEO, AI = dev team"** — Mô tả WHAT, AI làm HOW
2. **"Plan trước, code sau"** — LUÔN `/plan` trước thay đổi lớn
3. **"Skill on-demand, Rule always-on"** — Tách rõ 3 cấp chỉ thị
4. **"Zero Trust mọi output"** — AI = xác suất, review trước merge
5. **"File > 400 dòng = tách"** — AI bối rối với file dài, refactor kịp thời

---

## Nghiên Cứu (Google DeepSearch — 19/03/2026)

| # | File | Nội dung | Điểm |
|:-:|------|----------|:----:|
| 01 | `nghien-cuu-01-agentic-ai-coding-workflow.md` | IDE comparison, 10 Rules, Human/AI matrix, SKILL.md template | **92** |

---

## Hiện trạng Nodi POS

| Tính năng | Hiện tại | Mục tiêu |
|----------|:-------:|:-------:|
| AGENTS.md | ✅ Multi-agent topology | + Executive Assistant workflow |
| CLAUDE.md | ✅ Coding standards | + 10 Nguyên Tắc Vàng |
| TRO_LY_DIEU_HANH.md | ✅ 510 dòng, 9 sections | + 3 sections mới |
| Skills | 2 (ipc, intent) | + 3-5 thêm (migration, backup, deploy) |
| Workflows | 3 (/build, /commit, /full-pipeline) | + /plan |
| Knowledge Items | ✅ 18 KIs | Tối ưu RAG patterns |
