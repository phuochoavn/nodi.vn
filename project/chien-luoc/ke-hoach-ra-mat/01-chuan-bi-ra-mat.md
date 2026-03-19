# 🚀 GO LIVE Playbook — Sổ Tay Vận Hành Solopreneur Micro-SaaS B2B

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 13/03/2026
> **Mục đích**: Checklist trước ra mắt, Day-1 Operations, Runbook khẩn cấp, KPI Dashboard

---

## 1. Pre-launch Checklist Hoàn Chỉnh

### 1.1. Trụ Cột Kỹ Thuật (Technical Readiness)

| Hạng mục | Chi tiết cấu hình & Tiêu chuẩn | Trạng thái |
|----------|-------------------------------|:----------:|
| **Giám sát Lỗi (Sentry)** | Tích hợp `tauri-plugin-sentry` — bắt lỗi cả frontend (Vue 3/JS) và backend (Rust panics). Rà soát lỗi `serde rename_all`, lỗi luồng `rayon` + `Tokio` | [ ] |
| **SSL Auto-renew** | Certbot + Nginx. Cronjob: `0 0,12 * * * certbot renew -q`. SSL hết hạn = phá vỡ toàn bộ sync | [ ] |
| **Backup PostgreSQL → S3** | `pg_dump` hàng đêm (2:00 AM), format nén custom (`-Fc`). AWS CLI upload S3 + Retention Policy 30 ngày | [ ] |
| **Giám sát VPS** | Netdata/htop. Alert Telegram: CPU >85%, RAM >90%, Disk còn <15% | [ ] |
| **Chống DDoS & Firewall** | UFW (chỉ port 80, 443, SSH custom). Cloudflare Proxy ẩn IP. Nginx `limit_req` + `limit_conn` | [ ] |
| **Auto-Update Tauri** | Ed25519 signing. Health-check endpoint trả 200 OK. Không gây nghẽn DB | [ ] |

### 1.2. Trụ Cột Pháp Lý (Legal & Compliance)

| Hạng mục | Giải pháp & Hàm ý pháp lý | Trạng thái |
|----------|---------------------------|:----------:|
| **Chính sách Bảo mật (NĐ 13/2023)** | Nodi = "Bên Xử lý dữ liệu". Cam kết không mua bán/chuyển giao. Quy trình data wiping khi hủy gói. EULA cho Windows | [ ] |
| **Hóa đơn Điện tử (NĐ 68/2026 + NĐ 70/2025)** | Từ 1/6/2025: HKD doanh thu >1 tỷ/năm BẮT BUỘC HĐĐT từ máy tính tiền → **động lực tăng trưởng cực lớn**. Tích hợp API T-VAN (VNPT, MISA, EasyInvoice). TOS quy định rõ: Nodi chỉ kết nối kỹ thuật, trách nhiệm kê khai = đại lý | [ ] |
| **Đăng ký Website (Bộ Công Thương)** | online.gov.vn — bắt buộc vì nodi.vn có bảng giá + thu thập thông tin. Cần: ĐKKD + ảnh chụp Chính sách BM/Điều kiện GD/Thanh toán. Phê duyệt 3-5 ngày | [ ] |

### 1.3. Trụ Cột Marketing

| Hạng mục | Chiến lược thực thi | Trạng thái |
|----------|---------------------|:----------:|
| **Tối ưu Landing Page** | Nhắm vào "nỗi đau": thất thoát, thuốc BVTV hết hạn, sai lệch sổ nợ. CTA: "Dùng thử miễn phí 30 ngày - Không cần thẻ tín dụng". SEO long-tail: "phần mềm quản lý cửa hàng vật tư nông nghiệp" | [ ] |
| **Zalo OA** | Đăng ký OA Doanh nghiệp, xác thực bằng GPKD → mở khóa API + tự động hóa. Kênh chính: marketing + onboarding + support | [ ] |
| **Video Demo + Testimonials** | Screencast 2 phút: Quét mã → Lên đơn → Tính nợ → Xuất HĐ → Sync. 2-3 testimonial thật từ beta tester | [ ] |

### 1.4. Trụ Cột Vận Hành

| Hạng mục | Hệ thống | Trạng thái |
|----------|----------|:----------:|
| **Help Center** | Wiki/Notion tĩnh, 30 FAQ. Dự đoán kịch bản nông thôn: cập nhật giá hàng loạt, mất mạng, cài máy in USB, dữ liệu khi máy hỏng. Kèm hình ảnh chỉ rõ click | [ ] |
| **Zalo Chatbot** | Rule-based: "Báo giá" → thẻ liên kết. "Lỗi" → mở chat trực tiếp. Tiết kiệm thời gian gõ phím lặp | [ ] |

---

## 2. Day-1 Operations Manual

### 2.1. Onboarding Workflow

```
Khách truy cập nodi.vn → Tải .msi → Cài đặt → Đăng ký trên Desktop
    ↓
Backend Rust: mã hóa password → tạo SQLite local → gọi API → tạo Tenant_ID trên PostgreSQL
    ↓
App hiển thị QR Zalo OA → Khách quét → Chatbot gửi Dynamic Message chào cá nhân hóa
    ↓
Auto gửi hướng dẫn: "Nhập kho 100 mã vật tư bằng Excel trong 1 phút" (kèm video/file <1MB)
    ↓
→ Dữ liệu mẫu vào app NGAY LẬP TỨC → Khách thấy giá trị → Vượt rào cản ma sát
```

> **Thay thế Email bằng Zalo Automation** — email = "khái niệm chết" với tiểu thương nông thôn.

### 2.2. Quy Trình Báo Lỗi

- **In-App Feedback**: Nút "Báo lỗi" → auto đính kèm thông tin thiết bị + log → webhook → Zalo OA → chat 1:1
- **Sentry (lỗi nghiêm trọng)**: `sentry-rust-minidump` → stack trace → auto alert Telegram khi spike detection

### 2.3. Lịch Trình Day-1

| Khung giờ | Trọng tâm |
|:---------:|-----------|
| **07:00-08:00** | 🔍 Health-Check: API 200, SSL OK, backup S3 đêm qua, disk <70% |
| **08:00-09:30** | 📣 Kích hoạt marketing: Facebook Groups, Zalo Broadcast |
| **09:30-12:00** | 💬 Giám sát + Chat 1:1 với 10 user đầu tiên trên Zalo |
| **12:00-14:00** | 🔧 Code Review: Sentry dashboard, tìm sync fails do timeout 4G. Hotfix nếu blocker |
| **14:00-17:00** | 📞 Cold call user đã đăng ký nhưng chưa tạo HĐ đầu tiên. Phân tích tắc nghẽn UX |
| **17:00-19:00** | 📝 Tổng kết: log bugs vào Trello, thông báo cảm ơn Zalo OA. **NGHỈ NGƠI** |

---

## 3. Soft Launch vs Hard Launch

### 3.1. Soft Launch (2-4 tuần đầu)

- **Quy mô**: 5-10 đại lý beta (người quen, early adopters)
- **Chiến thuật**: **Invite-only** — nodi.vn hiển thị "Phiên bản giới hạn - Yêu cầu dùng thử"
- **Lợi ích**: Duyệt từng KH, white-glove onboarding, bảo vệ server non trẻ
- **Song hành**: Dùng Nodi POS chạy song song với sổ tay giấy/Excel để đối chiếu

### 3.2. Tiêu chí Chuyển sang Hard Launch

| # | Tiêu chí | Ngưỡng |
|:-:|----------|:------:|
| 1 | **Error Rate** | < 0.1% cho tác vụ sống còn (HĐ, công nợ, sync) |
| 2 | **Core Value Realization** | ≥ 3 đại lý dùng Nodi 14 ngày liên tục, bỏ sổ giấy |
| 3 | **Feedback Loop** | Đã fix ≥ 5 lỗi UX/UI nghiêm trọng nhất từ beta |

→ Khi đạt 3/3: Mở cổng đăng ký tự do, kích hoạt "Dùng thử 30 ngày PRO".

---

## 4. Runbook Khẩn Cấp

### Kịch bản 1: VPS Sập Hoàn Toàn

```
1. Offline-first = Khách vẫn bán hàng bình thường (SQLite local)
2. Hard Reboot VPS từ panel nhà cung cấp
3. Nếu thất bại → Zalo Broadcast: "Máy chủ bảo trì, BÁN HÀNG BÌNH THƯỜNG,
   dữ liệu tự đồng bộ khi server hoạt động lại"
4. Nếu VPS chết phần cứng → Terraform/Docker Compose → VPS mới từ snapshot
   → Cấu hình DNS → Restore Database
```

### Kịch bản 2: PostgreSQL Corrupt

```
1. TẮT Nginx/API Gateway → chặn mọi sync (ngăn dữ liệu hỏng lan)
2. SSH vào VPS → DROP database
3. Tải backup .dump từ S3 (bản gần nhất)
4. pg_restore -c -U postgres -d nodi_db -1 /tmp/nodi_backup.dump
5. Bật lại API → Client tự đẩy delta data chưa sync
```

### Kịch bản 3: SSL Hết Hạn

```
1. SSH → certbot renew --force-renewal
2. systemctl reload nginx
3. Rà soát lại cronjob:
   0 0,12 * * * root python3 -c 'import random; import time;
   time.sleep(random.random() * 3600)' && certbot renew -q
```

### Kịch bản 4: DDoS

```
1. Cloudflare Dashboard → Bật "I'm Under Attack Mode"
2. WAF Rate Limiting: chặn IP > 50 req/phút trên endpoint nhạy cảm
3. Kiểm tra Nginx: limit_conn + limit_req đã khai báo đúng
```

### Kịch bản 5: Solopreneur Ốm Nặng (Bus Factor = 1)

**"Tài liệu Bàn giao Khẩn cấp"** — tối đa 2 trang, chia sẻ cho 1 người thân:

| Nội dung | Chi tiết |
|----------|----------|
| **Tài khoản hạ tầng** | VPS, Domain, Cloudflare, Email admin |
| **Restart cơ bản** | 1 dòng lệnh restart server |
| **Thông báo KH** | Quy trình Zalo Broadcast xoa dịu |
| **Freelancer DevOps** | Tên + SĐT + thỏa thuận phí bảo hiểm từ trước |
| **Mật khẩu** | Bitwarden Emergency Access |

---

## 5. KPI Dashboard Day-1

### 5.1. Chỉ số Cốt lõi

**Chỉ số Kinh doanh:**

| Metric | Mô tả | Ngưỡng cảnh báo |
|--------|-------|:----------------:|
| **Signups** | Lượt đăng ký mới/24h | Tracking only |
| **Activation Rate** | % tạo hóa đơn đầu tiên / tổng signups | < 30% = UX có vấn đề |

**Chỉ số Kỹ thuật:**

| Metric | Mô tả | Ngưỡng cảnh báo |
|--------|-------|:----------------:|
| **Sync Success Rate** | % sync SQLite → PostgreSQL thành công | < **99%** = sự cố nghiêm trọng |
| **Error Rate** | Tần suất lỗi từ Sentry | > **0.1%** (giao dịch) = dừng marketing, hotfix |
| **Response Time (P95)** | Thời gian phản hồi API | > **2s** = kiểm tra query/index PostgreSQL |

### 5.2. Công cụ Miễn phí

| Mục đích | Công cụ | Ghi chú |
|----------|---------|---------|
| **Web Analytics** | Plausible / Umami | Self-hosted, không cookie, tôn trọng privacy |
| **System Metrics** | Bash script query DB | `SELECT COUNT(*) FROM users`, `invoices_today`, `sync_errors_count` |
| **Error Monitoring** | Sentry (Free tier) | 5,000 events/tháng — đủ cho Soft Launch |
| **Uptime Monitor** | UptimeRobot | Free 50 monitors |

---

## 6. Case Studies & Bài Học

### 6.1. Sai Lầm Phổ Biến

| Sai lầm | Bài học cho Nodi |
|---------|-----------------|
| **"Bán Vitamin thay Thuốc giảm đau"** | Truyền thông = nói về kết quả ("không bị phạt thuế", "biết ai nợ tiền"), KHÔNG nói về công nghệ ("Rust Tauri bảo mật") |
| **"Single-player Mode"** | Product Hunt / Reddit = vô nghĩa. KH nông thôn chỉ tin khi bạn vào Zalo Group đại lý, tư vấn miễn phí 1-2 tháng TRƯỚC launch |
| **"Đợi hoàn hảo"** | Ship sớm, fix nhanh. Lợi thế solopreneur = tốc độ ra quyết định + chi phí thấp |

### 6.2. Chống Launch Fatigue

- **Giới hạn**: Tối đa 12-14h ngày launch. KHÔNG lập trình tính năng mới 14 ngày đầu
- **"Do Things That Don't Scale"**: Gọi điện/chat 1:1 từng KH → tạo đại sứ trung thành
- **Bug fixes ONLY**: 14 ngày đầu chỉ sửa lỗi, KHÔNG thêm feature

### 6.3. Từ 30 ngày Dùng thử → Paid

| Điểm mạnh | Rủi ro |
|-----------|--------|
| Free 20 đơn/ngày = loại bỏ rào cản chi phí | Nếu 30 ngày PRO không có onboarding tốt → lãng phí |
| Mùa cao điểm: đơn hàng bùng nổ → chạm giới hạn → paywall tự nhiên | **Phải giúp KH setup danh mục thuốc sâu trong 48h đầu** |
| 299K/tháng = "yên tâm trước cơ quan Thuế" | Trial-to-Paid ≈ 0% nếu không tạo được dependency |

---

## Tổng Kết

```
┌──────────────────────────────────────────────────────────┐
│        NODI POS — GO LIVE PLAYBOOK SUMMARY               │
│                                                          │
│  📋 PRE-LAUNCH                                          │
│     ✓ Sentry + SSL + Backup S3 + Monitoring + Firewall  │
│     ✓ Privacy Policy + HĐĐT + Đăng ký Bộ Công Thương   │
│     ✓ Zalo OA + Video Demo + Landing Page               │
│     ✓ Help Center 30 FAQ + Chatbot rules                │
│                                                          │
│  🎯 LAUNCH STRATEGY                                     │
│     1. Soft Launch: 5-10 đại lý invite-only (2-4 tuần)  │
│     2. Hard Launch: khi Error <0.1% + 3 KH bỏ sổ giấy  │
│                                                          │
│  🛡️ RUNBOOK                                             │
│     VPS sập → Offline-first bảo vệ → Reboot/Restore    │
│     DB corrupt → Cô lập → S3 restore → Delta sync      │
│     Bus Factor → Bàn giao 2 trang + Bitwarden backup    │
│                                                          │
│  📊 KPI DAY-1                                           │
│     Sync Rate >99%, Error <0.1%, P95 <2s                │
│     Activation Rate (First Invoice) >30%                │
│                                                          │
│  ⚡ MINDSET                                              │
│     "Thuốc giảm đau" > "Vitamin"                        │
│     Do Things That Don't Scale                           │
│     14 ngày đầu = Bug fixes ONLY                        │
└──────────────────────────────────────────────────────────┘
```

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 13/03/2026*
*Lưu trữ bởi Nodi POS Strategy Team*
