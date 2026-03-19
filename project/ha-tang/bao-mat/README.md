# 🔒 Bảo Mật — Security Architecture

> Licensing, authentication, authorization, data protection

## Tài Liệu

| File | Nội dung | Trạng thái |
|------|----------|:----------:|
| `license-flow.md` | Luồng kích hoạt bản quyền (HWID, license key, trial) | ⬜ Cần viết |
| `authentication.md` | Cloud login, JWT, staff PIN | ⬜ Cần viết |
| `rbac.md` | Role-Based Access Control (Owner, Manager, Staff) | ⬜ Cần viết |
| `data-encryption.md` | Mã hóa dữ liệu nhạy cảm (CCCD, PIN) | ⬜ Cần viết |

## Hiện Trạng

- License: HWID fingerprint + license key file
- Online Guardian: kiểm tra license định kỳ (32KB code)
- Feature Guard: Free/Pro gate
- Staff PIN: 4-digit PIN cho nhân viên
- Cloud JWT: token-based auth cho VPS
