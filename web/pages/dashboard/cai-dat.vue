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

    <!-- Device Management -->
    <div class="card" style="margin-bottom:24px;">
      <div class="section-header">
        <h3>📱 Quản lý thiết bị <span class="device-counter">({{ devices.length }}/{{ maxDevices }})</span></h3>
      </div>

      <div v-if="devices.length > 0" class="device-list">
        <div v-for="device in devices" :key="device.id" class="device-item">
          <div class="device-icon">
            {{ deviceIcon(device.device_type) }}
          </div>
          <div class="device-info">
            <div class="device-name-row">
              <template v-if="editingId === device.id">
                <input v-model="editName" class="rename-input" @keyup.enter="saveRename(device.id)" @keyup.escape="editingId = null" ref="renameInput" />
                <button class="btn-sm success" @click="saveRename(device.id)">✓</button>
                <button class="btn-sm" @click="editingId = null">✕</button>
              </template>
              <template v-else>
                <span class="device-name">{{ device.device_name || 'Thiết bị' }}</span>
                <span class="device-type-badge">{{ deviceLabel(device.device_type) }}</span>
                <button class="btn-sm edit" @click="startRename(device)">✏️</button>
              </template>
            </div>
            <div class="device-meta">
              <span>ID: {{ device.device_id_masked }}</span>
              <span>Lần cuối: {{ fmtDate(device.last_active_at) }}</span>
            </div>
          </div>
          <button class="btn-remove" @click="removeDevice(device)" title="Gỡ thiết bị">🗑️</button>
        </div>
      </div>

      <div v-else class="empty-devices">
        Chưa có thiết bị nào được đăng ký
      </div>

      <div v-if="devices.length < maxDevices" class="slots-remaining">
        Còn {{ maxDevices - devices.length }} slot trống
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

// Devices
const devices = ref([])
const maxDevices = ref(10)
const editingId = ref(null)
const editName = ref('')

onMounted(async () => {
  try {
    const r = await fetchApi('/api/dashboard/settings')
    store.value = r.store || {}
  } catch (e) { console.error(e) }
  await loadDevices()
})

async function loadDevices() {
  try {
    const r = await fetchApi('/api/devices')
    devices.value = r.devices || []
    maxDevices.value = r.max_devices || 10
  } catch (e) { console.error(e) }
}

async function removeDevice(device) {
  const name = device.device_name || 'thiết bị này'
  if (!confirm(`Gỡ ${name}? Thiết bị sẽ cần kích hoạt lại.`)) return
  try {
    await fetchApi(`/api/devices/${device.id}`, { method: 'DELETE' })
    await loadDevices()
  } catch (e) {
    alert('Lỗi: ' + (e.data?.message || 'Không thể gỡ thiết bị'))
  }
}

function startRename(device) {
  editingId.value = device.id
  editName.value = device.device_name || ''
}

async function saveRename(id) {
  if (!editName.value.trim()) return
  try {
    await fetchApi(`/api/devices/${id}`, {
      method: 'PATCH',
      body: { device_name: editName.value.trim() },
    })
    editingId.value = null
    await loadDevices()
  } catch (e) { console.error(e) }
}

function deviceIcon(type) {
  return { windows: '💻', android: '📱', ios: '🍎', web: '🌐' }[type] || '📟'
}
function deviceLabel(type) {
  return { windows: 'Windows', android: 'Android', ios: 'iOS', web: 'Web' }[type] || type
}
function fmtDate(d) {
  if (!d) return '—'
  return new Date(d).toLocaleString('vi-VN', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' })
}

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

/* Device Management */
.section-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
.section-header h3 { margin: 0; }
.device-counter { font-weight: 400; color: #7C3AED; font-size: 0.9rem; }
.device-list { display: flex; flex-direction: column; gap: 8px; }
.device-item { display: flex; align-items: center; gap: 14px; padding: 14px 16px; background: #F8FAFC; border-radius: 10px; border: 1px solid #E2E8F0; }
.device-icon { font-size: 1.5rem; min-width: 40px; text-align: center; }
.device-info { flex: 1; min-width: 0; }
.device-name-row { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.device-name { font-weight: 600; font-size: 0.95rem; }
.device-type-badge { padding: 2px 8px; border-radius: 6px; font-size: 0.75rem; font-weight: 600; background: #EDE9FE; color: #6D28D9; }
.device-meta { display: flex; gap: 16px; font-size: 0.8rem; color: #94A3B8; margin-top: 4px; }
.btn-remove { background: none; border: none; cursor: pointer; font-size: 1rem; opacity: 0.5; padding: 4px; }
.btn-remove:hover { opacity: 1; }
.btn-sm { background: none; border: 1px solid #E2E8F0; border-radius: 4px; cursor: pointer; font-size: 0.8rem; padding: 2px 6px; }
.btn-sm.success { border-color: #10B981; color: #10B981; }
.btn-sm.edit { border: none; font-size: 0.85rem; opacity: 0.5; }
.btn-sm.edit:hover { opacity: 1; }
.rename-input { padding: 4px 8px; border: 1px solid #7C3AED; border-radius: 6px; font-size: 0.85rem; width: 200px; outline: none; }
.empty-devices { text-align: center; color: #94A3B8; padding: 32px; font-size: 0.9rem; }
.slots-remaining { text-align: center; color: #10B981; font-size: 0.85rem; margin-top: 12px; font-weight: 600; }

:root.dark .info-row { color: #E2E8F0; }
:root.dark .info-label { color: #94A3B8; }
:root.dark .device-item { background: #334155; border-color: #475569; }
:root.dark .device-name { color: #F1F5F9; }
:root.dark .device-meta { color: #64748B; }
:root.dark .rename-input { background: #1E293B; color: #E2E8F0; border-color: #7C3AED; }
@media (max-width: 768px) { .info-grid { grid-template-columns: 1fr; } .device-meta { flex-direction: column; gap: 2px; } }
</style>
