# 📋 Prompts Nghiên Cứu — Kế Hoạch Ra Mắt Nodi POS

> Dùng với **Google DeepSearch (Gemini)** — mỗi prompt 1 lần search riêng
> Kết quả lưu vào file tương ứng trong folder `ke-hoach-ra-mat/`

---

## Prompt 01 — GO LIVE Checklist & Pre-launch Operations

**Lưu vào**: `01-chuan-bi-ra-mat.md`

```
Tôi là solopreneur (1 người duy nhất) sắp ra mắt (GO LIVE) phần mềm quản lý bán hàng (POS) dạng SaaS cho đại lý vật tư nông nghiệp tại Việt Nam. Sản phẩm hiện tại:
- Desktop app (Windows) xây trên Tauri v2 (Rust backend + Vue 3 frontend)
- Kiến trúc offline-first: SQLite local → sync lên PostgreSQL trên VPS
- 113 sprints, 85,000+ LOC
- Giá: Dùng thử 30 ngày PRO → Free (20 đơn/ngày) → 299K/tháng hoặc 1.990K/năm
- Website: nodi.vn (đã có trang bảng giá, tải app, đăng ký)
- Chưa có user thực nào ngoài beta tester

Hãy nghiên cứu chuyên sâu và tạo một "GO LIVE Playbook" toàn diện cho solopreneur micro-SaaS. Cụ thể:

1. **Pre-launch Checklist hoàn chỉnh** (chia theo: Kỹ thuật, Pháp lý, Marketing, Vận hành):
   - Kỹ thuật: SSL auto-renew, monitoring VPS (uptime, CPU, RAM, disk), database backup verify, error tracking (Sentry?), logging, health-check endpoint, auto-restart scripts
   - Pháp lý: Chính sách bảo mật (Privacy Policy), điều khoản sử dụng (Terms of Service), EULA cho desktop app, đăng ký kinh doanh Việt Nam, HĐĐT setup
   - Marketing: Landing page optimization, Zalo OA setup, demo video, testimonial từ beta users, SEO cơ bản
   - Vận hành: Help Center viết sẵn, FAQ 30 câu phổ biến nhất, template trả lời Zalo, quy trình onboarding user mới

2. **Day-1 Operations Manual** — workflow chính xác trong ngày đầu tiên:
   - Khi user đăng ký → chuyện gì xảy ra? Auto email? Zalo welcome? Hướng dẫn cài đặt?
   - Khi user gặp lỗi → báo ở đâu? Zalo? Email? In-app feedback?
   - Monitoring dashboard cần theo dõi gì real-time?
   - Lịch trình làm việc ngày đầu launch (giờ nào check gì?)

3. **Soft Launch vs Hard Launch** cho micro-SaaS B2B nông thôn:
   - Nên mời bao nhiêu beta users trước khi public?
   - Criteria nào để quyết định chuyển từ Soft Launch sang Hard Launch?
   - Có nên giới hạn đăng ký trong tuần đầu (invite-only)?

4. **Runbook khẩn cấp** (Emergency Runbook) cho solopreneur:
   - VPS sập → bước 1, 2, 3?
   - Database corrupt → recovery procedure?
   - SSL hết hạn → auto-fix?
   - Bị DDoS → mitigation?
   - Nhà sáng lập ốm 1 tuần → ai can thiệp? Cần brief gì cho người backup?

5. **KPI Dashboard ngày đầu launch**:
   - Metrics nào cần track từ Day 1? (signups, activation rate, first invoice created, sync success rate, error rate)
   - Công cụ miễn phí nào phù hợp cho solopreneur? (Plausible, Umami, simple DB query?)
   - Threshold cảnh báo: bao nhiêu error/giờ là đáng lo? Response time bao nhiêu ms là chấp nhận được?

6. **Case studies GO LIVE** của các micro-SaaS solopreneur:
   - Những sai lầm phổ biến nhất ngày đầu launch?
   - "Launch fatigue" — làm sao tránh kiệt sức ngay khi launch?
   - Bài học từ IndieHackers, Hacker News "Launch HN" posts?

Hãy trả lời dưới dạng BÁO CÁO NGHIÊN CỨU CHUYÊN SÂU với bảng biểu, checklist có checkbox, và ví dụ cụ thể. Ưu tiên thực chiến cho 1 NGƯỜI DUY NHẤT vận hành, không phải team.
```

---

## Prompt 02 — Desktop Windows Launch Strategy

**Lưu vào**: `02-phat-hanh-desktop-windows.md`

```
Tôi đang chuẩn bị phát hành ứng dụng desktop Windows được xây dựng bằng Tauri v2 (Rust + Vue 3). Đây là phần mềm POS SaaS cho đại lý vật tư nông nghiệp Việt Nam. Giá 299K/tháng. Đặc điểm:
- File .exe installer, kích thước ~15MB (Tauri, không phải Electron)
- Offline-first: hoạt động 100% không cần internet, sync khi có mạng
- SQLite local database, sync lên PostgreSQL cloud
- Tích hợp máy in nhiệt (58mm/80mm) qua USB
- Target users: chủ đại lý 40-60 tuổi, Windows 10/11, máy tính cấu hình thấp
- Phân phối qua website nodi.vn (direct download .exe)
- KHÔNG qua Microsoft Store

Hãy nghiên cứu chuyên sâu về chiến lược phát hành ứng dụng desktop Windows cho B2B SaaS tại thị trường mới nổi:

1. **Code Signing Certificate cho Windows**:
   - Có bắt buộc không? SmartScreen warning nếu không sign?
   - OV (Organization Validation) vs EV (Extended Validation) certificate — cái nào cần?
   - Chi phí hàng năm? Nhà cung cấp nào rẻ nhất cho startup VN? (Sectigo, DigiCert, Comodo)
   - Quy trình sign .exe với Tauri v2 — cần config gì trong tauri.conf.json?
   - Nếu KHÔNG sign, cách nào giảm thiểu SmartScreen blocking?

2. **Auto-Update cho Tauri Desktop**:
   - Tauri v2 updater plugin: setup, hosting, cách hoạt động
   - Self-hosted update server vs GitHub Releases vs S3
   - Chiến lược rollout: 100% ngay hay canary release 10% trước?
   - Xử lý trường hợp update fail — rollback mechanism
   - Delta update (chỉ tải phần thay đổi) có khả thi với Tauri?

3. **Installer Experience cho người dùng lớn tuổi nông thôn**:
   - NSIS vs WiX vs MSI vs Tauri default (.msi) — cái nào smooth nhất?
   - Silent install có nên dùng? Desktop shortcut auto-create?
   - Antivirus false positive — làm sao tránh bị Kaspersky/BKAV chặn?
   - Cách xử lý khi Windows Defender SmartScreen chặn: hướng dẫn user bấm "More info" → "Run anyway"

4. **Phân phối .exe không qua Store**:
   - Direct download từ website vs Zalo gửi file trực tiếp
   - CDN nào phù hợp cho VN? (Cloudflare, BunnyCDN, hoặc VPS trực tiếp)
   - Download resume support nếu mạng chập chờn?
   - Checksum/hash verification cho user — có cần thiết cho tệp khách nông thôn?

5. **Hardware compatibility**:
   - Tauri v2 yêu cầu WebView2 (Edge Chromium) — Windows 10 version nào có sẵn?
   - Nếu WebView2 chưa có → auto-install bootstrapper?
   - Máy tính cũ (CPU Celeron, 2-4GB RAM) có chạy được không?
   - Tương thích máy in nhiệt phổ biến tại VN: Xprinter Q200, Sunmi, Epson TM-T82

6. **Telemetry & crash reporting cho desktop app**:
   - Sentry cho Tauri — setup cho cả Rust panic và JavaScript error
   - Anonymous usage analytics (không vi phạm NĐ 13/2023)
   - Disk space monitoring: cảnh báo khi SQLite quá lớn

Hãy trả lời dưới dạng BÁO CÁO NGHIÊN CỨU với bảng so sánh chi phí, quy trình kỹ thuật từng bước, và case studies từ các Tauri/Electron app tương tự.
```

---

## Prompt 03 — Android App Launch Strategy

**Lưu vào**: `03-ra-mat-android.md`

```

```

Tôi đang phát triển phần mềm POS SaaS cho đại lý vật tư nông nghiệp Việt Nam bằng Tauri v2 (Rust + Vue 3). Desktop Windows đã hoạt động production-ready. Giờ tôi muốn ra mắt trên Android. Thông tin:

- Tauri v2 hỗ trợ Android native (WebView + Rust backend qua JNI)
- Đã build thành công APK từ Tauri, nhưng chưa publish lên Store
- App cần: SQLite offline, camera quét barcode, Bluetooth máy in nhiệt
- Target device: điện thoại Android giá rẻ phổ biến ở nông thôn VN (Samsung Galaxy A04s, Redmi 12C, Realme C55, OPPO A17k) — Android 10+, 3-4GB RAM
- Tôi là solopreneur, KHÔNG có team QA, KHÔNG có hàng trăm thiết bị test

Hãy nghiên cứu chuyên sâu về chiến lược ra mắt app Android cho B2B SaaS micro:

1. **Google Play Store publishing cho startup VN**:

   - Tài khoản Google Play Developer: phí $25, yêu cầu xác minh danh tính 2024/2025
   - Quy trình xác minh D-U-N-S number cho doanh nghiệp VN — bắt buộc không?
   - Yêu cầu mới 2025-2026: Google Play data safety form, app content rating, target audience
   - Review time trung bình? Lý do bị reject phổ biến?
   - Classified as "Not designed for children" — cần khai báo gì?
2. **APK Sideload vs Google Play** — phân tích ưu nhược:

   - Sideload APK qua website/Zalo: không phí, không review, nhưng user phải "Allow unknown sources"
   - Google Play: uy tín, auto-update, nhưng 15% commission nếu thu phí in-app
   - Chiến lược hybrid: Free trên Play Store + billing bypass qua VietQR (như desktop)?
   - Có vi phạm Google Play policy nếu redirect thanh toán ra ngoài không?
3. **Tauri v2 Android — kỹ thuật chuyên sâu**:

   - Tauri Android architecture: Activity → WebView → Rust core qua JNI bridge
   - Performance trên thiết bị giá rẻ (Mediatek Helio G35, 3GB RAM)?
   - SQLite trên Android: thư mục lưu trữ, quyền storage, backup/restore
   - Camera integration cho barcode scanning: Tauri plugin vs JavaScript (Html5-QRCode)?
   - Bluetooth printing: ESC/POS commands qua Bluetooth SPP — Tauri plugin hay Android native?
   - Push notification: Firebase Cloud Messaging (FCM) có tích hợp được với Tauri?
   - Background sync: WorkManager vs AlarmManager cho periodic sync?
4. **Testing trên thiết bị giá rẻ Android — chiến lược solopreneur**:

   - Firebase Test Lab: free tier bao nhiêu thiết bị/tháng?
   - BrowserStack / Samsung Remote Test Lab — miễn phí?
   - Danh sách 10 thiết bị Android phổ biến nhất tại VN 2025-2026 cần test
   - Performance profiling: Android Profiler, memory leak detection
   - Crash reporting: Firebase Crashlytics vs Sentry — cái nào tốt hơn cho Tauri?
5. **App Signing & Release Management**:

   - Android App Bundle (.aab) vs APK — Google Play yêu cầu gì?
   - Play App Signing: Google quản lý key hay self-manage?
   - Versioning strategy: semver sync với desktop app?
   - Staged rollout: 10% → 50% → 100% — có nên dùng?
   - ProGuard/R8 obfuscation cho WebView app — cần không?
6. **Đặc thù thị trường Android nông thôn VN**:

   - Tỷ lệ Android vs iOS tại khu vực nông thôn VN? (dự đoán 95%+ Android)
   - Data saver mode, battery optimization "giết app" — cách handle?
   - Dung lượng cài đặt lý tưởng cho thị trường nông thôn (<50MB)?
   - Multi-user/multi-account trên 1 thiết bị (vợ chồng cùng dùng)?

Trả lời dạng BÁO CÁO có bảng so sánh, timeline ra mắt, chi phí ước tính, và checklist từng bước cho solopreneur.

```

```

## Prompt 04 — iOS App Launch (Không có Mac)

**Lưu vào**: `04-ra-mat-ios-khong-mac.md`

```
Tôi là solopreneur phát triển phần mềm POS SaaS (Tauri v2 — Rust + Vue 3) và KHÔNG SỞ HỮU máy Mac. Tôi cần ra mắt app trên iOS App Store. Desktop Windows đã production-ready, Android APK đã build được. Giờ iOS là thử thách lớn nhất.

Bối cảnh kỹ thuật:
- Tauri v2 hỗ trợ iOS native (WKWebView + Rust core)
- Build iOS BẮT BUỘC phải dùng macOS + Xcode (Apple restriction)
- Tôi chỉ có PC Windows, KHÔNG có MacBook hay Mac Mini
- Budget hạn chế (solopreneur, bootstrapping)
- App cần: SQLite offline, camera barcode, Bluetooth printer (iOS giới hạn)

Hãy nghiên cứu TOÀN DIỆN mọi cách để publish iOS app mà không sở hữu Mac:

1. **Giải pháp Cloud macOS cho CI/CD**:
   - **GitHub Actions macOS runners**: free tier bao nhiêu phút/tháng? Đủ build Tauri iOS?
   - **MacStadium / Mac Mini Cloud**: giá thuê Mac Mini M1/M2 hàng tháng?
   - **AWS EC2 Mac instances**: chi phí dedicated host? Có overkill cho indie dev?
   - **Codemagic CI/CD**: free tier, hỗ trợ Tauri v2 iOS không? Pricing cho indie?
   - **MacinCloud**: thuê Mac theo giờ/tháng — giá và trải nghiệm?
   - **Hetzner/OVH Mac Mini**: có option nào rẻ hơn ở EU?
   - So sánh tổng chi phí: thuê cloud vs mua Mac Mini M2 ($599 refurbished)

2. **Apple Developer Program cho doanh nghiệp VN**:
   - Phí $99/năm — đăng ký cá nhân hay tổ chức (organization)?
   - Nếu đăng ký tổ chức: cần D-U-N-S number — quy trình cho công ty VN?
   - Cần Apple ID region nào? VN hay US?
   - Thanh toán $99 bằng thẻ Visa VN được không?
   - Timeline: từ đăng ký → approved → có thể submit app mất bao lâu?

3. **App Store Review Guidelines — rủi ro reject**:
   - Guideline 3.1.1: In-app purchase bắt buộc cho digital goods — SaaS subscription có bắt buộc dùng IAP không?
   - "Reader app" exception (Netflix, Spotify model) — POS SaaS có qualify không?
   - Guideline 4.2: Minimum functionality — WebView app có bị reject vì "not native enough" không?
   - Guideline 2.1: Performance — WKWebView (Tauri) có đáp ứng được tiêu chuẩn Apple?
   - Metadata rejection: screenshot requirements, app description best practices
   - Kinh nghiệm submit Tauri v2 app lên App Store — có ai đã thành công chưa?

4. **iOS Billing Strategy — bypass 30% Apple Tax**:
   - Apple cho phép "link out" (External Purchase Links) từ 2024 — áp dụng ở VN chưa?
   - Chiến lược "Reader app": không có nút mua trong app, user đăng ký qua web nodi.vn
   - Anti-steering provisions: có được nói "giá rẻ hơn trên website" trong app không?
   - Small Business Program: 15% thay vì 30% — điều kiện đủ?
   - So sánh: 15% Apple commission vs 0% VietQR direct (qua desktop/web)
   - Quyết định: có nên offering In-App Purchase hay bypass hoàn toàn?

5. **Giới hạn kỹ thuật iOS vs Android cho POS app**:
   - Bluetooth printing trên iOS: MFi program bắt buộc? Máy in ESC/POS nào tương thích?
   - NFC/Barcode scanning: AVFoundation camera vs third-party SDK
   - Background sync: iOS giới hạn background execution — cách nào sync data khi app bị suspend?
   - SQLite trên iOS: App Group containers, iCloud backup exclusion
   - File system sandbox: import/export Excel file trên iOS thế nào?
   - Push notification: APNs setup (cần .p8 key từ Apple Developer portal)

6. **Timeline & Chi phí Tổng thể để có iOS app**:
   - Từ 0 → App Store: mất bao nhiêu tuần/tháng nếu code đã có (Tauri v2)?
   - Chi phí tối thiểu năm đầu: Apple Developer ($99) + Cloud Mac (bao nhiêu?) + SSL/Domain
   - ROI: với tỷ lệ iOS ~5% ở nông thôn VN, có đáng đầu tư ngay không?
   - Recommendation: nên launch iOS trước hay sau khi Desktop + Android ổn định?

7. **Phương án thay thế nếu iOS quá khó/đắt**:
   - Progressive Web App (PWA) trên Safari: khả năng offline, push notification (iOS 16.4+)?
   - Capacitor/Ionic wrapper thay vì Tauri cho iOS?
   - React Native / Flutter rewrite cho mobile — cost-benefit vs Tauri v2?
   - Chỉ build Android + Desktop, bỏ iOS — rủi ro mất bao nhiêu % thị trường?

Trả lời dạng BÁO CÁO CHUYÊN SÂU với bảng so sánh chi phí, decision matrix, timeline cụ thể, và recommendation rõ ràng cho solopreneur bootstrapped tại Việt Nam.
```

---

## Prompt 05 — Monitoring, Rollback & Disaster Recovery

**Lưu vào**: `05-giam-sat-va-khoi-phuc.md`

```
Tôi vận hành một phần mềm POS SaaS (Tauri v2 desktop + mobile) trên 1 VPS duy nhất (2 vCPU, 8GB RAM, Ubuntu). Backend: Rust Axum + PostgreSQL. Architecture: offline-first (SQLite client → sync → PostgreSQL cloud). Tôi là SOLOPRENEUR — Bus Factor = 1, không có team ops.

Hãy nghiên cứu chuyên sâu về monitoring, incident response, và disaster recovery cho solo-operated micro-SaaS:

1. **Monitoring Stack cho Solopreneur** (chi phí ~$0):
   - Uptime monitoring: UptimeRobot (free 50 monitors) vs BetterStack vs self-hosted
   - Server metrics: htop/glances vs Netdata vs Prometheus+Grafana — cái nào đủ dùng cho 1 VPS?
   - Application monitoring: health-check endpoint design, response time tracking
   - Database monitoring: PostgreSQL pg_stat_statements, slow query log, connection pool monitor
   - Disk space alerts: khi nào SQLite backup + WAL log phình to?
   - SSL certificate expiry monitoring: auto-alert trước 14 ngày

2. **Alerting — kênh nào cho solopreneur VN?**:
   - Telegram Bot: setup alert push, free, nhận trên điện thoại 24/7
   - Zalo notification: có API gửi alert cho chính mình không?
   - Email alert: Gmail + filter rule?
   - PagerDuty/OpsGenie: quá đắt cho indie — alternative nào miễn phí?
   - Escalation policy khi founder ngủ: alert nào đánh thức? Alert nào chờ sáng mai?

3. **Auto-healing Scripts**:
   - Systemd service: auto-restart Rust Axum khi crash (Restart=always)
   - PostgreSQL: auto-restart, auto-vacuum, connection pool recovery
   - Nginx: upstream health checks, automatic failover
   - Let's Encrypt: certbot auto-renew cron job — pitfalls phổ biến
   - Disk space: auto-cleanup old logs, old SQLite backups khi disk >85%
   - Memory leak detection: OOM killer config, ulimit settings

4. **Backup & Disaster Recovery cho PostgreSQL**:
   - pg_dump vs pg_basebackup vs WAL archiving — cái nào phù hợp micro-SaaS?
   - Backup schedule: mỗi ngày? 6 giờ? Real-time WAL streaming?
   - Backup storage: S3 (AWS) vs R2 (Cloudflare, free egress) vs Backblaze B2
   - Point-in-time Recovery (PITR): có cần cho micro-SaaS <1,000 users?
   - Backup verification: auto-restore test hàng tuần — script?
   - RPO (Recovery Point Objective) và RTO (Recovery Time Objective) cho SaaS nông nghiệp
   - Encryption at rest: pgcrypto vs full-disk encryption vs backup encryption

5. **Rollback Strategy cho App Updates**:
   - Tauri auto-updater: nếu update lỗi → user bị stuck → cách rollback?
   - Database migration rollback: up/down migration pattern
   - Feature flags: toggle tính năng mới mà không cần deploy lại
   - Canary release: roll out update cho 10% users trước — có khả thi cho solopreneur?
   - Blue-green deployment trên 1 VPS — có thể thực hiện được không?

6. **Incident Response Playbook cho 1 Người**:
   - Severity levels: P0 (mất dữ liệu) → P1 (app crash) → P2 (UI bug) → P3 (feature request)
   - P0 response: bước 1-2-3-4 cụ thể, timeline giải quyết tối đa
   - Communication template: thông báo gì cho user khi có sự cố? (Zalo OA, in-app banner)
   - Post-mortem template: viết gì sau khi khắc phục xong?
   - Status page: có cần cho micro-SaaS? (Cachet, Instatus free tier)

7. **Security Hardening cho VPS**:
   - SSH key-only, disable password login, change default port
   - Firewall: ufw rules cho Rust Axum (port 443, 80 only)
   - fail2ban: chống brute force SSH/API
   - Rate limiting: Nginx limit_req cho API endpoints
   - PostgreSQL: chỉ listen localhost, không expose port 5432 ra internet
   - Automatic security updates: unattended-upgrades config
   - Intrusion detection: OSSEC vs simple audit log?

Trả lời dạng BÁO CÁO với commands/scripts cụ thể (copy-paste được), bảng so sánh tools, và runbook từng bước. Tối ưu cho 1 NGƯỜI DUY NHẤT quản lý, chi phí gần 0.
```

---

## Thứ tự Nghiên cứu Đề xuất

| Ưu tiên | Prompt                                                                     | Lý do                                                      |
| :-------: | -------------------------------------------------------------------------- | ----------------------------------------------------------- |
|    🥇    | **01 — Chuẩn bị Ra mắt** (`01-chuan-bi-ra-mat.md`)             | Cần TRƯỚC mọi thứ khác                                |
|    🥈    | **05 — Giám sát & Khôi phục** (`05-giam-sat-va-khoi-phuc.md`) | Bảo vệ hệ thống SAU khi launch                          |
|    🥉    | **02 — Desktop Windows** (`02-phat-hanh-desktop-windows.md`)      | Nền tảng chính, Code Signing + Auto-Update               |
|   4️⃣   | **03 — Android** (`03-ra-mat-android.md`)                         | Mở rộng thị trường Android (~95% nông thôn VN)       |
|   5️⃣   | **04 — iOS (Không Mac)** (`04-ra-mat-ios-khong-mac.md`)          | Thách thức lớn nhất, nghiên cứu trước khi đầu tư |

---

*Soạn bởi Nodi POS Strategy Team — 13/03/2026*
