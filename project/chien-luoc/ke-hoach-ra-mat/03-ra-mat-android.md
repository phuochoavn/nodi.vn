# 🤖 Chiến Lược Ra Mắt Android — Tauri v2 POS SaaS Nông Nghiệp

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 13/03/2026
> **Mục đích**: Đặc thù Android nông thôn VN, Play Store publishing, Billing bypass, Kiến trúc kỹ thuật, Testing

---

## 1. Đặc Thù Thị Trường Android Nông Thôn VN

### 1.1. Phân Mảnh Phần Cứng

| Chỉ số | Giá trị |
|--------|:-------:|
| **Android % nông thôn** | ~95%+ (iOS ~5%) |
| **Thiết bị phổ biến** | Samsung A04s/A05/A15, Redmi 12C/14C, Oppo A17k, Realme |
| **Chip xử lý** | MediaTek Helio G35/G85 (12nm, đơn nhân yếu) |
| **RAM** | 3-4GB |
| **ROM** | 32-64GB |
| **Android version** | 10+ |

> **Hệ quả**: Phải giải phóng tải trọng từ WebView → Rust native. JavaScript nặng = lag trên Helio G35.

### 1.2. Battery Optimization — "Kẻ thù" của App

- **Xiaomi (MIUI/HyperOS), Oppo (ColorOS), Realme**: Tự động kill background processes
- **Bắt buộc**: Gọi `Intent(Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS)` khi cài lần đầu
- **Data Saver Mode**: Kiểm tra `ConnectivityManager.isActiveNetworkMetered()` trước khi sync ảnh lớn

### 1.3. Dung Lượng Cài Đặt

| Framework | Kích thước |
|-----------|:---------:|
| Electron | >100MB |
| **Tauri v2** | **<15-20MB** |

> Tauri tận dụng Android System WebView có sẵn → không đóng gói Chromium.

### 1.4. Đa Người Dùng Trên 1 Thiết Bị

- Vợ/chồng/con luân phiên bán hàng trên 1 điện thoại
- **Giải pháp**: Persistent Store Session + **Quick PIN Login 4 số**
- PIN hash trong SQLite local → xác thực offline, không cần mạng
- Mỗi HĐ gắn tag nhân viên ID theo PIN đang active

---

## 2. Phân Phối & Billing Bypass

### 2.1. Sideload APK — Ưu/Nhược

| ✅ Ưu | ❌ Nhược |
|-------|---------|
| 0% phí, không review | Android 11+: "Unknown sources" cảnh báo đáng sợ |
| Hotfix trong ngày | Android 13: "Restricted Settings" làm xám quyền nhạy cảm |
| Tự do hoàn toàn | Không auto-update → phân mảnh phiên bản |
| | User nông thôn = KHÔNG thao tác nổi |

### 2.2. Google Play — Chính sách 2025-2026

- **Phí**: $25 (1 lần) → tài khoản Developer
- **Commission**: 15% cho $1M đầu/năm (subscription sau 1 năm), 30% cho giao dịch khác
- **Alternative Billing**: Chỉ ở EU, Mỹ, Nhật, Ấn Độ — **VN CHƯA ĐƯỢC**
- Ngay cả khi được: Google vẫn thu 10-27% "service fee"

### 2.3. ✅ Chiến Lược Consumption-Only + VietQR Bypass

```
┌─────────────────────────────────────────────────────┐
│  CHIẾN LƯỢC THANH TOÁN TỐI ƯU — 0% PHÍ            │
│                                                     │
│  1. App Android = FREE trên Play Store              │
│  2. Trong app: KHÔNG có nút mua/nâng cấp/VietQR    │
│  3. App chỉ có: Login + Chức năng POS               │
│  4. Khi hết hạn → thông báo trung lập:             │
│     "Truy cập admin.nodi.vn để quản lý gói"         │
│  5. Web Portal: VietQR → webhook xác nhận → mở khóa│
│  6. App Android gọi API kiểm tra → auto unlock      │
│                                                     │
│  → Hợp lệ 100% với Google Play Policy              │
│  → Giữ trọn vẹn 100% doanh thu                     │
└─────────────────────────────────────────────────────┘
```

> **Cơ sở pháp lý**: Google quy định "digital goods sử dụng được bên ngoài app (Web/Desktop) không bắt buộc GPB."

---

## 3. Google Play Console — Quy Trình

### 3.1. Xác Minh Danh Tính

| Loại tài khoản | Yêu cầu |
|:--------------:|---------|
| **Cá nhân** | CCCD/Hộ chiếu + sao kê ngân hàng/hóa đơn điện. **Closed Testing 20 user × 14 ngày** trước khi publish |
| **Tổ chức** | D-U-N-S number (miễn phí, 30 ngày xử lý). Tên doanh nghiệp phải **khớp từng chữ** với Play Console |

> **Solopreneur chưa có pháp nhân**: Dùng tài khoản Cá nhân → thuyết phục 20 đại lý beta test 14 ngày.

### 3.2. Data Safety & Target Audience

- **Khai báo**: Storage (SQLite), Camera (barcode), Bluetooth (máy in)
- **Privacy Policy URL**: Bắt buộc
- **Target Audience**: Chọn **"Not designed for children" + 18+** → tránh Families Policy
- **App Access**: Cung cấp tài khoản Demo cho reviewer Google

### 3.3. App Signing & Rollout

- **Định dạng**: `.aab` (Android App Bundle) — KHÔNG phải APK
- **Play App Signing**: Google giữ Release Key. Upload Key ở local → mất key có thể xin cấp lại
- **Staged Rollout**: Phát hành **10%** trước → Sentry giám sát 72h → nếu OK → 100%
- **ProGuard/R8**: Bật giúp giảm size, nhưng cẩn thận không strip hàm JNI của Rust

---

## 4. Kiến Trúc Kỹ Thuật Tauri v2 Android

### 4.1. IPC & Hiệu năng

```
Activity → WebView (Vue 3) ←IPC→ Rust Native ←JNI→ Kotlin
```

| Chỉ số | Tauri v2 | Electron |
|--------|:--------:|:--------:|
| **RAM khởi động** | 50-100MB | 200-500MB |
| **Chiến lược** | Batching IPC calls | — |

> **KHÔNG** gọi `invoke()` hàng nghìn lần. Rust query SQLite → đóng gói 1 mảng JSON lớn → trả 1 lần.

### 4.2. SQLite & Scoped Storage

- **Lưu trữ**: `/data/user/0/com.nodi.pos/databases/` (sandboxed, không cần xin permission)
- **Backup**: KHÔNG thể copy trực tiếp ra External Storage (Android 11+ Scoped Storage)
- **Giải pháp**: `tauri-plugin-sharesheet` → Rust copy DB → cache → Android Sharesheet → Google Drive/Zalo
- **Restore**: File Picker → read-once URI → nạp đè vào Internal Storage

### 4.3. Camera Quét Mã Vạch

| Phương pháp | Vấn đề | Khuyến nghị |
|-------------|--------|:-----------:|
| **html5-qrcode (JS)** | WebRTC + WASM = sụt frame, nóng máy, auto-focus tệ trên Helio G35 | ❌ |
| **tauri-plugin-barcode-scanner** | Native ML Kit/CameraX, xử lý bằng DSP phần cứng | ✅ |

```javascript
// Cấu hình tối ưu
scan({
  windowed: true,        // WebView trong suốt, camera layer bên dưới
  formats: ['EAN_13']    // Chỉ scan EAN-13 → nhanh hơn
})
```

### 4.4. In Bluetooth Nhiệt (ESC/POS)

**Vấn đề**: `tauri-plugin-blec` = BLE only, KHÔNG hỗ trợ Bluetooth Classic SPP.

**Giải pháp: Custom Kotlin Plugin**

```
Vue 3 → invoke() → Rust → run_mobile_plugin() → JNI →
    Kotlin @Command (Dispatchers.IO) →
    DantSu/ESCPOS-ThermalPrinter-Android →
    Socket SPP (UUID: 00001101-...) →
    Máy in nhiệt
```

| Bước | Chi tiết |
|:----:|---------|
| 1 | `plugin android init` → tạo module Kotlin |
| 2 | Nhúng `DantSu/ESCPOS-ThermalPrinter-Android` vào `build.gradle` |
| 3 | Kotlin `@Command` + `Dispatchers.IO` (KHÔNG chạy Main Thread → tránh ANR) |
| 4 | Hỗ trợ: UTF-8 tiếng Việt, bitmap logo, barcode, cash drawer |

### 4.5. Background Sync — WorkManager

| Phương pháp | Đánh giá |
|-------------|:--------:|
| AlarmManager | ❌ Wake CPU cưỡng bức → pin drain → bị kill |
| **WorkManager** | ✅ Guaranteed execution, sống sót qua kill/reboot |

```kotlin
// Kotlin — CoroutineWorker
Constraints.Builder()
    .setRequiredNetworkType(NetworkType.CONNECTED)
    .build()
// → Chỉ sync khi có mạng
// → Exponential backoff tự động nếu fail
// → Phải viết trong Custom Android Plugin của Tauri
```

### 4.6. Push Notifications — FCM

- Plugin: `tauri-plugin-remote-push`
- Cần **can thiệp thủ công** `src-tauri/gen/android/`:
  - Copy `google-services.json` từ Firebase Console
  - Cấu hình `senderId` trong `tauri.conf.json`
  - Priority = **High** cho sự kiện tài chính → xuyên qua Doze Mode

---

## 5. Testing Đám Mây (Zero-Device QA)

### 5.1. Nền tảng Miễn phí

| Công cụ | Free Tier | Loại test |
|---------|:---------:|:---------:|
| **Firebase Test Lab** | 10 ảo + 5 thật/ngày (30 phút) | Robo Test tự động, Memory Leak |
| **Samsung RTL** | Miễn phí hoàn toàn | Điều khiển Galaxy A thật qua streaming |

> Samsung RTL = kiểm tra UX/UI thực tế trên Galaxy A — dòng chiếm lĩnh nông thôn VN.

### 5.2. Crash Reporting

| Công cụ | JS Error | Rust Panic | Native Crash |
|---------|:--------:|:----------:|:------------:|
| Firebase Crashlytics | ❌ | ❌ | ✅ (Java/Kotlin only) |
| **Sentry (sentry-tauri)** | ✅ | ✅ | ✅ |

> **Sentry** vượt trội cho kiến trúc Tauri hybrid.

---

## 6. Chi Phí & Lộ Trình

### 6.1. Tổng Chi Phí

| Hạng mục | Chi phí |
|----------|:-------:|
| Google Play Developer | **$25** (1 lần) |
| D-U-N-S (nếu có pháp nhân) | $0 (mất 30 ngày) |
| Firebase (FCM + Test Lab) | $0 (Spark Plan) |
| Samsung RTL | $0 |
| Sentry | $0 (Developer Tier) |
| **TỔNG** | **$25** |

### 6.2. Lộ Trình 8 Tuần

| Tuần | Trọng tâm |
|:----:|-----------|
| **1** | Pháp lý: D-U-N-S + Play Console + Privacy Policy + Data Safety |
| **2** | Custom Kotlin Plugin: Bluetooth ESC/POS printer |
| **3** | Barcode scanner (native) + WorkManager sync |
| **4** | Mobile UI/UX: Touch targets + PIN multi-user + Battery Optimization bypass |
| **5** | Cloud QA: Firebase Test Lab (Robo) + Samsung RTL (manual) |
| **6** | Sentry integration + Keystore signing + Build AAB (<20MB) |
| **7** | Submit Play Store + Web Portal VietQR Billing Bypass |
| **8** | **Staged Rollout 10%** → Sentry 72h → OK → 100% |

---

## Tổng Kết

```
┌──────────────────────────────────────────────────────────┐
│        NODI POS — ANDROID LAUNCH STRATEGY               │
│                                                          │
│  📱 THỊ TRƯỜNG                                          │
│     Android ~95% nông thôn VN                            │
│     Helio G35/3GB RAM → Tauri (<100MB RAM) >> Electron   │
│     Dung lượng app <20MB (vs Electron >100MB)            │
│                                                          │
│  💰 BILLING                                              │
│     Consumption-Only model → 0% phí Google               │
│     VietQR thanh toán qua Web Portal (không trong app)   │
│                                                          │
│  🏪 PLAY STORE                                           │
│     $25 một lần + Closed Testing 20 user × 14 ngày       │
│     Staged Rollout 10% → 72h Sentry → 100%              │
│                                                          │
│  🔧 KỸ THUẬT                                            │
│     Camera: Native ML Kit (không JS)                     │
│     Printer: Custom Kotlin Plugin + SPP Bluetooth        │
│     Sync: WorkManager (guaranteed, survive kill/reboot)  │
│     Push: FCM (High priority cho giao dịch)              │
│                                                          │
│  🧪 TESTING                                             │
│     Firebase Test Lab + Samsung RTL = $0                 │
│     Sentry > Crashlytics cho Tauri hybrid                │
│                                                          │
│  💵 TỔNG CHI PHÍ: $25                                   │
│  📅 TIMELINE: 8 tuần từ Desktop → Production Android    │
└──────────────────────────────────────────────────────────┘
```

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 13/03/2026*
*Lưu trữ bởi Nodi POS Strategy Team*
