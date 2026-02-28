<template>
  <div class="login-page">
    <div class="login-bg"></div>
    <div class="login-card">
      <div class="login-logo">
        <span>🌾</span>
        <h1>Nodi<span class="accent">POS</span></h1>
      </div>
      <p class="login-subtitle">Đăng ký tài khoản miễn phí</p>

      <form @submit.prevent="handleRegister">
        <div class="form-group">
          <label for="store_name">🏪 Tên cửa hàng</label>
          <input id="store_name" v-model="storeName" type="text" placeholder="VD: Đại lý VTNN Minh Phát" required autofocus>
          <p v-if="storeNameError" class="field-error">{{ storeNameError }}</p>
        </div>
        <div class="form-group">
          <label for="phone">📱 Số điện thoại</label>
          <input id="phone" v-model="phone" type="tel" placeholder="VD: 0374222326" required>
          <p v-if="phoneError" class="field-error">{{ phoneError }}</p>
        </div>
        <div class="form-group">
          <label for="username">👤 Tên đăng nhập</label>
          <input id="username" v-model="username" type="text" placeholder="VD: minhphat2025" required>
          <p v-if="usernameError" class="field-error">{{ usernameError }}</p>
        </div>
        <div class="form-group">
          <label for="password">🔒 Mật khẩu</label>
          <div class="password-wrapper">
            <input id="password" v-model="password" :type="showPw ? 'text' : 'password'" placeholder="Tối thiểu 6 ký tự" required>
            <button type="button" class="eye-btn" @click="showPw = !showPw" tabindex="-1">
              <span v-if="showPw">🙈</span>
              <span v-else>👁️</span>
            </button>
          </div>
          <p v-if="passwordError" class="field-error">{{ passwordError }}</p>
        </div>
        <div class="form-group">
          <label for="confirmPassword">🔒 Xác nhận mật khẩu</label>
          <div class="password-wrapper">
            <input id="confirmPassword" v-model="confirmPassword" :type="showPw2 ? 'text' : 'password'" placeholder="Nhập lại mật khẩu" required>
            <button type="button" class="eye-btn" @click="showPw2 = !showPw2" tabindex="-1">
              <span v-if="showPw2">🙈</span>
              <span v-else>👁️</span>
            </button>
          </div>
          <p v-if="confirmError" class="field-error">{{ confirmError }}</p>
        </div>

        <p v-if="error" class="error">{{ error }}</p>
        <p v-if="success" class="success">{{ success }}</p>

        <button type="submit" class="btn btn-primary" :disabled="loading" style="width:100%;justify-content:center;">
          {{ loading ? 'Đang đăng ký...' : 'Đăng ký miễn phí' }}
        </button>
      </form>

      <div class="register-benefits">
        <p>✅ Miễn phí vĩnh viễn &nbsp;•&nbsp; ✅ Quản lý tồn kho &nbsp;•&nbsp; ✅ Cloud backup</p>
      </div>

      <p class="login-help">
        Đã có tài khoản? <NuxtLink to="/login" class="register-link">Đăng nhập →</NuxtLink>
      </p>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: false })
useHead({ title: 'Đăng ký — Nodi POS' })

const config = useRuntimeConfig()
const apiBase = config.public.apiBase || ''

const storeName = ref('')
const phone = ref('')
const username = ref('')
const password = ref('')
const confirmPassword = ref('')
const error = ref('')
const success = ref('')
const loading = ref(false)
const showPw = ref(false)
const showPw2 = ref(false)

// Real-time validation
const storeNameError = computed(() => {
  if (!storeName.value) return ''
  if (storeName.value.trim().length < 2) return 'Tên cửa hàng quá ngắn'
  return ''
})

const phoneError = computed(() => {
  if (!phone.value) return ''
  if (!/^0\d{9,10}$/.test(phone.value)) return 'SĐT phải 10-11 số, bắt đầu bằng 0'
  return ''
})

const usernameError = computed(() => {
  if (!username.value) return ''
  if (username.value.length < 3) return 'Tối thiểu 3 ký tự'
  if (!/^[a-zA-Z0-9_]+$/.test(username.value)) return 'Chỉ gồm chữ, số và _'
  return ''
})

const passwordError = computed(() => {
  if (!password.value) return ''
  if (password.value.length < 6) return 'Mật khẩu tối thiểu 6 ký tự'
  return ''
})

const confirmError = computed(() => {
  if (!confirmPassword.value) return ''
  if (confirmPassword.value !== password.value) return 'Mật khẩu không khớp'
  return ''
})

async function handleRegister() {
  error.value = ''
  success.value = ''

  // Client-side validation
  if (storeNameError.value || phoneError.value || usernameError.value || passwordError.value || confirmError.value) {
    error.value = 'Vui lòng kiểm tra lại thông tin'
    return
  }
  if (!storeName.value.trim() || !phone.value || !username.value || !password.value || !confirmPassword.value) {
    error.value = 'Vui lòng điền đầy đủ thông tin'
    return
  }
  if (password.value !== confirmPassword.value) {
    error.value = 'Mật khẩu xác nhận không khớp'
    return
  }

  loading.value = true
  try {
    const res = await $fetch(`${apiBase}/api/register`, {
      method: 'POST',
      body: {
        username: username.value.trim().toLowerCase(),
        phone: phone.value,
        password: password.value,
        store_name: storeName.value.trim(),
      },
    })

    if (res.success) {
      // Store auth token
      const authToken = useCookie('nodi_token', { maxAge: 86400 })
      authToken.value = res.token

      const storesCookie = useCookie('nodi_stores', { maxAge: 86400 })
      storesCookie.value = JSON.stringify(res.stores || [])

      const activeStore = useCookie('nodi_active_store', { maxAge: 86400 })
      activeStore.value = res.store_id

      success.value = 'Đăng ký thành công! Đang chuyển hướng...'
      setTimeout(() => {
        navigateTo('/dashboard')
      }, 1000)
    } else {
      error.value = res.message || 'Đăng ký thất bại'
    }
  } catch (e) {
    if (e.data?.message) {
      error.value = e.data.message
    } else {
      error.value = 'Lỗi kết nối server. Vui lòng thử lại.'
    }
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}
.login-bg {
  position: fixed; inset: 0;
  background: linear-gradient(135deg, #0F172A 0%, #1a365d 50%, #0f4c3a 100%);
  z-index: 0;
}
.login-card {
  position: relative; z-index: 1;
  background: white;
  border-radius: 16px;
  padding: 40px 36px;
  width: 100%;
  max-width: 440px;
  box-shadow: 0 20px 60px rgb(0 0 0 / 0.3);
}
.login-logo {
  text-align: center;
  margin-bottom: 8px;
}
.login-logo span { font-size: 2.5rem; }
.login-logo h1 { font-size: 1.8rem; font-weight: 800; margin-top: 8px; }
.login-logo .accent { color: #10B981; }
.login-subtitle {
  text-align: center;
  color: #64748B;
  margin-bottom: 28px;
}
.form-group { margin-bottom: 16px; }
.form-group label {
  display: block; font-weight: 600; margin-bottom: 6px; font-size: 0.95rem;
}
.form-group input {
  width: 100%; padding: 12px 16px;
  border: 1px solid #E2E8F0; border-radius: 10px;
  font-size: 1rem; font-family: inherit;
  transition: border-color 0.3s;
}
.form-group input:focus {
  outline: none; border-color: #2563EB;
  box-shadow: 0 0 0 3px rgb(37 99 235 / 0.1);
}
.password-wrapper {
  position: relative;
}
.password-wrapper input {
  padding-right: 48px;
}
.eye-btn {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.2rem;
  padding: 4px 8px;
  border-radius: 6px;
  transition: background 0.2s;
}
.eye-btn:hover {
  background: #F1F5F9;
}
.field-error { color: #F59E0B; font-size: 0.8rem; margin-top: 4px; }
.error { color: #EF4444; font-size: 0.9rem; margin-bottom: 12px; text-align: center; }
.success { color: #10B981; font-size: 0.9rem; margin-bottom: 12px; text-align: center; font-weight: 600; }
.register-benefits {
  text-align: center; margin-top: 16px;
  padding: 12px;
  background: #F0FDF4;
  border-radius: 8px;
  font-size: 0.8rem;
  color: #166534;
}
.login-help {
  text-align: center; margin-top: 16px; color: #64748B; font-size: 0.9rem;
}
.register-link { color: #10B981; font-weight: 600; }
.register-link:hover { text-decoration: underline; }
@media (max-width: 480px) {
  .login-card { margin: 16px; padding: 28px 20px; }
}
</style>
