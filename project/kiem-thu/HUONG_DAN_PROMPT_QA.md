# 📋 Hướng Dẫn Viết Prompt Cho Agent QA

> Cập nhật: 19/03/2026
> Nguyên tắc: QA KHÔNG được biết WHY (tại sao thay đổi), chỉ được biết WHAT (kiểm tra gì)

---

## ⚠️ Quy Tắc Viết Prompt QA — Tránh Nhiễm Bias

### ❌ SAI — Prompt bị nhiễm

```
"Agent Dev vừa tách PragmaticAIEngine.ts thành 13 modules.
Em đã nghiệm thu build 0 errors, tests 1238/1238 pass.
Anh kiểm tra lại giúp nhé."
```

**Lỗi**: QA biết ai làm, biết đã nghiệm thu pass → thiên vị "chắc đúng rồi"

### ✅ ĐÚNG — Prompt trung lập

```
"Đánh giá kiến trúc AI Engine tại src/services/ai/pragmatic/.
Kiểm tra: build, tests, backward compatibility, code quality.
Đọc QA_AGENT_BRIEF.md trước khi bắt đầu."
```

**Đúng**: Không nói ai làm, không nói đã pass, chỉ nói KIỂM TRA GÌ.

---

### 5 Nguyên Tắc

| # | Nguyên tắc | Ví dụ SAI | Ví dụ ĐÚNG |
|:-:|-----------|----------|------------|
| 1 | **Không nói "đã pass"** | "Build đã 0 errors" | "Kiểm tra build có errors không" |
| 2 | **Không nói ai làm** | "Agent FE vừa tách..." | "Kiểm tra cấu trúc tại..." |
| 3 | **Không giải thích why** | "Tách để giảm LOC..." | "Đánh giá code quality..." |
| 4 | **Chỉ định file/folder cụ thể** | "Kiểm tra code mới" | "Kiểm tra `src/services/ai/pragmatic/`" |
| 5 | **Luôn trỏ đến QA Brief** | Không nhắc brief | "Đọc `project/kiem-thu/QA_AGENT_BRIEF.md`" |

---

## 📝 Template Prompt QA

```
Bạn là QA Agent. Đọc `project/kiem-thu/QA_AGENT_BRIEF.md` trước khi bắt đầu.

Nhiệm vụ: Đánh giá [MÔ TẢ TRUNG LẬP — không nói ai làm, không nói đã pass].

Phạm vi kiểm tra:
- [Files/folders cụ thể]

Tiêu chí đánh giá:
- [Liệt kê criteria cụ thể]

Lưu báo cáo vào: project/kiem-thu/ket-qua/[ngày].md
```
