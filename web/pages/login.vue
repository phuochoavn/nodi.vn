<template>
  <div class="login-page">
    <div class="login-bg"></div>
    <div class="login-card">
      <div class="login-logo">
        <span>🌾</span>
        <h1>Nodi<span class="accent">POS</span></h1>
      </div>
      <p class="login-subtitle">Đăng nhập vào Dashboard</p>

      <form @submit.prevent="handleLogin">
        <div class="form-group">
          <label for="phone">📱 Số điện thoại</label>
          <input id="phone" v-model="phone" type="tel" placeholder="Nhập số điện thoại" required autofocus>
        </div>
        <div class="form-group">
          <label for="password">🔒 Mật khẩu</label>
          <div class="password-wrapper">
            <input id="password" v-model="password" :type="showPw ? 'text' : 'password'" placeholder="Nhập mật khẩu" required>
            <button type="button" class="eye-btn" @click="showPw = !showPw" tabindex="-1">
              <span v-if="showPw">🙈</span>
              <span v-else>👁️</span>
            </button>
          </div>
        </div>
        <p v-if="error" class="error">{{ error }}</p>
        <button type="submit" class="btn btn-primary" :disabled="loading" style="width:100%;justify-content:center;">
          {{ loading ? 'Đang đăng nhập...' : 'Đăng nhập' }}
        </button>
      </form>

      <div class="login-links">
        <a href="#" @click.prevent="alert('Liên hệ Zalo 0374.222.326 để reset mật khẩu')" class="forgot">Quên mật khẩu?</a>
      </div>
      <p class="login-help">
        Chưa có tài khoản? <NuxtLink to="/register" class="register-link">Đăng ký miễn phí →</NuxtLink>
      </p>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: false })
useHead({ title: 'Đăng nhập — Nodi POS' })

const { login, isAuthenticated } = useAuth()
const phone = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)
const showPw = ref(false)

onMounted(() => {
  if (isAuthenticated.value) navigateTo('/dashboard')
})

async function handleLogin() {
  error.value = ''
  loading.value = true
  try {
    const res = await login(phone.value, password.value)
    if (res.success) {
      const dest = res.user?.role === 'admin' ? '/admin' : '/dashboard'
      navigateTo(dest)
    } else {
      error.value = res.message || 'Đăng nhập thất bại'
    }
  } catch (e) {
    error.value = 'Lỗi kết nối server'
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
  padding: 48px 40px;
  width: 100%;
  max-width: 420px;
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
  margin-bottom: 32px;
}
.form-group { margin-bottom: 20px; }
.form-group label {
  display: block; font-weight: 600; margin-bottom: 6px; font-size: 0.95rem;
}
.form-group input {
  width: 100%; padding: 14px 16px;
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
.error { color: #EF4444; font-size: 0.9rem; margin-bottom: 12px; text-align: center; }
.login-links {
  text-align: center; margin-top: 16px;
}
.forgot {
  color: #64748B; font-size: 0.9rem; cursor: pointer;
}
.forgot:hover { color: #2563EB; }
.login-help {
  text-align: center; margin-top: 12px; color: #64748B; font-size: 0.9rem;
}
.login-help a { color: #2563EB; font-weight: 600; }
@media (max-width: 480px) {
  .login-card { margin: 16px; padding: 32px 24px; }
}
</style>
