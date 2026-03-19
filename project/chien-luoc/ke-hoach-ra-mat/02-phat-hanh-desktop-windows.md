# 🖥️ Chiến Lược Phát Hành Desktop Windows — Tauri v2 POS SaaS

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 13/03/2026
> **Mục đích**: Code Signing, Auto-Update, Installer Experience, CDN, Phần cứng, Telemetry

---

## 1. Code Signing & Vượt Qua SmartScreen

### 1.1. OV vs EV Certificate

| Nhà cung cấp | Loại | Chi phí/năm | Đặc trưng | SmartScreen |
|--------------|:----:|:-----------:|-----------|:-----------:|
| **Sectigo/Comodo** | **OV** | **~$226-$287** | File mềm, CI/CD friendly, không giới hạn app ký | Cần tích lũy danh tiếng |
| DigiCert | OV | ~$386 | Uy tín cao, hỗ trợ nhanh | Cần tích lũy danh tiếng |
| Sectigo/Comodo | EV | ~$297-$345 | USB Token bắt buộc, khó CI/CD | ✅ Vượt qua ngay lập tức |
| DigiCert | EV | ~$507-$579 | Enterprise, xác thực cực nghiêm | ✅ Vượt qua ngay lập tức |

> **Khuyến nghị**: Sectigo OV (~$226/năm) — đủ tiêu chuẩn mã hóa, chi phí kiểm soát được cho startup.

### 1.2. Cấu hình Tauri v2

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "TRICH_XUAT_TU_CERTMGR",
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.sectigo.com"
    }
  }
}
```

> **Quan trọng**: `timestampUrl` bắt buộc — chứng minh mã ký tại thời điểm cert còn hiệu lực → .exe cũ vẫn hợp lệ khi cert hết hạn.

### 1.3. Chiến lược Giảm thiểu SmartScreen (3 mũi nhọn)

1. **Truyền thông trên trang tải**: GIF/video hướng dẫn "More info" → "Run anyway" — **bình thường hóa** cảnh báo
2. **Nộp WDSI**: Upload .exe lên Windows Defender Security Intelligence → yêu cầu phân tích thủ công
3. **Bảo toàn danh tiếng**: KHÔNG đổi tên publisher, KHÔNG đổi cert, hạn chế update nhỏ lẻ giai đoạn đầu

---

## 2. Auto-Update

### 2.1. Kiến trúc

```
GitHub Actions (tauri-action) → Build → Sign
    ↓
GitHub Releases (miễn phí, ổn định)
    ↓
latest.json (S3/CDN) → chứa version, URL, signature
    ↓
Tauri Plugin Updater → kiểm tra → tải → cài passive
```

### 2.2. Cấu hình

```json
{
  "plugins": {
    "updater": {
      "pubkey": "NOI_DUNG_KHOA_PUBLIC_KEY",
      "endpoints": ["https://cdn.nodi.vn/updates/latest.json"],
      "windows": {
        "installMode": "passive"
      }
    }
  }
}
```

> **`passive`**: Thanh tiến trình nhỏ, tự hoàn tất — KHÔNG yêu cầu click. Tránh `quiet` (user tưởng máy treo).

### 2.3. Canary Release (10% → 100%)

- `endpoints` trỏ về **Gateway API nội bộ** (không trỏ CDN trực tiếp)
- API nhận Shop ID → 10% nhóm thử nghiệm = trả 200 + JSON → 90% còn lại = trả 204 No Content

### 2.4. Rollback Mechanism

- Tauri KHÔNG có API rollback tự động
- **Giải pháp**: Ghi flag vào SQLite mỗi lần khởi động → nếu crash 3 lần trong 5 phút sau update → Rust ép gọi binary ổn định cũ nhất từ server → ghi đè

### 2.5. Delta Update

- Tauri v2 chưa hỗ trợ mặc định cho Windows
- **KHÔNG ảnh hưởng**: App chỉ ~15MB → tải toàn bộ mất vài giây

---

## 3. Installer Experience

### 3.1. NSIS > WiX (MSI)

| Tiêu chí | NSIS (.exe) | WiX (.msi) |
|----------|:-----------:|:----------:|
| **Per-User install** | ✅ Bỏ qua UAC hoàn toàn | ❌ Thường yêu cầu Admin |
| **Máy cũ** | ✅ Không cần VBSCRIPT | ❌ Phụ thuộc VBSCRIPT |
| **Nén** | ✅ Tốt hơn, ~15MB | 🟡 Kém hơn |
| **Hooks** | ✅ NSIS_HOOK_POSTINSTALL | ❌ Hạn chế |

> **Bắt buộc**: `"installMode": "perUser"` — cài vào `%LOCALAPPDATA%`, BỎ QUA UAC. Đại lý thường có DeepFreeze hoặc không nhớ password Admin.

### 3.2. Cấu hình NSIS tối ưu

```json
{
  "bundle": {
    "windows": {
      "nsis": {
        "installMode": "perUser",
        "displayLanguageSelector": false
      }
    }
  }
}
```

- **Desktop shortcut tự động**: Hook `NSIS_HOOK_POSTINSTALL` → tạo shortcut
- **Ẩn chọn ngôn ngữ**: Giảm 1 click không cần thiết

### 3.3. Antivirus False Positive (BKAV & Kaspersky)

**Quy trình Whitelisting:**

```
Tạo .exe → Quét VirusTotal → Cờ đỏ?
    ├─ Không → Sẵn sàng phát hành
    └─ Có → Gửi mẫu:
        ├─ BKAV: fpreport@bkav.com (kèm link VirusTotal + giải trình)
        ├─ Kaspersky: Threat Intelligence Portal → "Submit to reanalyze"
        └─ Chờ xác nhận → Phát hành
```

**Hướng dẫn user**: Thêm `%LOCALAPPDATA%\NodiPOS` vào "Exclusions" / "Trusted Applications" của antivirus → tăng tốc xử lý HĐ.

---

## 4. Phân phối & CDN

### 4.1. Zalo: KHÔNG gửi .exe trực tiếp

- Zalo **chặn hoàn toàn** tệp .exe (chính sách chống ransomware)
- Nén .rar có password = quá phức tạp cho user 50+ tuổi

> **Chiến lược duy nhất**: Gửi link `nodi.vn/tai-ve` qua Zalo → mở trình duyệt → tải .exe từ CDN.

### 4.2. BunnyCDN > Cloudflare (cho phân phối .exe)

| Tiêu chí | BunnyCDN | Cloudflare Free |
|----------|:--------:|:--------------:|
| **Latency SEA** | < 29ms | Không đảm bảo peak hours |
| **Resume Download** | ✅ Ổn định | 🟡 Không tối ưu cho binary |
| **Perma-Cache** | ✅ Chuyên biệt | ❌ Giới hạn |
| **Phù hợp** | ✅ Software distribution | 🟡 Web content |

> **Checksum**: KHÔNG cần cho khách nông thôn — Code Signing Certificate đã đảm bảo tính toàn vẹn.

---

## 5. Tương thích Phần cứng

### 5.1. WebView2

- **Có sẵn**: Windows 11 + Windows 10 ≥ 1803 (April 2018)
- **Không có**: Win 10 cũ, tắt Windows Update → ứng dụng Tauri KHÔNG chạy được

**Giải pháp: Bootstrapper tự động**

```json
{
  "bundle": {
    "windows": {
      "webviewInstallMode": "downloadBootstrapper"
    }
  }
}
```

→ Khi cài setup.exe → kiểm tra Registry → nếu thiếu WebView2 → tự tải + cài từ Microsoft → tiếp tục. Giữ setup ~15MB thay vì 127MB (offlineInstaller).

### 5.2. Tauri vs Electron — Hiệu năng

| Chỉ số | Tauri v2 | Electron |
|--------|:--------:|:--------:|
| **Dung lượng cài** | ~2.5-15MB | ~85MB |
| **Cold Boot** | ~2s | ~4s |
| **RAM** | **42%** (tương đối) | 100% |
| **Tiết kiệm** | **96% dung lượng, 58% RAM** | — |

> Máy Celeron/2GB RAM → Tauri chạy mượt, Electron = treo.

### 5.3. Máy in Nhiệt (ESC/POS)

- **Xprinter Q200, Sunmi, Epson TM-T82** — phổ biến tại VN
- **Rust libraries**: `thermoprint` hoặc `escpos-rs`
  - `rust_decimal` → không sai số dấu phẩy động
  - Codepage CP858 → in tiếng Việt có dấu
  - Barcode EAN-13 + Cash drawer kick
- **Luồng**: Vue 3 → JSON → `invoke()` IPC → Rust `#[tauri::command]` → ESC/POS bytes → USB → máy in
- **Latency**: Gần 0 (zero-latency)

---

## 6. Telemetry & Tuân thủ NĐ 13/2023

### 6.1. Sentry cho Tauri

- Plugin `sentry-tauri` → bắt lỗi cả **JS (Vue 3)** và **Rust panics (Minidumps)**
- Breadcrumbs đồng bộ Frontend + Backend → tái hiện chuỗi thao tác dẫn đến crash
- **Free tier**: 5,000 events/tháng

### 6.2. Tuân thủ NĐ 13/2023/NĐ-CP

**Nguyên tắc 1 — Ẩn danh hóa:**
- `sendDefaultPii = false` → KHÔNG gửi IP, email, tài khoản
- Hash mọi ID thành UUID không thể dịch ngược

**Nguyên tắc 2 — Chấp thuận chủ động:**
- First Launch: Popup tiếng Việt: "Thu thập dữ liệu lỗi ẩn danh. KHÔNG thu thập nội dung kinh doanh."
- Chỉ khi user nhấp "Đồng ý" → khởi tạo Sentry module

### 6.3. Giám sát SQLite phình to

```rust
#[tauri::command]
fn check_db_size(app_handle: AppHandle) -> Result<u64, String> {
    let app_dir = app_handle.path().app_data_dir()
        .ok_or("Không tìm thấy đường dẫn")?;
    let db_path = app_dir.join("pos_database.db");
    match std::fs::metadata(&db_path) {
        Ok(meta) => Ok(meta.len()),
        Err(e) => Err(e.to_string()),
    }
}
```

> Ngưỡng cảnh báo: >500MB → thông báo bảo trì + gửi tín hiệu cầu cứu kỹ thuật.

---

## Tổng Kết

```
┌──────────────────────────────────────────────────────────┐
│     NODI POS — DESKTOP WINDOWS LAUNCH STRATEGY          │
│                                                          │
│  🔐 CODE SIGNING                                        │
│     Sectigo OV ~$226/năm + timestampUrl                 │
│     SmartScreen: GIF hướng dẫn + nộp WDSI + giữ cert    │
│                                                          │
│  🔄 AUTO-UPDATE                                         │
│     GitHub Releases + latest.json + CDN                  │
│     installMode: "passive" + Canary 10%→100%            │
│     Rollback: flag SQLite 3 crash → ghi đè binary cũ   │
│                                                          │
│  📦 INSTALLER                                           │
│     NSIS + perUser (bỏ qua UAC) + Desktop shortcut     │
│     Whitelist BKAV/Kaspersky TRƯỚC khi phát hành       │
│                                                          │
│  🌐 CDN                                                │
│     BunnyCDN (resume + low latency SEA)                 │
│     Zalo: gửi link nodi.vn/tai-ve, KHÔNG gửi .exe      │
│                                                          │
│  🖨️ PHẦN CỨNG                                          │
│     WebView2 downloadBootstrapper (auto-install)         │
│     ESC/POS qua Rust IPC (zero-latency)                │
│     Tauri: 96% nhẹ hơn Electron, 58% ít RAM hơn       │
│                                                          │
│  📊 TELEMETRY                                           │
│     Sentry (JS + Rust) + NĐ 13 compliance              │
│     Ẩn danh + Explicit Consent popup                    │
└──────────────────────────────────────────────────────────┘
```

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 13/03/2026*
*Lưu trữ bởi Nodi POS Strategy Team*
