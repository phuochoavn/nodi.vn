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
.info-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
.info-row { padding: 8px 0; color: #1E293B; }
.info-label { font-weight: 600; color: #64748B; }
.success-msg { color: #10B981; margin-bottom: 12px; }
.error-msg { color: #EF4444; margin-bottom: 12px; }
:root.dark .info-row { color: #E2E8F0; }
:root.dark .info-label { color: #94A3B8; }
@media (max-width: 768px) { .info-grid { grid-template-columns: 1fr; } }
</style>
