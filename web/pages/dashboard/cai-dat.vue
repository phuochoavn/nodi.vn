<template>
  <div>
    <h2 class="page-title">⚙️ Cài đặt</h2>

    <div class="card" style="margin-bottom:24px;">
      <h3>🏪 Thông tin cửa hàng</h3>
      <div class="info-grid">
        <div class="info-row"><span class="info-label">Tên:</span> {{ store.name || '—' }}</div>
        <div class="info-row"><span class="info-label">Địa chỉ:</span> {{ store.address || '—' }}</div>
        <div class="info-row"><span class="info-label">SĐT:</span> {{ store.phone || '—' }}</div>
        <div class="info-row"><span class="info-label">License:</span> {{ store.license_key || '—' }}</div>
        <div class="info-row"><span class="info-label">Gói:</span> {{ store.license_type || '—' }}</div>
        <div class="info-row"><span class="info-label">Kích hoạt:</span> {{ store.activated_at || '—' }}</div>
      </div>
    </div>

    <div class="card">
      <h3>🔒 Đổi mật khẩu</h3>
      <form @submit.prevent="changePassword">
        <div class="form-group">
          <label>Mật khẩu hiện tại</label>
          <input v-model="pw.current" type="password" required>
        </div>
        <div class="form-group">
          <label>Mật khẩu mới</label>
          <input v-model="pw.newPw" type="password" required minlength="6">
        </div>
        <p v-if="msg" :class="success ? 'success-msg' : 'error-msg'">{{ msg }}</p>
        <button type="submit" class="btn btn-primary">Đổi mật khẩu</button>
      </form>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Cài đặt — Dashboard' })

const { fetchApi } = useAuth()
const store = ref({})
const pw = reactive({ current: '', newPw: '' })
const msg = ref('')
const success = ref(false)

onMounted(async () => {
  try {
    const r = await fetchApi('/api/dashboard/settings')
    store.value = r.store || {}
  } catch (e) { console.error(e) }
})

async function changePassword() {
  msg.value = ''
  try {
    await fetchApi('/api/dashboard/settings/password', {
      method: 'PUT',
      body: { current_password: pw.current, new_password: pw.newPw },
    })
    msg.value = '✅ Đổi mật khẩu thành công!'
    success.value = true
    pw.current = ''; pw.newPw = ''
  } catch (e) {
    msg.value = '❌ ' + (e.data?.message || 'Lỗi đổi mật khẩu')
    success.value = false
  }
}
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.card h3 { font-size: 1.1rem; font-weight: 700; margin-bottom: 16px; }
.info-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
.info-row { padding: 8px 0; }
.info-label { font-weight: 600; color: #64748B; }
.form-group { margin-bottom: 16px; }
.form-group label { display: block; font-weight: 600; margin-bottom: 6px; font-size: 0.95rem; }
.form-group input {
  width: 100%; max-width: 400px; padding: 12px 16px;
  border: 1px solid #E2E8F0; border-radius: 10px; font-size: 1rem; font-family: inherit;
}
.form-group input:focus { outline: none; border-color: #2563EB; box-shadow: 0 0 0 3px rgb(37 99 235 / 0.1); }
.success-msg { color: #10B981; margin-bottom: 12px; }
.error-msg { color: #EF4444; margin-bottom: 12px; }
@media (max-width: 768px) { .info-grid { grid-template-columns: 1fr; } }
</style>
