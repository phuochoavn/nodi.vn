# 🍎 Chiến Lược Ra Mắt iOS (Không Có Mac) — Tauri v2 POS SaaS

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 13/03/2026
> **Mục đích**: Cloud macOS CI/CD, Apple Developer Program, App Store Review, Apple Tax bypass, Giới hạn kỹ thuật iOS, Plan B

---

## 1. Giải Pháp Cloud macOS (CI/CD)

### 1.1. So Sánh Chi Phí

| Giải pháp | Chi phí | Debug | Đánh giá |
|-----------|:-------:|:-----:|:--------:|
| **Codemagic** (pay-as-you-go) | $0.095/phút (M2) | Tự động CI/CD | ✅ Tối ưu cho CI/CD |
| GitHub Actions (macos_xl) | $0.102/phút (M2 Pro) | Tự động CI/CD | 🟡 Đắt hơn, 10x multiplier |
| MacinCloud | ~$25-30/tháng | Remote Desktop | ⚠️ Lag, hạn chế root |
| AWS EC2 Mac | ~$1.23/giờ (~$930/tháng) | Bare metal | ❌ Quá đắt cho solopreneur |
| **Mac Mini M1/M2 cũ** | **$400-600 (1 lần)** | **Gỡ lỗi vật lý tối đa** | ✅ **KHUYẾN NGHỊ** |

> **Cold build Tauri v2**: 25-35 phút → ~$3/lần trên CI/CD. Gỡ lỗi qua CI/CD = vòng lặp chậm chạp 30 phút/lần.

### 1.2. Chiến Lược 2 Giai Đoạn (Khuyến Nghị)

```
┌─────────────────────────────────────────────┐
│  GĐ 1: LẬP TRÌNH & GỠ LỖI                │
│  Mac Mini M1/M2 cũ ($400-600 một lần)      │
│  → Xcode, real-time debugger                │
│  → Kết nối iPhone qua cáp                   │
│  → Test Bluetooth printer + camera trực tiếp│
│                                              │
│  GĐ 2: PHÂN PHỐI & BẢO TRÌ                │
│  Codemagic pay-as-you-go ($0.095/phút)      │
│  → Tự động build + sign + push App Store    │
│  → ~$150-200/năm cho updates định kỳ        │
└─────────────────────────────────────────────┘
```

---

## 2. Apple Developer Program — VN

### 2.1. Cá Nhân vs Tổ Chức

| Tiêu chí | Cá nhân | Tổ chức |
|----------|:-------:|:-------:|
| **Phí** | $99/năm | $99/năm |
| **Hiển thị App Store** | Tên pháp lý cá nhân | Tên công ty/thương hiệu |
| **Yêu cầu** | CCCD + 2FA | Pháp nhân + D-U-N-S |
| **Phù hợp** | MVP nội bộ | ✅ B2B chuyên nghiệp |

> **B2B SaaS**: Đăng ký **Tổ chức** để hiện tên thương hiệu. Không chấp nhận Hộ kinh doanh cá thể, chỉ TNHH/Cổ phần.

### 2.2. Quy Trình D-U-N-S

1. Chuẩn bị Giấy ĐKKD (bản dịch Anh công chứng)
2. Tra cứu D-U-N-S qua cổng Apple → nộp thông tin cho D&B
3. Email đăng ký = tên miền công ty (admin@nodi.vn)
4. D&B xử lý: ~5 ngày (có thể 2-3 tuần nếu xác minh qua ĐT)
5. Chờ thêm 2 ngày để D&B đồng bộ với Apple
6. Nộp Apple Developer Program

### 2.3. Thanh Toán $99 — Vấn Đề VN

**Vì sao thẻ VN bị từ chối?**
- Apple xử lý ở Mỹ → ngân hàng VN chặn giao dịch xuyên biên giới
- Thuật toán chống gian lận nhầm lẫn với BIN attack

**Chiến lược vượt qua:**
1. Dư ≥$110-120 trong thẻ (phí chuyển đổi ngoại tệ)
2. Region Apple ID = Việt Nam, Billing Address khớp thẻ ngân hàng
3. **KHÔNG nhấp thanh toán lặp lại** → bị blacklist → gọi hotline ngân hàng mở khóa
4. Dùng Edge/Firefox Incognito trên desktop (tránh Safari cache lỗi)

---

## 3. App Store Review — Rủi Ro & Vượt Ải

### 3.1. Guideline 4.2 — Chức Năng Tối Thiểu

> Apple từ chối app "web đóng gói". Tauri dùng WKWebView → rủi ro cao.

**Cách vượt qua:**
- ✅ Loại bỏ bounce scrolling, bôi đen text
- ✅ 60fps, nút phản hồi tức thì
- ✅ Bundle ~10MB, RAM ~50MB, startup <1 giây (nhờ Rust)
- ✅ **Offline-first**: Tắt Wi-Fi vẫn hoạt động bình thường (SQLite)
- ❌ KHÔNG hiện "No Internet Connection" của trình duyệt

### 3.2. Guideline 3.1.1 — In-App Purchase (Apple Tax)

| Chiến lược | Hợp lệ VN? | Chi tiết |
|------------|:----------:|---------|
| IAP 30% | ✅ | Đúng luật nhưng đắt |
| **IAP 15%** (Small Business) | ✅ | Doanh thu <$1M/năm |
| External Payment Link | ❌ | Chỉ US/EU/JP/IN |
| Reader App (3.1.3a) | ❌ | Chỉ cho Netflix/Spotify, **KHÔNG** cho POS |
| **Multiplatform Service** (3.1.3b) | ✅ | **KHUYẾN NGHỊ** |

### 3.3. ✅ Chiến Lược Multiplatform Service (0% Apple Tax)

```
┌──────────────────────────────────────────────┐
│  APP iOS = TRÌNH ĐỌC DỊCH VỤ               │
│                                              │
│  ✅ Chỉ có màn hình Login                    │
│  ✅ Chức năng POS sau khi đăng nhập          │
│  ❌ KHÔNG có nút Nâng cấp/Mua gói           │
│  ❌ KHÔNG có bảng giá                        │
│  ❌ KHÔNG có link, QR, text hướng dẫn mua    │
│                                              │
│  Khi hết hạn → thông báo TRUNG LẬP:         │
│  "Tài khoản không có quyền truy cập.        │
│   Liên hệ quản trị viên."                   │
│                                              │
│  Thanh toán: Web Portal + VietQR = 0% phí   │
└──────────────────────────────────────────────┘
```

### 3.4. Nếu Chấp Nhận IAP — Thuế Chồng Thuế (VN)

```
Giao dịch $100 qua IAP tại VN:
  - Apple commission (Small Business): -15%  → $85
  - FCT 5% trên hoa hồng Apple              → trừ thêm
  - PIT/CIT 2-5% trên tổng giao dịch        → trừ thêm
  ─────────────────────────────────────────
  Thực nhận: << $85 (ước tính ~$78-80)
  
  So sánh VietQR: $100 → nhận $100 (0% phí)
```

### 3.5. Metadata Chống Reject

- Screenshots: Luồng thao tác thực tế, KHÔNG dữ liệu giả
- Privacy Policy URL: Hoạt động, mô tả chính xác thu thập dữ liệu
- App Access: Tài khoản Demo cho reviewer Apple

---

## 4. Giới Hạn Kỹ Thuật iOS vs Android

### 4.1. Bluetooth Printer

| | Android | iOS |
|--|:-------:|:---:|
| **Classic BT (SPP)** | ✅ Tự do | ❌ Cần MFi ($300+ máy in) |
| **BLE** | ✅ | ✅ Dùng CoreBluetooth |
| **Plugin** | Custom Kotlin SPP | `tauri-plugin-blec` (btleplug) |

**iOS giải pháp: BLE**
- Máy in BLE giá rẻ (Xprinter, Zjiang có module BLE)
- Plugin: `tauri-plugin-blec` hoặc `tauri-plugin-bluetooth`
- **Bắt buộc**: `Info.plist` → `NSBluetoothAlwaysUsageDescription`
- **Lưu ý**: BLE bandwidth thấp (20-512 bytes/gói) → phải chunk ESC/POS data → queue cẩn thận

### 4.2. Camera Barcode

- Plugin: `tauri-plugin-barcode-scanner` (AVFoundation native)
- Rust ≥ 1.77.2
- Camera chạy bên dưới WKWebView → CSS transparent nền
- **Bắt buộc**: `Info.plist` → `NSCameraUsageDescription`

### 4.3. Background Sync — iOS Draconian

| | Android | iOS |
|--|:-------:|:---:|
| **Background** | WorkManager (thoải mái) | Suspended khi minimize |
| **API** | Service / WorkManager | BGAppRefreshTask (<30s) |
| **Sync lớn** | Bất kỳ lúc nào | BGProcessingTask (chỉ khi sạc + WiFi) |
| **Survive kill** | ✅ WorkManager | ✅ BGAppRefreshTask (hệ thống lên lịch) |

> iOS chỉ cho ~30 giây sync nền. Code Rust phải lưu trạng thái nếu chưa xong.

### 4.4. SQLite & iCloud

- Sandbox: Mỗi app chỉ truy cập Container riêng
- App Group: Cần nếu muốn chia sẻ DB giữa app + Widget
- **QUAN TRỌNG**: Gắn cờ `NSURLIsExcludedFromBackupKey` lên file .sqlite
  → Tránh iCloud backup DB dung lượng lớn → Apple reject

### 4.5. File Export & Push

- Export báo cáo: `tauri-plugin-sharesheet` → iOS Share Sheet
- Push: APNs → cần file .p8 từ Apple Developer Portal → FCM

---

## 5. Chi Phí Năm 1

| Hạng mục | Chi phí |
|----------|:-------:|
| Apple Developer Program | **$99/năm** |
| Mac Mini M1/M2 cũ | **$400-600** (1 lần) |
| Codemagic CI/CD | **$150-200/năm** |
| Backend (VPS, domain, SSL) | ~$150/năm |
| **TỔNG NĂM 1** | **$800-1,050** |

> ROI: iOS = 41.3% thị phần VN. Dù nông thôn chỉ 5-10%, nhưng LTV người dùng iOS CAO HƠN Android.

---

## 6. Lộ Trình 8 Tuần

| Tuần | Trọng tâm |
|:----:|-----------|
| **1-2** | Apple Developer ($99) + D-U-N-S + Mua Mac Mini cũ |
| **3-4** | Port Tauri → Mac. Test Simulator. Sửa IPC bridge + Vue 3 UI Cupertino |
| **5-6** | BLE printer (chunk ESC/POS). Camera barcode. App Group + SQLite backup |
| **7-8** | Xóa UI nâng cấp/giá (Guideline 3.1.1). Build .ipa + Screenshots + App Store Connect + Codemagic |

---

## 7. Kịch Bản Dự Phòng (Plan B)

| Plan | Ưu | Nhược | Phù hợp khi |
|:----:|-----|------|------------|
| **B1: PWA (Safari)** | 0 phí, instant update, Web Push (iOS 16.4+) | ❌ **Không có Web Bluetooth** → không in được | Không cần máy in |
| **B2: Capacitor/Ionic** | Hệ sinh thái plugin lớn, ổn định | Mất lõi Rust → JS xử lý DB chậm | Tauri IPC lỗi quá nhiều trên iOS |
| **B3: Flutter/RN** | Native hoàn toàn | Viết lại từ đầu (6-12 tháng) | Dự án dài hạn, có budget |
| **B4: Bỏ iOS** | 0 chi phí | Mất 41% thị trường VN + khách LTV cao nhất | Tuyệt vọng |

---

## Tổng Kết

```
┌──────────────────────────────────────────────────────────┐
│        NODI POS — iOS LAUNCH STRATEGY                   │
│                                                          │
│  🖥️ BUILD                                               │
│     Mac Mini M1/M2 cũ ($400-600) + Codemagic CI/CD     │
│     Cold build: 25-35 phút → ~$3/lần trên cloud        │
│                                                          │
│  🍎 APPLE DEVELOPER                                     │
│     $99/năm + D-U-N-S (pháp nhân)                       │
│     Thẻ VN bị reject → gọi ngân hàng mở khóa           │
│                                                          │
│  💰 BILLING                                              │
│     Multiplatform Service → 0% Apple Tax                │
│     App chỉ có Login + POS, KHÔNG UI mua hàng          │
│     VietQR thanh toán qua Web Portal                    │
│                                                          │
│  🔧 KỸ THUẬT                                            │
│     Printer: BLE (không cần MFi) + chunk ESC/POS       │
│     Camera: AVFoundation native                         │
│     Sync: BGAppRefreshTask (<30s) — KHÔNG như Android   │
│     SQLite: Exclude từ iCloud backup                    │
│                                                          │
│  💵 TỔNG NĂM 1: $800-1,050                             │
│  📅 TIMELINE: 8 tuần (sau khi Desktop + Android ổn)    │
│                                                          │
│  ⚠️ CHIẾN LƯỢC: Ra Desktop + Android TRƯỚC             │
│     Ổn định → mới tấn công iOS                          │
└──────────────────────────────────────────────────────────┘
```

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 13/03/2026*
*Lưu trữ bởi Nodi POS Strategy Team*
