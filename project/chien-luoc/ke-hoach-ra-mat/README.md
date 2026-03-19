# 🚀 Kế Hoạch Ra Mắt — Nodi POS

> **Tổng hợp tinh hoa từ 5 tài liệu nghiên cứu chuyên sâu**
> Cập nhật: 13/03/2026

---

## Thứ Tự Ra Mắt

```
 ① Desktop Windows  →  ② Android (Play Store)  →  ③ iOS (App Store)
      RA MẮT ĐẦU             NGAY SAU ĐÓ              KHI CÓ DOANH THU
```

---

## ① Desktop Windows — Nền Tảng Chính

📄 Chi tiết: [02-phat-hanh-desktop-windows.md](./02-phat-hanh-desktop-windows.md)

| Hạng mục | Giải pháp | Chi phí |
|----------|-----------|:-------:|
| **Code Signing** | Sectigo OV (SmartScreen cần tích lũy danh tiếng) | ~$226/năm |
| **SmartScreen** | GIF hướng dẫn "More info → Run anyway" + nộp WDSI | $0 |
| **Installer** | NSIS + `perUser` (bỏ qua UAC, quan trọng với DeepFreeze) | $0 |
| **Auto-Update** | Tauri plugin-updater + `passive` mode + Canary 10%→100% | $0 |
| **CDN** | BunnyCDN (resume download, low latency SEA) | Rẻ |
| **Phân phối Zalo** | Gửi link `nodi.vn/tai-ve`, **KHÔNG gửi .exe** (Zalo chặn) | $0 |
| **WebView2** | `downloadBootstrapper` auto-install nếu thiếu | $0 |
| **Antivirus** | Whitelist BKAV/Kaspersky TRƯỚC khi phát hành | $0 |
| **Telemetry** | Sentry (JS + Rust) + NĐ 13 compliance + consent popup | $0 |

> **Hiệu năng**: Tauri giảm 96% dung lượng, 58% RAM so với Electron. Máy Celeron/2GB RAM chạy mượt.

---

## ② Android — Mở Rộng Thị Trường

📄 Chi tiết: [03-ra-mat-android.md](./03-ra-mat-android.md)

| Hạng mục | Giải pháp | Chi phí |
|----------|-----------|:-------:|
| **Play Store** | Google Play Developer (1 lần) | **$25** |
| **Billing** | **Consumption-Only + VietQR Web Portal = 0% phí Google** | $0 |
| **Printer** | Custom Kotlin Plugin + DantSu ESC/POS (Bluetooth SPP) | $0 |
| **Barcode** | `tauri-plugin-barcode-scanner` (Native ML Kit, không JS) | $0 |
| **Sync nền** | WorkManager (sống sót kill/reboot) | $0 |
| **Push** | FCM (High priority cho giao dịch) | $0 |
| **Testing** | Firebase Test Lab + Samsung RTL | $0 |
| **Crash** | Sentry > Crashlytics cho Tauri hybrid | $0 |

**Chiến lược thanh toán**: App **FREE** trên Play Store → chỉ có Login + POS → hết hạn → "Truy cập admin.nodi.vn để quản lý gói" → VietQR 0% phí.

**Timeline**: 8 tuần từ Desktop → Production Android.

---

## ③ iOS — Khi Có Doanh Thu (~$150-200)

📄 Chi tiết: [04-ra-mat-ios-khong-mac.md](./04-ra-mat-ios-khong-mac.md)

### Chiến lược: Cloud Build (Không Cần Mua Mac)

| Hạng mục | Giải pháp | Chi phí |
|----------|-----------|:-------:|
| **Apple Developer** | Đăng ký Cá nhân (Apple ID iPhone) | **$99/năm** |
| **Build** | **Codemagic** pay-as-you-go (Mac đám mây) | ~$1-3/lần |
| **Test** | TestFlight → cài trực tiếp trên iPhone cá nhân | $0 |
| **Billing** | Multiplatform Service: App chỉ Login + POS, KHÔNG UI mua = **0% Apple Tax** | $0 |
| **Printer** | **BLE** (không cần MFi $300+). Plugin `tauri-plugin-blec` | $0 |
| **Sync nền** | `BGAppRefreshTask` (<30 giây, khác Android) | $0 |
| **Barcode** | `tauri-plugin-barcode-scanner` (AVFoundation native) | $0 |

### Quy trình thực tế (không cần Mac)

```
Code trên Windows → git push → Codemagic build iOS (~15-30 phút)
→ Tự upload TestFlight → Test trên iPhone → OK → Submit App Store
```

### Tiết kiệm Codemagic

| Bước | Công cụ | Chi phí |
|------|---------|:-------:|
| Test UI/logic | Chrome DevTools mobile view | $0 |
| Cấu hình iOS | tauri.conf.json + Info.plist trên Windows | $0 |
| Build thực sự | Codemagic (chỉ khi tự tin) | ~$1-3 |

> **Ước tính**: 3-5 lần build đầu tiên (~$5-8) + Apple Dev $99 = **~$105-130** ra App Store.
> Sau đó: ~$2-3/tháng cho updates.
> Khi doanh thu đủ: Mua Mac Mini M1/M2 cũ ($400-600) để debug nhanh 10x.

---

## Vận Hành & Giám Sát ($0)

📄 Chi tiết: [05-giam-sat-va-khoi-phuc.md](./05-giam-sat-va-khoi-phuc.md)

| Hạng mục | Công cụ | Chi phí |
|----------|---------|:-------:|
| **Uptime** | BetterStack (gọi điện P0 lúc 3 giờ sáng) | $0 |
| **Metrics** | Netdata (ML anomaly, zero-config) | $0 |
| **Alerting** | Telegram Bot (bypass DND) | $0 |
| **Backup** | pg_dump mỗi 6h → GPG → Cloudflare R2 | $0 |
| **Rollback** | Blue-Green 1 VPS + Tauri Dynamic Update Server | $0 |
| **Security** | SSH key-only + UFW + Fail2ban + auto security updates | $0 |

> **Lợi thế offline-first**: VPS sập → khách vẫn bán hàng bình thường → sync lại khi VPS phục hồi.

---

## GO LIVE Checklist — Ngày Ra Mắt

📄 Chi tiết: [01-chuan-bi-ra-mat.md](./01-chuan-bi-ra-mat.md)

### Pre-launch (Trước ngày G)

- [ ] **Kỹ thuật**: Sentry, SSL, Backup, Monitoring, Health-check API
- [ ] **Pháp lý**: Privacy Policy, NĐ 13/2023 compliance, Đăng ký website Bộ Công Thương
- [ ] **Marketing**: Landing page nodi.vn/tai-ve, Zalo OA, Video demo
- [ ] **Vận hành**: Help Center, chatbot FAQ, tài khoản demo

### Day-1 Operations

```
06:00  Kiểm tra hệ thống lần cuối
07:00  Bật landing page + post Zalo OA
08:00  Onboard 3-5 đại lý beta đầu tiên (hỗ trợ 1:1 qua Zalo)
12:00  Check Sentry + Netdata
18:00  Tổng kết Day-1 metrics
21:00  Backup thủ công + review feedback
```

### Soft Launch → Hard Launch

| Tiêu chí | Ngưỡng |
|----------|:------:|
| Crash rate | < 1% |
| Sync success | > 95% |
| NPS từ beta users | > 7/10 |

---

## Tổng Chi Phí Ra Mắt

| Nền tảng | Khởi tạo | Hàng năm |
|----------|:--------:|:--------:|
| 🖥️ Desktop Windows | Sectigo OV $226 | $226 |
| 🤖 Android | $25 (1 lần) | $0 |
| 🍎 iOS | Apple Dev $99 + Codemagic ~$10 | ~$125 |
| 🛡️ Monitoring | $0 | $0 |
| **TỔNG** | **~$360** | **~$351/năm** |

> iOS có thể **trì hoãn** cho đến khi Desktop + Android tạo doanh thu. Khi sẵn sàng: chỉ cần ~$150-200 là lên App Store.

---

## Tài Liệu Chi Tiết

| # | File | Nội dung |
|:-:|------|---------|
| 1 | [01-chuan-bi-ra-mat.md](./01-chuan-bi-ra-mat.md) | GO LIVE Checklist, Day-1 Ops, Runbook, KPI |
| 2 | [02-phat-hanh-desktop-windows.md](./02-phat-hanh-desktop-windows.md) | Code Signing, Auto-Update, NSIS, CDN, Telemetry |
| 3 | [03-ra-mat-android.md](./03-ra-mat-android.md) | Play Store, Billing Bypass, Kotlin Plugin, Testing |
| 4 | [04-ra-mat-ios-khong-mac.md](./04-ra-mat-ios-khong-mac.md) | Cloud Build, Apple Tax Bypass, BLE Printer, Plan B |
| 5 | [05-giam-sat-va-khoi-phuc.md](./05-giam-sat-va-khoi-phuc.md) | Monitoring $0, Backup/DR, Rollback, Security |
| 6 | [06-toi-uu-vps-hien-tai.md](./06-toi-uu-vps-hien-tai.md) | PostgreSQL Tuning, Nginx Hardening, SSH, R2 Backup, DR |
| — | [PROMPTS_NGHIEN_CUU.md](./PROMPTS_NGHIEN_CUU.md) | 5 prompts DeepSearch gốc |

---

*Tổng hợp từ nghiên cứu Google DeepSearch (Gemini) — 13/03/2026*
*Nodi POS — SaaS cho đại lý vật tư nông nghiệp Việt Nam*
*Giá: Freemium → 299K/tháng hoặc 1.990K/năm*
