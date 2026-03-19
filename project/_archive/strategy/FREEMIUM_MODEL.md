# 🎯 Nodi POS Freemium Model

> Tài liệu mô tả chi tiết sự khác biệt giữa gói FREE và PRO

---

## 📊 Tổng Quan Mô Hình

```
┌─────────────────────────────────────────────────────────────────┐
│                     NODI POS FREEMIUM MODEL                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   🆓 FREE MODE                        💎 PRO MODE              │
│   ─────────────                       ────────────              │
│   • Không cần đăng nhập               • Mua License Key         │
│   • 30 ngày dùng thử đầy đủ           • Đăng nhập được          │
│   • Sau trial: 20 đơn/ngày            • Không giới hạn đơn      │
│   • Data được bảo vệ tự động          • Khôi phục data mọi lúc  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔄 Lifecycle Người Dùng

```
[Cài app lần đầu]
       │
       ▼
┌──────────────────┐
│  🎁 TRIAL MODE   │ ← 30 ngày dùng đầy đủ tính năng
│  (30 ngày)       │   như PRO (trừ đăng nhập)
└────────┬─────────┘
         │
         ▼ (Hết 30 ngày)
┌──────────────────┐
│  🆓 FREE MODE    │ ← Giới hạn 20 đơn/ngày
│  (Vĩnh viễn)     │   Vẫn bảo vệ data
└────────┬─────────┘
         │
         ▼ (Mua License Key)
┌──────────────────┐
│  💎 PRO MODE     │ ← Full features + đăng nhập
│  (Trọn đời)      │   Khôi phục data cũ
└──────────────────┘
```

---

## 📋 So Sánh Chi Tiết Tính Năng

### Tính Năng Cơ Bản

| Tính năng | 🎁 Trial (30 ngày) | 🆓 Free | 💎 Pro |
|-----------|:------------------:|:-------:|:------:|
| Bán hàng | ✅ Không giới hạn | ⚠️ **20 đơn/ngày** | ✅ Không giới hạn |
| Quản lý sản phẩm | ✅ Không giới hạn | ✅ Không giới hạn | ✅ Không giới hạn |
| Quản lý khách hàng | ✅ | ✅ | ✅ |
| Quản lý nhà cung cấp | ✅ | ✅ | ✅ |
| Nhập hàng | ✅ | ✅ | ✅ |
| Báo cáo cơ bản | ✅ | ✅ | ✅ |
| Xuất Excel | ✅ | ✅ | ✅ |

### Tính Năng Nâng Cao

| Tính năng | 🎁 Trial | 🆓 Free | 💎 Pro |
|-----------|:--------:|:-------:|:------:|
| 🤖 AI Chatbot | ✅ | ❌ | ✅ |
| ☁️ Cloud Sync 2 chiều | ❌ | ❌ | ✅ |
| 📱 Multi-device | ❌ | ❌ | ✅ |
| 🔄 Khôi phục data | ❌ | ❌ | ✅ |
| 🧾 E-Invoice (HĐĐT) | ❌ | ❌ | ✅ |
| 🔐 Đăng nhập | ❌ | ❌ | ✅ |

### Bảo Vệ Dữ Liệu

| Tính năng | 🎁 Trial | 🆓 Free | 💎 Pro |
|-----------|:--------:|:-------:|:------:|
| 🛡️ Auto backup lên cloud | ✅ | ✅ | ✅ |
| 🔒 Mã hóa dữ liệu | ✅ | ✅ | ✅ |
| 🔄 Khôi phục khi máy hỏng | ❌ | ❌ | ✅ |

---

## 💬 Messaging Strategy

### Nguyên Tắc Truyền Đạt

> **"Bảo vệ dữ liệu"** chứ KHÔNG PHẢI **"Thu thập dữ liệu"**

| ❌ Không nói | ✅ Nên nói |
|--------------|------------|
| Thu thập data | Bảo vệ data |
| Gửi lên server | Backup an toàn |
| Lưu trữ thông tin | Đảm bảo an toàn dữ liệu |
| Đồng bộ dữ liệu | Không lo mất dữ liệu |

### UI Messages

**Onboarding (Lần đầu dùng app):**
```
🛡️ Dữ liệu của bạn luôn được bảo vệ

Nodi POS tự động backup dữ liệu của bạn để đảm bảo 
bạn không bao giờ mất thông tin quan trọng, 
ngay cả khi máy tính gặp sự cố.

[ Tuyệt vời! ✓ ]
```

**Khi hết quota 20 đơn/ngày:**
```
🔒 Đã đạt giới hạn 20 đơn/ngày

Nâng cấp lên gói PRO để:
• Bán hàng không giới hạn
• Khôi phục data mọi lúc
• Sử dụng AI Chatbot thông minh

[ Nâng cấp PRO ] [ Để mai tính ]
```

**Khi phát hiện cài lại máy:**
```
🎉 Tin vui! Dữ liệu cũ vẫn an toàn

Chúng tôi đã bảo vệ dữ liệu của bạn.
Nâng cấp PRO để khôi phục ngay!

[ Nâng cấp PRO ] [ Dùng mới ]
```

---

## 🎯 Conversion Strategy

### Friction Points (Điểm thúc đẩy mua PRO)

1. **20 đơn/ngày** - Giới hạn chính, xuất hiện mỗi ngày
2. **AI Chatbot bị khóa** - Hiển thị nút nhưng yêu cầu PRO
3. **Khôi phục data** - Khi máy hỏng, cần PRO để lấy lại

### Customer Support Script

> **Khách**: "Em ơi, máy anh hỏng, mất hết data rồi!"
>
> **Hỗ trợ**: "Anh yên tâm ạ! Nodi POS đã bảo vệ dữ liệu của anh rồi ạ. 
> Toàn bộ thông tin khách hàng, sản phẩm, đơn hàng vẫn an toàn. 
> Anh nâng cấp lên gói PRO, em sẽ khôi phục lại ngay cho ạ!"

---

## 📅 Changelog

| Ngày | Thay đổi |
|------|----------|
| 2026-01-30 | Tạo tài liệu, thay đổi trial 7→30 ngày, bỏ giới hạn 100 SP |
