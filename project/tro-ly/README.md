# 🤖 Trợ Lý AI Agent — Nodi POS

> **Cập nhật**: 19/03/2026
> Đây là trung tâm điều phối cho tất cả AI agents làm việc trên dự án Nodi POS.

---

## 📖 Mục đích

Folder này chứa QUY TRÌNH và KIẾN THỨC để mọi agent (mới hay cũ) đều:
1. **Biết mình là ai** — vai trò, scope, giới hạn
2. **Biết đọc gì** — file nào cần đọc trước khi làm
3. **Biết làm xong thì làm gì** — sắp xếp briefs, ghi nhật ký, báo cáo

---

## 📁 Cấu trúc

| File | Nội dung |
|------|---------|
| `README.md` | File này — hướng dẫn tổng quan cho agent |
| [`TRO_LY_DIEU_HANH.md`](./TRO_LY_DIEU_HANH.md) | Quy trình điều hành, 10 Rules, Ma trận phân công, Sổ Bài Học |

---

## 🎯 2 Môi Trường Agent — App vs VPS

> Agent App và Agent VPS hoạt động ở 2 nơi KHÁC NHAU, brief phải ghi rõ.

| | 🖥️ Agent App (PC + Mobile) | 🌐 Agent VPS |
|--|:--:|:--:|
| **Chạy ở đâu** | Máy local (Antigravity IDE) | VPS (SSH terminal) |
| **Code** | `src/` (Vue) + `src-tauri/` (Rust) | Docker, Nginx, PostgreSQL |
| **Build** | `cmd /c npm run build` + `cargo build` | `docker compose up --build` |
| **Test** | `npx tsx ai_training_center/test_runner.ts` | `curl` API + `psql` queries |
| **Repo** | `agri-pos-tauri` (local) | Repo riêng trên VPS |

### Khi viết brief phải ghi rõ:

- **Brief App** → `cmd /c`, files trong `src/` hoặc `src-tauri/`
- **Brief VPS** → Lệnh Linux, files trên server
- **Cùng 1 feature** (ví dụ sync) có thể cần **2 briefs song song** — 1 App + 1 VPS

---

## 🔄 Quy Trình Bắt Buộc Cho MỌI Agent

### Trước khi làm

1. ĐỌC brief trong `briefs/active/` (brief giao cho mình)
2. ĐỌC `TRO_LY_DIEU_HANH.md` → phần "Sổ Bài Học" (tránh lặp sai lầm)
3. ĐỌC `CLAUDE.md` → phần Pre-Fix Checklist
4. XÁC NHẬN mình hiểu task → bắt đầu làm

### Trong khi làm

1. Chạy build/test sau mỗi thay đổi lớn
2. Log progress vào output (agent tự ghi)
3. Nếu phát hiện vấn đề ngoài scope → GHI CHÚ, KHÔNG TỰ Ý SỬA

### Sau khi làm xong

1. **CHẠY BUILD** — `cmd /c npm run build` hoặc `cmd /c cargo build`
2. **CHẠY TEST** — `cmd /c npx tsx ai_training_center/test_runner.ts`
3. **BÁO CÁO** — theo format trong brief
4. **⚠️ QUAN TRỌNG — SẮP XẾP BRIEF**:

```
Sau khi hoàn tất → DI CHUYỂN brief từ:
  briefs/active/[tên-brief].md
    →
  briefs/done/[tên-brief].md
```

5. **GHI NHẬT KÝ** — thêm entry vào `project/nhat-ky/[ngày].md` (format xem mẫu)
6. **🔄 NẾU FIX BUGS TỪ QA** → PHẢI viết prompt QA retest ngay (xem [Quy trình QA](#-quy-trình-gọi-agent-qa-quan-trọng))

> ⚠️ Các bước 4, 5, 6 là BẮT BUỘC. Không hoàn thành = chưa xong task.
> ⚠️ Bước 6 đặc biệt quan trọng: Dev fix xong → Trợ lý TỰ ĐỘNG viết prompt QA retest → gửi user. KHÔNG CHỜ user nhắc.

---

## 📂 Cách Sắp Xếp Briefs

```
briefs/
├── active/         ← Briefs đang chờ hoặc đang thực hiện
│   ├── [agent]-[task].md
│   └── ...
├── done/           ← Briefs đã hoàn tất (lưu trữ)
│   ├── 2026-03-19_refactor-ai-engine.md
│   ├── 2026-03-19_optimize-bundle-size.md
│   └── ...
└── templates/      ← Template cho mỗi loại brief
    ├── TEMPLATE_FE.md
    ├── TEMPLATE_BE.md
    ├── TEMPLATE_VPS.md
    └── TEMPLATE_QA.md
```

### Quy tắc đặt tên brief đã done

```
[YYYY-MM-DD]_[tên-brief-gốc].md
```

Ví dụ: `2026-03-19_refactor-ai-engine.md`

### Ai di chuyển?

- **Agent thực hiện brief** → di chuyển brief vào `done/` sau khi hoàn tất
- **Agent nghiệm thu** → xác nhận brief đã ở `done/`
- Nếu brief bị quên trong `active/` > 7 ngày → Tech Lead dọn

---

## 📝 Cách Ghi Nhật Ký

File: `project/nhat-ky/[YYYY-MM-DD].md`

Format mỗi entry:

```markdown
## [emoji] [DD/MM/YYYY] ([giờ bắt đầu]–[giờ kết thúc]) — [Tên việc]

### Kết quả

- [Bullet 1]
- [Bullet 2]
- Brief: `briefs/done/[tên].md`

---
```

Entry MỚI NHẤT ở ĐẦU file (prepend, không append).

---

## 🎓 Sổ Bài Học

Nằm trong `TRO_LY_DIEU_HANH.md` → phần cuối.

Mọi agent PHẢI đọc trước khi làm. Nếu phát hiện bài học mới → THÊM VÀO.

---

## 🔗 Files Liên Quan

| File | Đọc khi nào |
|------|-----------|
| `CLAUDE.md` | Mọi lúc — tech stack, conventions |
| `project/ha-tang/README.md` | Khi cần hiểu kiến trúc tổng |
| `project/kiem-thu/QA_AGENT_BRIEF.md` | Agent QA bắt buộc đọc |
| `project/kiem-thu/HUONG_DAN_PROMPT_QA.md` | **Khi viết prompt cho Agent QA** |
| `.agent/AGENTS.md` | Hiểu topology team |

---

## 🧪 Quy Trình Gọi Agent QA (QUAN TRỌNG)

> **Đọc kỹ `project/kiem-thu/HUONG_DAN_PROMPT_QA.md` trước khi viết prompt QA.**

### Khi nào cần gọi QA?

- Sau sprint lớn (> 3 briefs hoàn tất)
- Trước build APK
- Trước Go-Live
- Khi user yêu cầu

### Cách viết prompt QA — TRÁNH NHIỄM BIAS

**5 nguyên tắc bắt buộc:**

1. ❌ **KHÔNG nói "đã pass"** — để QA tự verify
2. ❌ **KHÔNG nói ai làm** — QA không cần biết dev nào
3. ❌ **KHÔNG giải thích tại sao** — QA chỉ cần biết kiểm tra gì
4. ✅ **CHỈ nêu file/folder cần kiểm tra** — phạm vi rõ ràng
5. ✅ **LUÔN trỏ đến QA_AGENT_BRIEF.md** — để QA biết quy trình

### Template chuẩn

```
Bạn là QA Agent. Đọc file project/kiem-thu/QA_AGENT_BRIEF.md trước khi bắt đầu.

Nhiệm vụ: Đánh giá [MÔ TẢ TRUNG LẬP]

Phạm vi:
- [Files/folders cụ thể]

Kiểm tra:
1. [Tiêu chí 1]
2. [Tiêu chí 2]
...

Format báo cáo: Theo QA_AGENT_BRIEF.md — BẮT BUỘC có cột "Bằng chứng"
Lưu vào: project/kiem-thu/ket-qua/[ngày]_[tên].md
```

### ⚠️ CẤM KỴ tuyệt đối

- ❌ KHÔNG tự làm QA (agent trợ lý đã tham gia dev → nhiễm bias)
- ❌ KHÔNG gửi walkthrough/reasoning của mình cho QA
- ❌ KHÔNG viết "em đã nghiệm thu pass" trong prompt QA

