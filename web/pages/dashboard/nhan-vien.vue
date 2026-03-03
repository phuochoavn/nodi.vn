<template>
  <div>
    <h2 class="page-title">👥 Quản lý Nhân viên</h2>

    <div class="card">
      <div v-if="loading" class="skeleton-wrap">
        <div v-for="i in 4" :key="i" class="skeleton-row"></div>
      </div>
      <div v-else-if="staffOnly.length === 0" class="empty-state">
        <div class="empty-icon">👤</div>
        <p>Chưa có nhân viên nào</p>
        <p class="empty-sub">Thêm nhân viên từ ứng dụng Nodi POS (desktop), dữ liệu sẽ tự động đồng bộ lên đây.</p>
      </div>
      <div v-else>
        <!-- Stats -->
        <!-- Owner info -->
        <div v-if="ownerName" class="owner-info">
          <span>👑 Chủ cửa hàng: <strong>{{ ownerName }}</strong> — toàn quyền (9/9)</span>
        </div>

        <!-- Stats -->
        <div class="stats-row">
          <div class="mini-stat">
            <div class="mini-value">{{ staffOnly.length }}</div>
            <div class="mini-label">Nhân viên</div>
          </div>
          <div class="mini-stat">
            <div class="mini-value green">{{ staffOnly.filter(s => s.is_active).length }}</div>
            <div class="mini-label">Hoạt động</div>
          </div>
          <div class="mini-stat">
            <div class="mini-value red">{{ staffOnly.filter(s => !s.is_active).length }}</div>
            <div class="mini-label">Vô hiệu</div>
          </div>
        </div>

        <div class="info-bar">
          <span>ℹ️ Thêm/xóa nhân viên từ ứng dụng desktop. Tại đây bạn có thể sửa quyền và bật/tắt trạng thái.</span>
        </div>

        <!-- Table -->
        <div class="table-wrap">
          <table class="data-table">
            <thead>
              <tr>
                <th>Họ tên</th>
                <th>Vai trò</th>
                <th>Trạng thái</th>
                <th>PIN</th>
                <th>Quyền</th>
                <th>Thao tác</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="s in staffOnly" :key="s.id" :class="{ inactive: !s.is_active }">
                <td class="name-cell">
                  <div class="avatar" :class="s.role === 'owner' ? 'owner' : ''">{{ s.display_name?.charAt(0)?.toUpperCase() }}</div>
                  <div>
                    <div class="name-text">{{ s.display_name }}</div>
                    <div class="username-text">{{ s.username }}</div>
                  </div>
                </td>
                <td>
                  <span class="role-badge staff">Nhân viên</span>
                </td>
                <td>
                  <span class="status-dot" :class="s.is_active ? 'active' : ''"></span>
                  {{ s.is_active ? 'Hoạt động' : 'Vô hiệu' }}
                </td>
                <td>
                  <span v-if="s.pin_set" class="pin-badge set" @click="openPinModal(s)" title="Bấm để đổi PIN">🔑 Đã đặt</span>
                  <span v-else class="pin-badge not-set" @click="openPinModal(s)" title="Bấm để đặt PIN">➕ Đặt PIN</span>
                </td>
                <td>
                  <span class="perm-count" :class="permCount(s.permissions) > 0 ? '' : 'zero'">
                    {{ permCount(s.permissions) }}/9
                  </span>
                </td>
                <td class="actions-cell">
                  <button class="btn-edit" @click="openPermModal(s)" title="Sửa quyền">✏️ Quyền</button>
                  <button class="btn-toggle" :class="s.is_active ? 'deactivate' : 'activate'" @click="toggleActive(s)" :disabled="toggling === s.id">
                    {{ s.is_active ? '🚫 Tắt' : '✅ Bật' }}
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- Permission Modal -->
    <Teleport to="body">
      <div v-if="showPermModal" class="modal-backdrop" @click.self="showPermModal = false">
        <div class="modal-box">
          <div class="modal-header">
            <h3>🔐 Phân quyền: {{ editingStaff?.display_name }}</h3>
            <button class="close-btn" @click="showPermModal = false">✕</button>
          </div>
          <div class="modal-body">
            <div v-for="p in permKeys" :key="p.key" class="perm-row">
              <label class="toggle-label">
                <span class="perm-icon">{{ p.icon }}</span>
                <span class="perm-text">{{ p.label }}</span>
                <div class="toggle-switch" :class="{ on: editPerms[p.key] }" @click="editPerms[p.key] = !editPerms[p.key]">
                  <div class="toggle-knob"></div>
                </div>
              </label>
            </div>
          </div>
          <div class="modal-footer">
            <button class="btn-cancel" @click="showPermModal = false">Hủy</button>
            <button class="btn-save" @click="savePerms" :disabled="saving">
              {{ saving ? 'Đang lưu...' : '💾 Lưu quyền' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>

    <!-- PIN Modal -->
    <Teleport to="body">
      <div v-if="showPinModal" class="modal-backdrop" @click.self="showPinModal = false">
        <div class="modal-box" style="max-width: 360px">
          <div class="modal-header">
            <h3>🔑 Đổi mã PIN: {{ pinStaff?.display_name }}</h3>
            <button class="close-btn" @click="showPinModal = false">✕</button>
          </div>
          <div class="modal-body" style="padding: 20px">
            <p style="margin-bottom: 12px; font-size: 0.85rem; color: #64748B">Nhập mã PIN 4 chữ số cho nhân viên</p>
            <div class="pin-input-wrap">
              <input v-for="i in 4" :key="i" :ref="el => { if (el) pinRefs[i-1] = el }" type="text" inputmode="numeric" maxlength="1" class="pin-digit" v-model="pinDigits[i-1]" @input="onPinInput(i-1)" @keydown="onPinKeydown($event, i-1)" />
            </div>
            <p v-if="pinError" class="pin-error">{{ pinError }}</p>
          </div>
          <div class="modal-footer">
            <button class="btn-cancel" @click="showPinModal = false">Hủy</button>
            <button class="btn-save" @click="savePin" :disabled="savingPin || pinDigits.join('').length < 4">
              {{ savingPin ? 'Đang lưu...' : '💾 Lưu PIN' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Nhân viên — Dashboard' })

const { fetchApi } = useAuth()
const { addToast } = useToast()

const staff = ref([])
const loading = ref(true)
const showPermModal = ref(false)
const editingStaff = ref(null)
const editPerms = ref({})
const saving = ref(false)
const toggling = ref(null)
const showPinModal = ref(false)
const pinStaff = ref(null)
const pinDigits = ref(['', '', '', ''])
const pinRefs = ref([])
const savingPin = ref(false)
const pinError = ref('')

const staffOnly = computed(() => staff.value.filter(s => s.role !== 'owner'))
const ownerName = computed(() => {
  const o = staff.value.find(s => s.role === 'owner')
  return o ? o.display_name : null
})

const permKeys = [
  { key: 'view_sales', label: 'Xem trang Bán hàng + Lịch sử ĐH', icon: '🛒' },
  { key: 'view_inventory', label: 'Xem Kho hàng + Nhập hàng + NCC', icon: '📦' },
  { key: 'view_revenue', label: 'Xem doanh thu sidebar', icon: '💰' },
  { key: 'view_reports', label: 'Xem Báo cáo + Chốt sổ + Thuế', icon: '📊' },
  { key: 'view_customers', label: 'Xem Khách hàng', icon: '👥' },
  { key: 'view_cashflow', label: 'Xem Thu Chi', icon: '💳' },
  { key: 'delete_order', label: 'Xóa hóa đơn', icon: '🗑️' },
  { key: 'edit_product_price', label: 'Sửa giá sản phẩm', icon: '✏️' },
  { key: 'access_settings', label: 'Vào Cài đặt', icon: '⚙️' },
]

onMounted(async () => {
  try {
    const r = await fetchApi('/api/dashboard/staff')
    staff.value = r.staff || []
  } catch (e) { console.error(e) }
  loading.value = false
})

function permCount(perms) {
  if (!perms || typeof perms !== 'object') return 0
  return Object.values(perms).filter(v => v === true).length
}

function openPermModal(s) {
  editingStaff.value = s
  const defaults = {}
  permKeys.forEach(p => { defaults[p.key] = false })
  editPerms.value = { ...defaults, ...(s.permissions || {}) }
  showPermModal.value = true
}

async function savePerms() {
  saving.value = true
  try {
    await fetchApi(`/api/dashboard/staff/${editingStaff.value.id}/permissions`, {
      method: 'PUT',
      body: { permissions: editPerms.value },
    })
    // Update local state
    const idx = staff.value.findIndex(s => s.id === editingStaff.value.id)
    if (idx !== -1) staff.value[idx].permissions = { ...editPerms.value }
    showPermModal.value = false
    addToast('Cập nhật quyền thành công', 'success')
  } catch (e) {
    addToast('Lỗi cập nhật quyền', 'error')
  }
  saving.value = false
}

async function toggleActive(s) {
  toggling.value = s.id
  try {
    const r = await fetchApi(`/api/dashboard/staff/${s.id}/toggle-active`, { method: 'PUT' })
    const idx = staff.value.findIndex(x => x.id === s.id)
    if (idx !== -1) staff.value[idx].is_active = r.is_active
    addToast(r.message, 'success')
  } catch (e) {
    addToast('Lỗi thay đổi trạng thái', 'error')
  }
  toggling.value = null
}

function openPinModal(s) {
  pinStaff.value = s
  pinDigits.value = ['', '', '', '']
  pinError.value = ''
  showPinModal.value = true
  nextTick(() => { if (pinRefs.value[0]) pinRefs.value[0].focus() })
}

function onPinInput(idx) {
  const v = pinDigits.value[idx]
  if (v && !/^\d$/.test(v)) { pinDigits.value[idx] = ''; return }
  if (v && idx < 3 && pinRefs.value[idx + 1]) pinRefs.value[idx + 1].focus()
}

function onPinKeydown(e, idx) {
  if (e.key === 'Backspace' && !pinDigits.value[idx] && idx > 0) {
    pinRefs.value[idx - 1].focus()
  }
}

async function savePin() {
  const pin = pinDigits.value.join('')
  if (pin.length !== 4) { pinError.value = 'Nhập đủ 4 chữ số'; return }
  savingPin.value = true
  pinError.value = ''
  try {
    await fetchApi(`/api/dashboard/staff/${pinStaff.value.id}/pin`, {
      method: 'PUT',
      body: { pin },
    })
    const idx = staff.value.findIndex(s => s.id === pinStaff.value.id)
    if (idx !== -1) staff.value[idx].pin_set = true
    showPinModal.value = false
    addToast('Cập nhật mã PIN thành công', 'success')
  } catch (e) {
    pinError.value = e.data?.message || 'Lỗi cập nhật PIN'
  }
  savingPin.value = false
}
</script>

<style scoped>
.owner-info { background: #FEF3C7; border: 1px solid #FCD34D; border-radius: 8px; padding: 10px 14px; margin-bottom: 16px; font-size: 0.85rem; color: #92400E; }
.info-bar { background: #EFF6FF; border: 1px solid #BFDBFE; border-radius: 8px; padding: 10px 14px; margin-bottom: 16px; font-size: 0.85rem; color: #1E40AF; }

.stats-row { display: flex; gap: 12px; margin-bottom: 16px; }
.mini-stat { flex: 1; text-align: center; padding: 12px; background: #F8FAFC; border-radius: 10px; }
.mini-value { font-size: 1.4rem; font-weight: 800; color: #1E293B; }
.mini-value.green { color: #10B981; }
.mini-value.red { color: #EF4444; }
.mini-value.blue { color: #3B82F6; }
.mini-label { font-size: 0.75rem; color: #64748B; margin-top: 2px; }

.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-size: 0.8rem; font-weight: 600; color: #64748B; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; font-size: 0.9rem; vertical-align: middle; }
.data-table tr:hover { background: #F8FAFC; }
.data-table tr.inactive { opacity: 0.55; }

.name-cell { display: flex; align-items: center; gap: 10px; }
.avatar { width: 36px; height: 36px; border-radius: 50%; background: linear-gradient(135deg, #2563EB, #7C3AED); color: white; display: flex; align-items: center; justify-content: center; font-weight: 700; font-size: 0.9rem; flex-shrink: 0; }
.avatar.owner { background: linear-gradient(135deg, #F59E0B, #EF4444); }
.name-text { font-weight: 600; color: #1E293B; }
.username-text { font-size: 0.75rem; color: #94A3B8; }

.role-badge { padding: 4px 10px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }
.role-badge.owner { background: #FEF3C7; color: #B45309; }
.role-badge.staff { background: #DBEAFE; color: #1D4ED8; }

.status-dot { display: inline-block; width: 8px; height: 8px; border-radius: 50%; background: #CBD5E1; margin-right: 6px; }
.status-dot.active { background: #10B981; box-shadow: 0 0 6px rgba(16, 185, 129, 0.5); }

.pin-badge { font-size: 0.8rem; padding: 2px 8px; border-radius: 6px; cursor: pointer; transition: all 0.15s; }
.pin-badge.set { background: #DCFCE7; color: #166534; }
.pin-badge.set:hover { background: #BBF7D0; }
.pin-badge.not-set { color: #2563EB; background: #EFF6FF; border: 1px dashed #93C5FD; }
.pin-badge.not-set:hover { background: #DBEAFE; }

.pin-input-wrap { display: flex; gap: 10px; justify-content: center; margin: 16px 0; }
.pin-digit { width: 52px; height: 60px; text-align: center; font-size: 1.5rem; font-weight: 700; border: 2px solid #E2E8F0; border-radius: 12px; outline: none; transition: border-color 0.2s; background: #F8FAFC; }
.pin-digit:focus { border-color: #2563EB; box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.15); background: white; }
.pin-error { color: #EF4444; font-size: 0.8rem; text-align: center; margin-top: 8px; }

.perm-count { font-weight: 700; font-size: 0.9rem; color: #2563EB; }
.perm-count.full { color: #10B981; }
.perm-count.zero { color: #EF4444; }

.actions-cell { display: flex; gap: 6px; align-items: center; }
.btn-edit { background: #EFF6FF; border: 1px solid #BFDBFE; color: #2563EB; padding: 5px 10px; border-radius: 6px; cursor: pointer; font-size: 0.8rem; font-weight: 600; transition: all 0.15s; }
.btn-edit:hover { background: #DBEAFE; }
.btn-toggle { padding: 5px 10px; border-radius: 6px; cursor: pointer; font-size: 0.8rem; font-weight: 600; border: 1px solid; transition: all 0.15s; }
.btn-toggle.deactivate { background: #FEF2F2; border-color: #FECACA; color: #DC2626; }
.btn-toggle.deactivate:hover { background: #FEE2E2; }
.btn-toggle.activate { background: #F0FDF4; border-color: #BBF7D0; color: #16A34A; }
.btn-toggle.activate:hover { background: #DCFCE7; }
.btn-toggle:disabled { opacity: 0.5; cursor: not-allowed; }
.owner-label { color: #94A3B8; font-size: 0.8rem; }

/* Modal */
.modal-backdrop { position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); z-index: 9999; display: flex; align-items: center; justify-content: center; animation: fadeIn 0.15s; }
.modal-box { background: white; border-radius: 16px; width: 480px; max-width: 95vw; max-height: 90vh; overflow-y: auto; box-shadow: 0 20px 60px rgba(0,0,0,0.2); }
.modal-header { display: flex; justify-content: space-between; align-items: center; padding: 20px 24px; border-bottom: 1px solid #E2E8F0; }
.modal-header h3 { margin: 0; font-size: 1.1rem; color: #1E293B; }
.close-btn { background: none; border: none; font-size: 1.2rem; cursor: pointer; color: #64748B; padding: 4px 8px; border-radius: 6px; }
.close-btn:hover { background: #F1F5F9; }
.modal-body { padding: 16px 24px; }
.modal-footer { display: flex; justify-content: flex-end; gap: 10px; padding: 16px 24px; border-top: 1px solid #E2E8F0; }

.perm-row { padding: 10px 0; border-bottom: 1px solid #F1F5F9; }
.perm-row:last-child { border-bottom: none; }
.toggle-label { display: flex; align-items: center; gap: 10px; cursor: pointer; }
.perm-icon { font-size: 1.2rem; width: 28px; text-align: center; }
.perm-text { flex: 1; font-size: 0.9rem; color: #334155; }

.toggle-switch { width: 44px; height: 24px; border-radius: 12px; background: #CBD5E1; position: relative; transition: background 0.2s; cursor: pointer; flex-shrink: 0; }
.toggle-switch.on { background: #10B981; }
.toggle-knob { width: 20px; height: 20px; border-radius: 50%; background: white; position: absolute; top: 2px; left: 2px; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.2); }
.toggle-switch.on .toggle-knob { transform: translateX(20px); }

.btn-cancel { background: #F1F5F9; border: 1px solid #E2E8F0; color: #64748B; padding: 8px 16px; border-radius: 8px; cursor: pointer; font-weight: 600; }
.btn-cancel:hover { background: #E2E8F0; }
.btn-save { background: #2563EB; border: none; color: white; padding: 8px 20px; border-radius: 8px; cursor: pointer; font-weight: 600; }
.btn-save:hover { background: #1D4ED8; }
.btn-save:disabled { opacity: 0.5; cursor: not-allowed; }

.empty-state { text-align: center; padding: 48px 24px; }
.empty-icon { font-size: 3rem; margin-bottom: 12px; }
.empty-state p { color: #64748B; font-size: 1rem; }
.empty-sub { font-size: 0.85rem; margin-top: 4px; }

.skeleton-wrap { display: flex; flex-direction: column; gap: 12px; }
.skeleton-row { height: 48px; background: linear-gradient(90deg, #F1F5F9 25%, #E2E8F0 50%, #F1F5F9 75%); background-size: 200% 100%; animation: shimmer 1.5s infinite; border-radius: 8px; }
@keyframes shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }
@keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }

/* Dark mode */
:root.dark .owner-info { background: #422006; border-color: #A16207; color: #FDE68A; }
:root.dark .info-bar { background: #1E293B; border-color: #334155; color: #93C5FD; }
:root.dark .mini-stat { background: #1E293B; }
:root.dark .mini-value { color: #E2E8F0; }
:root.dark .data-table th { color: #94A3B8; border-color: #334155; }
:root.dark .data-table td { border-color: #1E293B; }
:root.dark .data-table tr:hover { background: #1E293B; }
:root.dark .name-text { color: #E2E8F0; }
:root.dark .modal-box { background: #0F172A; }
:root.dark .modal-header { border-color: #334155; }
:root.dark .modal-header h3 { color: #E2E8F0; }
:root.dark .modal-footer { border-color: #334155; }
:root.dark .perm-text { color: #CBD5E1; }
:root.dark .perm-row { border-color: #1E293B; }
:root.dark .toggle-switch { background: #475569; }
:root.dark .btn-cancel { background: #1E293B; border-color: #334155; color: #94A3B8; }
:root.dark .pin-digit { background: #1E293B; border-color: #334155; color: #E2E8F0; }
:root.dark .pin-digit:focus { border-color: #3B82F6; background: #0F172A; }
:root.dark .pin-badge.set { background: #064E3B; color: #6EE7B7; }
:root.dark .pin-badge.not-set { background: #1E293B; border-color: #3B82F6; color: #93C5FD; }
</style>
