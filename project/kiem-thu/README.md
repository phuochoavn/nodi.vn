# 🧪 Kiểm Thử & Đánh Giá — Nodi POS

> **Cập nhật**: 19/03/2026
> Folder này là "tòa án" của dự án — nơi đánh giá KHÁCH QUAN, KHÔNG THIÊN VỊ.

---

## ⚖️ Triết Lý QA

> **"QA không phải để chứng minh code đúng — mà để TÌM RA chỗ code SAI."**

### 3 Nguyên Tắc Vàng

1. **Không bao giờ tin agent tự báo cáo** — PHẢI verify độc lập
2. **Đánh giá khách quan** — không bị cuốn theo momentum dự án, không "cho qua" vì "đã cố gắng rồi"
3. **Tìm lỗi trước khi user tìm** — mỗi bug QA bỏ sót = 1 khách hàng mất niềm tin

---

## 📁 Cấu trúc

```
kiem-thu/
├── README.md                 ← File này
├── QA_AGENT_BRIEF.md         ← ⭐ AGENT QA ĐỌC FILE NÀY ĐẦU TIÊN
├── ket-qua/                  ← Kết quả test theo ngày
│   └── 2026-03-19.md
├── regression/               ← Regression test notes
└── tieu-chuan/               ← Tiêu chuẩn chấp nhận (acceptance criteria)
```

---

## 🔗 Files Liên Quan

| File | Mục đích |
|------|---------|
| `ai_training_center/` | Test suite AI chatbot (1238 tests) |
| `project/ha-tang/README.md` | Điểm hạ tầng hiện tại |
| `CLAUDE.md` | Tech stack, conventions |
