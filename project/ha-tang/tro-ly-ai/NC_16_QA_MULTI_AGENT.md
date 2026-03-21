# 🔬 NC DeepSearch #16: QA Độc Lập Trong Multi-Agent AI Development

> **Nguồn**: Google DeepSearch — 19/03/2026
> **Điểm**: 97/100 — Cực kỳ chất lượng, trực tiếp áp dụng cho Nodi POS
> **Chủ đề**: Context isolation, adversarial QA, project knowledge frameworks, Vietnamese NLP testing

---

## 🎯 7 Bài Học Chính Rút Ra

### 1. Context Window Isolation — QA KHÔNG được thấy lý do của Dev

> "QA agent must never access the development agent's scratchpad or chain-of-thought"

**Áp dụng**: Khi giao brief cho Agent QA → chỉ gửi:
- Code output (file changes)
- Brief gốc (specification)
- ❌ KHÔNG gửi dev agent's reasoning hoặc walkthrough

### 2. Asymmetric Tooling — QA chỉ có tools chẩn đoán

| Agent | Tools được phép | Tools CẤM |
|-------|----------------|-----------|
| Dev | Code gen, file edit, build | — |
| **QA** | **Linters, test runners, grep, log parsers** | **Code gen, file edit** |

**Áp dụng**: QA agent KHÔNG được sửa code, chỉ chạy test và báo lỗi.

### 3. Model Diversity — Dùng model khác cho QA

> "An architectural flaw that seems sound to one model's latent space is likely to be flagged by a different model"

**Áp dụng**: 
- Dev agents: Claude Opus 4.6 (Max)
- **QA agent: Claude Sonnet 4.5 hoặc Gemini** → latent space khác → tìm lỗi tốt hơn

### 4. Adversarial Prompting — QA là "kẻ phản biện"

> "Your primary objective is to PROVE that the solution is defective"

**Áp dụng**: System prompt QA agent:
```
Bạn là kiểm toán viên NGHIÊM KHẮC. Mục tiêu: CHỨNG MINH code có lỗi.
Bạn KHÔNG phải helper — bạn là AUDITOR. Đánh giá trên TIÊU CHUẨN, không trên cố gắng.
```

### 5. Cognitive Forcing — Bắt buộc giải trình từng pass/fail

> "QA agent must map developer's code to the original brief line-by-line"

**Áp dụng**: QA agent PHẢI output structured report:
```json
{
  "criterion": "Build 0 errors",
  "status": "PASS",
  "evidence": "npm run build → 0 errors in 18.24s",
  "file_path": "build output log"
}
```
Không có evidence → auto FAIL.

### 6. Markdown Agent Primitives — 5 loại file cho onboarding

| File | Mục đích |
|------|---------|
| `.spec.md` | Yêu cầu sản phẩm (task briefs) |
| `.instructions.md` | Coding standards (CLAUDE.md) |
| `.chatmode.md` | Persona + tools cho agent |
| `.context.md` | Kiến trúc hệ thống |
| `.memory.md` | Sổ bài học (TRO_LY_DIEU_HANH) |

→ Đã có tương đương: `briefs/`, `CLAUDE.md`, `QA_AGENT_BRIEF.md`, `ha-tang/README.md`, `TRO_LY_DIEU_HANH.md`

### 7. Metrics — 5 KPIs cho team nhỏ tự động hóa cao

| KPI | Ý nghĩa | Cách đo |
|-----|---------|---------|
| **Defect Density** | Lỗi/brief — brief nào rủi ro cao nhất? | Đếm bugs theo brief |
| **Flaky Test Rate** | % tests fail ngẫu nhiên (NLP non-deterministic) | Chạy 3 lần, so pass rates |
| **Defect Leakage** | % bugs lọt qua QA → production | User reports / QA bugs |
| **MTTD** | Thời gian phát hiện lỗi sau commit | Timestamp tracking |
| **Pass Rate Stability** | Trend pass/fail qua nhiều builds | Lưu kết quả theo ngày |

---

## 📋 Checklist Cải Thiện Từ NC

- [x] QA Agent Brief đã có 6 phases + scoring ✅
- [ ] Thêm adversarial prompting vào QA system prompt
- [ ] Thêm cognitive forcing (evidence-based pass/fail)
- [ ] Thêm 5 KPIs tracking vào báo cáo
- [ ] Thêm Vietnamese NLP edge cases (non-diacritical, dialect)
- [ ] Thêm offline-first sync testing (network drop simulation)
- [ ] Tách QA tools vs Dev tools trong AGENTS.md

---

## 🇻🇳 Vietnamese NLP Test Gaps Phát Hiện

| Category | Ví dụ | Đã test? |
|----------|-------|:--------:|
| Word Segmentation | "phân bón hóa học" = 1 entity | ✅ 1238 tests |
| Non-Diacritical | "phan bon" → "phân bón" | ✅ Phonetic tests |
| Regional Dialect | Bắc vs Nam terminology | ✅ farmer_dialect tests |
| Rapid Mobile Input | Typo + missing space | ✅ product_typo tests |
| Adversarial injection | SQL injection qua chatbot | ✅ edge_injection tests |

→ **Đã cover tốt 5/5 categories!** Test suite 1238 cases đang rất mạnh.

---

## 🏗️ Offline-First Sync Testing (Chưa có)

NC đề xuất test:
1. **Network drop giữa sync** → data integrity?
2. **Rapid connect/disconnect** → corruption?
3. **AES-256 encryption verify** → ciphertext length?
4. **Tauri IPC flood** → memory leak?
5. **async_queue reconciliation** → success rate?

→ Đây là vùng test MỚI cần bổ sung cho Nodi POS.
