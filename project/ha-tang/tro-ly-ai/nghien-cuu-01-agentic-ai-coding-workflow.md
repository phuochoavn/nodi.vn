# 📚 NC Trợ Lý AI #01: Agentic AI Coding Workflow cho Nodi POS

> **Nguồn**: Google DeepSearch
> **Ngày**: 2026-03-19
> **Prompt**: AI Engineering Manager — Agentic workflows, prompt engineering, multi-agent
> **Liên quan**: AGENTS.md, CLAUDE.md, TRO_LY_DIEU_HANH.md, .agent/skills/

## Tóm tắt

Software 3.0 paradigm: prompt-driven dev, founder = CEO đội kỹ sư AI. So sánh 7 Agentic IDE (Antigravity tốt nhất cho Nodi). Three-Layer Memory chống context overflow. 10 Nguyên Tắc Vàng. Ma trận Human/AI delegation. SKILL.md Progressive Disclosure (3 cấp). Executive Assistant 4 bước. Case study Devin, Shopify, Bolt.new.

---

## 1. Software 3.0 — Prompt-Driven Development

| Giai đoạn | Đặc trưng | Vai trò con người |
|-----------|---------|-------------------|
| **1.0** | Logic cứng (C++, Java) | Viết từng dòng code |
| **2.0** | Neural networks, học từ data | Thiết kế model, train |
| **3.0** | LLM = lớp máy tính lập trình | **Mô tả WHAT, AI làm HOW** |

> Founder không chuyên kỹ thuật → CEO đội kỹ sư AI → mô tả ý định, AI thực thi

---

## 2. So sánh Agentic IDE 2025-2026

| IDE | Phức tạp | Tự trị | Phù hợp Nodi |
|-----|:--------:|:------:|:------------:|
| **Antigravity** | 9 | 8.5 | ✅ **Rất cao** — multi-agent, Manager surface |
| **Claude Code** (CLI) | 9 | 9 | ✅ Cao — SWE-bench 80.9%, nhưng CLI |
| **Cursor** | 8 | 7.5 | ✅ Cao — Plan Mode, Composer |
| **Windsurf** | 7 | 7 | 🟡 TB — workflows mạnh, hay crash |
| **Cline** | 7 | 8 | ❌ Thấp — quá riskky cho non-coder |
| **Bolt.new** | 3 | 8 | ❌ — không support Tauri/Rust |
| **Devin AI** | 8.5 | 10 | ❌ — "hỗn loạn tự trị" nếu không giám sát |

---

## 3. Three-Layer Memory (Chống context overflow)

| Layer | Nội dung | Thời gian sống |
|-------|---------|:-------------:|
| **Working Context** | Files đang edit, chỉ thị tác vụ | Phiên hiện tại |
| **Session Transcripts** | Checkpoints, trạng thái trung gian | Phục hồi khi lỗi |
| **Persistent KB** | AGENTS.md, KI system, docs/ | Vĩnh viễn |

> Antigravity đã có KI (Knowledge Items) = Layer 3 tự động

### RAG cho Codebase
- Không nhồi toàn bộ code vào context
- Tạo docs/ phân miền: ARCHITECTURE.md, STATE_MANAGEMENT.md, DATABASE_SCHEMA.md
- AGENTS.md liên kết: "Đọc @docs/DATABASE_SCHEMA.md trước khi sửa DB"
- MCP (Model Context Protocol) cho truy vấn real-time

---

## 4. Prompt Engineering cho Code Agents

### 3 Kỹ thuật chính

| Kỹ thuật | Khi nào | Ví dụ |
|---------|--------|-------|
| **Zero-shot** | Logic đơn giản, thuật toán chung | "Viết hàm sort array" |
| **Few-shot** | Convention đặc thù (Tauri IPC) | Cung cấp mẫu #[tauri::command] |
| **Chain-of-Thought** | Debug phức tạp | "Phân tích step-by-step, 3 nguyên nhân tiềm năng" |

### 4 Anti-Patterns phải tránh

| Anti-Pattern | Sai | Đúng |
|-------------|-----|------|
| **Omni-Prompt** | "Làm cho code tốt hơn" | "Tối ưu hiệu năng query trong invoices.rs" |
| **Thiếu constraint** | Không nói giới hạn | "KHÔNG lưu key trong localStorage, dùng Rust Tauri" |
| **Ý định mơ hồ** | "Làm đẹp hơn" | "Redesign grid sản phẩm, render < 50ms cho 10K items" |
| **Tin tuyệt đối** | Merge không review | Luôn review trước khi merge |

---

## 5. Skill vs Workflow vs Rule

| Loại | Tính chất | Khi nào | Ví dụ Nodi |
|------|---------|--------|-----------|
| **Global Rule** | Khai báo, luôn tồn tại | Mọi tương tác | "Luôn dùng Composition API" |
| **Workflow** | Step-by-step, slash command | Quy trình lặp lại | `/build`, `/commit` |
| **Skill** | Module chuyên sâu, on-demand | Bài toán phức tạp | `tauri-ipc-command`, `add-new-intent` |

### SKILL.md Progressive Disclosure (3 cấp)

| Cấp | Nội dung | Token | Khi nào load |
|:---:|---------|:-----:|:----------:|
| 1 | YAML Frontmatter (name, description) | ~100 | **Luôn luôn** |
| 2 | Instructions chi tiết | < 5000 | Khi trigger match |
| 3 | Scripts/resources bên ngoài | 0 (chạy shell) | Khi cần |

---

## 6. Multi-Agent Orchestration

### Coordinator Pattern (phù hợp Nodi nhất)

```
Founder (ngôn ngữ tự nhiên)
    ↓
Tech Lead Agent (Opus 4.6 — KHÔNG code, chỉ phân tích)
    ↓ Hierarchical Task Decomposition
    ├── Backend Agent (Rust)
    ├── Frontend Agent (Vue 3)
    └── QA Agent (Tests)
```

### Git Worktree cho Multi-Agent

```bash
# Mỗi agent = 1 worktree riêng
git worktree add ../nodi-backend feat-db-schema
git worktree add ../nodi-frontend feat-ui-pos
# Không ghi đè nhau, conflict chỉ ở merge phase
```

---

## 7. Executive Assistant Framework (4 bước)

| Bước | Ai làm | Sản phẩm |
|------|--------|---------|
| 1. Vision | Founder | Tóm tắt 1-2 câu |
| 2. Plan Mode | AI | SPEC.md chi tiết, edge cases |
| 3. Human Review | Founder | Tinh chỉnh SPEC.md |
| 4. Milestone Execution | AI | Code + test tuần tự |

---

## 8. Ma Trận Human vs AI

| Hạng mục | **Founder quyết định** | **AI thực thi** |
|---------|:---:|:---:|
| Logic nghiệp vụ (UX, công nợ, luồng bán) | ✅ 100% | Chỉ thực thi |
| Kiến trúc hạ tầng (DB, sync, stack) | Phê duyệt | ✅ Phân tích + đề xuất |
| Bảo mật (RBAC, IPC, auth) | Nguyên tắc nghiệp vụ | ✅ Triển khai kỹ thuật |
| Code + Test | Review định kỳ | ✅ **100% trách nhiệm** |

---

## 9. 10 Nguyên Tắc Vàng

1. **PM Mindset** — Đối xử AI như dev giỏi nhưng thiếu context business
2. **Encode Intent Upfront** — Mục tiêu vĩ mô trước, kỹ thuật sau
3. **Plan Mode trước khi code** — `/plan` trước mọi thay đổi lớn
4. **Guardrails** — TypeScript strict, Rust compiler = tường phòng ngự
5. **Never Swallow Errors** — Cấm `.unwrap()` trong Rust, luôn Result<T, E>
6. **TDD Loops** — Viết test trước, code sau
7. **AGENTS.md = Single Source of Truth** — Gộp mọi rules vào 1 nơi
8. **Git Worktree cách ly** — Mỗi agent 1 worktree
9. **Tái cấu trúc khi > 400 dòng** — AI bối rối với file dài
10. **Zero Trust** — AI = xác suất, LUÔN review trước merge

---

## 10. Case Studies

| Dự án | Bài học | Áp dụng Nodi |
|-------|--------|-------------|
| **Devin AI** | Tự trị quá mức → vòng lặp sửa lỗi vô tận | Human-in-the-loop bắt buộc |
| **Bolt.new / v0.dev** | Tạo prototype nhanh, nhưng không support Tauri | Chỉ dùng cho mockup UI |
| **Shopify** | Migration RN Bridge → JSI, giảm 37% TCO | IPC JSON → MessagePack |
| **Replit Agent** | Sandbox environment, tốt cho learning | Không phù hợp production |

---

## Nguồn tham khảo

- Andrej Karpathy: Software 3.0 concept
- Anthropic: Claude Code & Agent architecture
- Agentic AI Foundation: AGENTS.md standard
- GitHub Copilot: Agent Mode, Prompt Files
- Cursor: Plan Mode, .cursorrules
- Windsurf: Cascade workflows
- Cognition Labs: Devin AI learnings
- Model Context Protocol (MCP) specification
