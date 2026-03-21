# 🔐 Bảo Mật — Security Architecture

> License, auth, RBAC, encryption, compliance NĐ 13/2023

---

## 🏆 Tinh Hoa Cần Làm — Tổng hợp từ Nghiên Cứu

### 🔴 Ưu tiên Tối Cao (trước khi tung app)

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 1 | **SQLCipher AES-256** | Mã hóa toàn bộ .db, PBKDF2 64K+ rounds, HMAC per page | Lớn |
| 2 | **Khóa → DPAPI / Android TEE** | Không lưu key trên filesystem. Win: Credential Manager, Android: Keystore + TEE | Lớn |
| 3 | **Audit Triggers** | SQLite AFTER UPDATE/DELETE → `audit_log` tự động, JSON delta | TB |
| 4 | **Hash Chain** | `H(n) = SHA-256(Data + H(n-1))` → sổ cái bất biến | TB |
| 5 | **RBAC xác thực ở Rust** | Không chỉ ẩn UI, phải check quyền ở backend | TB |

### 🟠 Ưu tiên Cao (gần tung app)

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 6 | **License Ed25519 + HW Fingerprint** | JWT ký bằng Private Key, gắn DNA thiết bị (CPUID+GUID) | Lớn |
| 7 | **TLS Certificate Pinning** | `rustls` + hash hardcoded, bỏ qua OS Trust Store | TB |
| 8 | **JWT short-lived** | Access Token 15-30 min, Refresh Token, LƯU trong Rust RAM only | TB |
| 9 | **Consent UI (NĐ 13)** | Màn hình đồng ý khi tạo hồ sơ KH, lưu `consent_flag` + timestamp | Nhỏ |
| 10 | **External Anchoring** | Push Root Hash audit lên VPS, detect tampering | TB |

### 🟡 Ưu tiên Trung bình

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 11 | **Soft-delete + Anonymize** | Quyền xóa dữ liệu (NĐ 13): ẩn danh hóa thay DELETE | Nhỏ |
| 12 | **Tauri Capabilities lock** | Chỉ cho phép commands cần thiết, Context Isolation BẬT | Nhỏ |
| 13 | **Tamper detection alert** | Rust detect: decrypt fail liên tục → push notification chủ đại lý | TB |
| 14 | **Biometric unlock** | `tauri-plugin-biometry` → vân tay/mặt để giải khóa DB | TB |

---

### 💎 5 Nguyên tắc Bảo mật

1. **"Zero Trust tại app level"** — WebView = không tin cậy, mọi thứ nhạy cảm → Rust
2. **"Khóa gắn liền HW, không gắn filesystem"** — DPAPI/TEE, key chỉ tồn tại trong RAM
3. **"Sổ cái bất biến > App logging"** — SQLite Triggers + Hash Chain, không phụ thuộc code
4. **"Tệp .db ngoài thiết bị = white noise"** — SQLCipher + HW key = vô giá trị nếu bị sao chép
5. **"Privacy by Design"** — NĐ 13/2023 là bộ giáp pháp lý, không phải gánh nặng

---

## Nghiên Cứu Đã Hoàn Thành (Google DeepSearch — 18/03/2026)

| # | File | Nội dung | Điểm |
|:-:|------|----------|:----:|
| 01 | `nghien-cuu-01-bao-mat-offline-first-pos.md` | SQLCipher, License Ed25519, RBAC, TLS Pinning, NĐ 13/2023 | **96** |

---

## Hiện trạng Nodi POS

| Tính năng | Hiện tại | Mục tiêu |
|----------|:-------:|:-------:|
| Mã hóa SQLite | ❌ | SQLCipher AES-256 |
| Quản lý khóa | ❌ | DPAPI / Android TEE |
| RBAC | ✅ Owner/Manager/Staff (UI) | + xác thực Rust backend |
| Audit Log | ❌ | Triggers + Hash Chain |
| License | ❌ | Ed25519 JWT + HW Fingerprint |
| TLS Pinning | ❌ | rustls + cert hash |
| NĐ 13/2023 | ❌ | Consent UI + Soft-delete |
