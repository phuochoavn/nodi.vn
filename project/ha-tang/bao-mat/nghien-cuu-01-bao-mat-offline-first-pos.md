# 📚 NC Bảo Mật #01: Khung Phòng Thủ Toàn Diện cho Nodi POS Offline-First

> **Nguồn**: Google DeepSearch
> **Ngày**: 2026-03-18
> **Prompt**: Cải tiến 4 trụ cột (Application Security Architect perspective)
> **Liên quan**: SQLCipher, License Ed25519, RBAC, TLS Pinning, NĐ 13/2023

## Tóm tắt

Blueprint bảo mật đa lớp (Defense in Depth) cho POS offline-first: mã hóa SQLite bằng SQLCipher AES-256, quản lý khóa qua Windows DPAPI / Android TEE, license offline Ed25519 + Hardware Fingerprinting, RBAC 3 cấp, Audit Log bất biến (Hash Chain), TLS Certificate Pinning, tuân thủ NĐ 13/2023.

---

## 1. Case Study: Bảo mật POS toàn cầu

| POS | Bài học | Áp dụng Nodi |
|-----|--------|-------------|
| **Toast** | Offline 40s auto-switch, LAN routing, Transaction Queue mã hóa, 24-72h sync window | Queue công nợ mã hóa, sync khi có Wi-Fi |
| **Square** | HSM hardware encrypt tại point-of-swipe, Double Encryption (HW + SW), Write-only audit | Audit Log mã hóa bằng Public Key VPS = chỉ ghi, không đọc cục bộ |
| **Lightspeed** | Tách luồng tài chính ↔ bán hàng, Cloud = Source of Truth, Tokenization | Conflict Resolution đa thiết bị, đối soát trên VPS |

---

## 2. Kiến trúc Tauri v2 — Ranh giới Tin cậy

```
┌─────────────────────────────────┐
│ WebView (Vue 3) — KHÔNG TIN CẬY │
│ - Hiển thị UI                    │
│ - Gửi Commands qua IPC           │
│ ❌ KHÔNG giữ token, key, logic   │
└──────────┬──────────────────────┘
           │ IPC (Capabilities kiểm soát)
           ▼
┌─────────────────────────────────┐
│ Rust Backend — PHÁO ĐÀI         │
│ ✅ Khóa mã hóa SQLCipher         │
│ ✅ JWT Access/Refresh Token       │
│ ✅ License validation Ed25519     │
│ ✅ Hardware Fingerprint SHA-256   │
│ ✅ Audit Hash Chain               │
│ ✅ TLS Pinning (rustls)           │
└─────────────────────────────────┘
```

### Quy tắc sắt
- ✅ Context Isolation: BẬT
- ✅ Capabilities: chỉ cho phép commands cần thiết
- ❌ WebView KHÔNG truy cập filesystem, shell, token
- ❌ KHÔNG hardcode API key trong frontend

---

## 3. Mã hóa SQLite — SQLCipher vs SEE

| Tiêu chí | **SQLCipher** ✅ | SQLite SEE |
|---------|:----------------:|:----------:|
| Giấy phép | BSD (free) / $999/năm | $2,000 + $3,500/năm |
| Thuật toán | AES-256 (CBC/GCM) | AES-256/128 (OFB/CCM), RC4 |
| Chống brute-force | ✅ **PBKDF2** 64K+ rounds + salt | ❌ Không có mặc định |
| Chống giả mạo | ✅ **HMAC** per page | Cần CCM/GCM riêng |
| Secure Delete | ✅ Mặc định | ❌ Cần bật thủ công |
| Rust ecosystem | ✅ `rusqlite` + `bundled-sqlcipher` | ❌ Tự viết C binding |
| **Kết luận** | **Trụ cột bắt buộc** | Không phù hợp SME |

---

## 4. Quản lý Khóa — Hardware-Backed

```
Lần khởi chạy đầu:
  Rust → Sinh khóa AES-256 ngẫu nhiên
       → Windows: DPAPI (Credential Manager)
       → Android: TEE / Secure Element (Keystore)
       → Khóa KHÔNG BAO GIỜ nằm trên filesystem

Mỗi lần mở app:
  Rust → Yêu cầu OS giải phóng khóa (hoặc qua biometrics)
       → Nạp vào RAM → PRAGMA key cho SQLCipher
       → App đóng → Xóa sạch khỏi RAM
```

### Kịch bản mất cắp
- Cắm USB sao chép .db → **Vô nghĩa** (không có khóa)
- Thêm vân tay mới trên Android → **TEE thu hồi mọi khóa cũ**
- Reverse engineer binary → Rust + LLVM obfuscation = cực khó

---

## 5. License Offline — Ed25519 + Hardware Fingerprint

### Luồng cấp phép

```
1. Đại lý mua → VPS tạo JWT chứa:
   - Thời hạn, tính năng
   - Hardware DNA (SHA-256 của CPUID + MachineGUID + SMBIOS)
   - Ký bằng Private Key (Ed25519)

2. JWT → lưu cục bộ trong app

3. Mỗi lần khởi động offline:
   Rust → Verify JWT bằng Public Key (hardcoded)
        → Thu thập lại HW fingerprint → SHA-256
        → So sánh với DNA trong JWT
        → Sai bất kỳ thứ gì → KHÓA APP
```

### Chống nhân bản
- Copy JWT sang máy khác → **DNA không khớp → khóa**
- Sửa ngày hết hạn → **Chữ ký hỏng → khóa**
- Decompile Rust binary → **LLVM CFO + no GC → cực khó patch**

---

## 6. RBAC — 3 Cấp phân quyền

| Quyền | Owner | Manager | Staff |
|-------|:-----:|:-------:|:-----:|
| Cấu hình giá bán | ✅ | ❌ | ❌ |
| Sửa công nợ | ✅ | ❌ | ❌ |
| Phê duyệt bán nợ vượt hạn mức | ✅ | ✅ | ❌ |
| Xem báo cáo | ✅ | ✅ | ❌ |
| Tạo đơn hàng | ✅ | ✅ | ✅ |
| Thu tiền mặt | ✅ | ✅ | ✅ |
| Xóa vĩnh viễn hóa đơn | ✅ | ❌ | ❌ |
| Xem tổng dư nợ | ✅ | ✅ | ❌ |

> ⚠️ RBAC phải xác thực ở **cấp database** (Rust), không chỉ ẩn UI

---

## 7. Audit Log — Sổ cái Bất biến

### Tầng 1: SQLite Triggers (tự động)

```sql
CREATE TRIGGER audit_debts_update
AFTER UPDATE ON debts
BEGIN
  INSERT INTO audit_log (timestamp, actor_id, table_name, action, record_deltas)
  VALUES (datetime('now'), OLD.updated_by, 'debts', 'UPDATE',
    json_object(
      CASE WHEN OLD.amount IS NOT NEW.amount
        THEN 'amount', json_object('old', OLD.amount, 'new', NEW.amount)
      END
    ));
END;
```

### Tầng 2: Hash Chain (chống xóa/sửa log)

```
H(n) = SHA-256( CanonicalJSON(Data_n) + H(n-1) + Timestamp )

→ Sửa bản ghi N → H(N) thay đổi → H(N+1) hỏng → toàn bộ chuỗi đổ
→ Tính lại toàn bộ? → External Anchor trên VPS phát hiện
```

### Tầng 3: External Anchoring
- Khi online → push Root Hash lên VPS
- VPS hash ≠ Local hash → **CẢNH BÁO AN NINH**

---

## 8. Bảo mật Đồng bộ — TLS Pinning + JWT

### TLS Certificate Pinning (rustls)

```
❌ Không tin Trust Store của OS (dễ bị cài CA giả)
✅ Chỉ chấp nhận certificate hash hardcoded trong Rust
✅ Dùng rustls (thuần Rust, không phụ thuộc OS)
→ Proxy/Fiddler/MITM → NGẮT KẾT NỐI ngay
```

### JWT Token lifecycle

| Token | Thời hạn | Lưu ở đâu |
|-------|:--------:|:----------:|
| Access Token | 15-30 phút | Rust RAM only |
| Refresh Token | Dài hơn | Rust RAM only |
| ❌ **KHÔNG BAO GIỜ** | — | LocalStorage/SessionStorage |

---

## 9. Tuân thủ NĐ 13/2023/NĐ-CP

| Điều | Nghĩa vụ | Giải pháp Nodi POS | Rủi ro |
|:----:|---------|-------------------|:------:|
| 11, 13 | Đồng ý rõ ràng + quyền được biết | UI consent screen + lưu `consent_flag` + timestamp | 🔴 |
| 2.4, 26 | Bảo vệ dữ liệu nhạy cảm (giao dịch, công nợ) | SQLCipher + RBAC + TLS Pinning | 🔴 |
| 14, 15 | Quyền trích xuất + xóa dữ liệu | Export PDF/CSV + Soft-delete (ẩn danh hóa) | 🟡 |
| 38 | Nhật ký hệ thống | Audit Triggers + Hash Chain + External Anchor | 🔴 |
| 23, 30 | Thông báo vi phạm 72h | Rust telemetry: detect decrypt fail + push alert | 🟡 |

---

## 10. Bảng Đe dọa → Giải pháp → Ưu tiên

| Mối đe dọa | Giải pháp | Ưu tiên |
|-----------|---------|:-------:|
| Cắm USB sao chép .db | SQLCipher + HW key management | **🔴 TỐI CAO** |
| Sửa công nợ gian lận | Audit Triggers + Hash Chain + VPS Anchor | **🔴 TỐI CAO** |
| Bẻ khóa/chia sẻ chùa | Ed25519 JWT + Hardware Fingerprint | 🟠 CAO |
| MITM trên Wi-Fi | rustls + Certificate Pinning + JWT short-lived | 🟠 CAO |
| Vi phạm NĐ 13/2023 | Privacy by Design: consent, RBAC, soft-delete, DPIA | 🟠 CAO |
| XSS chiếm token | Tauri Capabilities + Context Isolation + token in Rust only | 🟡 TB |

---

## Nguồn tham khảo

- SQLCipher (Zetetic) documentation
- Tauri v2 Security model & Capabilities
- Toast POS Offline Mode & Background Processing
- Square POS Double Encryption & HSM
- Lightspeed POS multi-mode sync
- NĐ 13/2023/NĐ-CP (Bảo vệ Dữ liệu Cá nhân VN)
- Ed25519 digital signature algorithm
- PBKDF2 & HMAC standards
- rustls & reqwest TLS documentation
- Android Keystore & TEE architecture
- Windows DPAPI & Credential Manager
