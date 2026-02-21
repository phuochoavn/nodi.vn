<template>
  <div>
    <div class="page-header"><h2 class="page-title">🔑 License Manager</h2><button class="btn btn-primary" @click="showCreate = true">+ Tạo License</button></div>

    <!-- Stats bar -->
    <div class="stats-bar">
      <div class="stat-pill"><span class="pill-n">{{ stats.total }}</span> Tổng</div>
      <div class="stat-pill active"><span class="pill-n">{{ stats.active }}</span> Active</div>
      <div class="stat-pill warn"><span class="pill-n">{{ stats.expired }}</span> Hết hạn</div>
      <div class="stat-pill danger"><span class="pill-n">{{ stats.revoked }}</span> Revoked</div>
      <div class="stat-pill muted"><span class="pill-n">{{ stats.pending }}</span> Pending</div>
    </div>

    <!-- License Table -->
    <div class="table-wrap card">
      <table class="data-table">
        <thead><tr><th>Key</th><th>Loại</th><th>Trạng thái</th><th>Cửa hàng</th><th>Hết hạn</th><th>Còn lại</th><th>TT</th><th>Actions</th></tr></thead>
        <tbody>
          <tr v-for="l in licenses" :key="l.id" :class="rowClass(l)" @click="openDetail(l)">
            <td class="mono">{{ l.license_key }}</td>
            <td>{{ typeLabel(l.license_type) }}</td>
            <td><span class="badge" :class="l.status">{{ statusLabel(l.status) }}</span></td>
            <td>{{ l.store_name || '—' }}</td>
            <td>{{ l.expires_at?.slice(0,10) || '—' }}</td>
            <td :class="remainClass(l)">{{ remainLabel(l) }}</td>
            <td><span :class="l.has_recent_payment ? 'pay-ok' : 'pay-no'">{{ l.has_recent_payment ? '💰' : '⚠️' }}</span></td>
            <td class="actions" @click.stop>
              <button class="btn-sm blue" @click="showExtend = l" title="Gia hạn">📅</button>
              <button v-if="l.status!=='REVOKED'" class="btn-sm red" @click="doAction(l.id,'revoke')" title="Thu hồi">🚫</button>
              <button v-if="l.status==='REVOKED'" class="btn-sm green" @click="doAction(l.id,'activate')" title="Kích hoạt lại">✅</button>
              <button class="btn-sm" @click="doAction(l.id,'reset_hwid')" title="Reset HWID">🔄</button>
              <button class="btn-sm blue" @click="showPayment = l" title="Ghi nhận TT">💳</button>
            </td>
          </tr>
          <tr v-if="licenses.length===0"><td colspan="8" class="empty">Chưa có license</td></tr>
        </tbody>
      </table>
    </div>

    <!-- Create Modal -->
    <div v-if="showCreate" class="modal-overlay" @click.self="showCreate=false">
      <div class="modal card">
        <h3>Tạo License mới</h3>
        <div class="form-group"><label>Gói</label>
          <select v-model="newType">
            <option value="MONTHLY">Tháng (30 ngày)</option>
            <option value="YEARLY">Năm (365 ngày)</option>
            <option value="TRIAL">Dùng thử (30 ngày)</option>
          </select>
        </div>
        <div class="form-group"><label>Tên cửa hàng / Ghi chú</label><input v-model="newNote" placeholder="KH Anh Ba - Đồng Tháp"></div>
        <div class="duration-info">⏱️ Thời hạn: <strong>{{ newType==='YEARLY'?'365':'30' }} ngày</strong></div>
        <p v-if="created" class="success">✅ {{ created }}</p>
        <p v-if="createError" class="error-msg">❌ {{ createError }}</p>
        <button class="btn btn-primary" @click="create" :disabled="creating" style="width:100%;justify-content:center;">{{ creating ? 'Đang tạo...' : 'Tạo' }}</button>
      </div>
    </div>

    <!-- Extend Modal -->
    <div v-if="showExtend" class="modal-overlay" @click.self="showExtend=null">
      <div class="modal card">
        <h3>📅 Gia hạn License</h3>
        <p class="mono" style="margin-bottom:12px;">{{ showExtend.license_key }}</p>
        <div class="form-group"><label>Thêm</label>
          <select v-model="extendDays">
            <option :value="30">30 ngày</option>
            <option :value="90">90 ngày</option>
            <option :value="365">365 ngày</option>
          </select>
        </div>
        <div class="duration-info">📆 Hết hạn hiện tại: <strong>{{ showExtend.expires_at?.slice(0,10) || 'Chưa set' }}</strong><br>📆 Hết hạn mới: <strong>{{ newExpiry }}</strong></div>
        <button class="btn btn-primary" @click="doExtend" :disabled="extending" style="width:100%;justify-content:center;">{{ extending ? 'Đang gia hạn...' : 'Gia hạn' }}</button>
      </div>
    </div>

    <!-- Payment Modal -->
    <div v-if="showPayment" class="modal-overlay" @click.self="showPayment=null">
      <div class="modal card">
        <h3>💳 Ghi nhận thanh toán</h3>
        <p class="mono" style="margin-bottom:12px;">{{ showPayment.license_key }} — {{ showPayment.store_name }}</p>
        <div class="form-group"><label>Số tiền (VND)</label><input v-model.number="payAmount" type="number" placeholder="200000"></div>
        <div class="form-group"><label>Phương thức</label>
          <select v-model="payMethod">
            <option value="BANK_TRANSFER">Chuyển khoản</option>
            <option value="CASH">Tiền mặt</option>
            <option value="MOMO">MoMo</option>
          </select>
        </div>
        <div class="form-group"><label>Ghi chú</label><input v-model="payNote" placeholder="CK T2/2026"></div>
        <p v-if="payError" class="error-msg">❌ {{ payError }}</p>
        <p v-if="paySuccess" class="success">✅ Đã ghi nhận</p>
        <button class="btn btn-primary" @click="doPayment" :disabled="paying" style="width:100%;justify-content:center;">{{ paying ? 'Đang lưu...' : 'Ghi nhận' }}</button>
      </div>
    </div>

    <!-- Detail Panel -->
    <div v-if="detailLicense" class="modal-overlay" @click.self="detailLicense=null">
      <div class="detail-panel card">
        <div class="detail-header">
          <h3>{{ detailLicense.license_key }}</h3>
          <span class="badge" :class="detailLicense.status">{{ statusLabel(detailLicense.status) }}</span>
        </div>
        <div class="detail-grid">
          <div><strong>Gói:</strong> {{ typeLabel(detailLicense.license_type) }}</div>
          <div><strong>Cửa hàng:</strong> {{ detailLicense.store_name || '—' }}</div>
          <div><strong>Hết hạn:</strong> {{ detailLicense.expires_at?.slice(0,10) || '—' }}</div>
          <div><strong>Kích hoạt:</strong> {{ detailLicense.activated_at?.slice(0,10) || '—' }}</div>
          <div><strong>Còn lại:</strong> {{ remainLabel(detailLicense) }}</div>
          <div><strong>HWID:</strong> {{ detailLicense.hwid?.slice(0,20) || '—' }}</div>
        </div>
        <h4 style="margin-top:20px;">💰 Lịch sử thanh toán</h4>
        <table class="data-table" v-if="detailPayments.length > 0">
          <thead><tr><th>Ngày</th><th>Số tiền</th><th>PT</th><th>Ghi chú</th></tr></thead>
          <tbody>
            <tr v-for="p in detailPayments" :key="p.id">
              <td>{{ p.created_at?.slice(0,10) }}</td>
              <td class="mono">{{ fmtMoney(p.amount) }}</td>
              <td>{{ methodLabel(p.payment_method) }}</td>
              <td>{{ p.note || '—' }}</td>
            </tr>
          </tbody>
        </table>
        <p v-else class="empty">Chưa có thanh toán</p>
        <div class="detail-actions">
          <button class="btn btn-primary" @click="showPayment = detailLicense; detailLicense = null">+ Ghi nhận thanh toán</button>
          <button class="btn-sm blue" @click="showExtend = detailLicense; detailLicense = null">📅 Gia hạn</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'License Manager — Admin' })
const { fetchApi } = useAuth()
const licenses = ref([])
const stats = ref({ total: 0, active: 0, expired: 0, revoked: 0, pending: 0 })
const showCreate = ref(false)
const newType = ref('MONTHLY')
const newNote = ref('')
const created = ref('')
const createError = ref('')
const creating = ref(false)

// Extend
const showExtend = ref(null)
const extendDays = ref(30)
const extending = ref(false)

// Payment
const showPayment = ref(null)
const payAmount = ref(200000)
const payMethod = ref('BANK_TRANSFER')
const payNote = ref('')
const payError = ref('')
const paySuccess = ref(false)
const paying = ref(false)

// Detail
const detailLicense = ref(null)
const detailPayments = ref([])

const newExpiry = computed(() => {
  if (!showExtend.value) return ''
  const base = showExtend.value.expires_at ? new Date(showExtend.value.expires_at) : new Date()
  const now = new Date()
  const start = base > now ? base : now
  start.setDate(start.getDate() + extendDays.value)
  return start.toISOString().slice(0,10)
})

async function load() {
  try {
    const r = await fetchApi('/api/admin/licenses')
    licenses.value = r.licenses
    stats.value = { total: r.total, active: r.active, expired: r.expired, revoked: r.revoked, pending: r.pending }
  } catch(e) { console.error(e) }
}

async function create() {
  createError.value = ''; creating.value = true
  try {
    const r = await fetchApi('/api/admin/licenses', { method: 'POST', body: { license_type: newType.value, note: newNote.value || undefined } })
    created.value = r.license_key; newNote.value = ''; load()
  } catch(e) { createError.value = e?.data?.message || 'Lỗi tạo license'; console.error(e) }
  finally { creating.value = false }
}

async function doAction(id, act) {
  const msgs = { revoke: 'Thu hồi key này? Cửa hàng sẽ không thể sử dụng.', reset_hwid: 'Reset HWID? Cửa hàng cần đăng nhập lại.' }
  if (msgs[act] && !confirm(msgs[act])) return
  try { await fetchApi(`/api/admin/licenses/${id}`, { method: 'PUT', body: { action: act } }); load() } catch(e) { console.error(e) }
}

async function doExtend() {
  extending.value = true
  try {
    await fetchApi(`/api/admin/licenses/${showExtend.value.id}`, { method: 'PUT', body: { action: 'extend', extend_days: extendDays.value } })
    showExtend.value = null; load()
  } catch(e) { console.error(e) }
  finally { extending.value = false }
}

async function doPayment() {
  payError.value = ''; paySuccess.value = false; paying.value = true
  try {
    await fetchApi(`/api/admin/licenses/${showPayment.value.id}/payments`, { method: 'POST', body: { amount: payAmount.value, payment_method: payMethod.value, note: payNote.value || undefined } })
    paySuccess.value = true; payNote.value = ''; load()
    setTimeout(() => { showPayment.value = null; paySuccess.value = false }, 1200)
  } catch(e) { payError.value = e?.data?.message || 'Lỗi ghi nhận'; console.error(e) }
  finally { paying.value = false }
}

async function openDetail(l) {
  detailLicense.value = l
  try { const r = await fetchApi(`/api/admin/licenses/${l.id}/payments`); detailPayments.value = r.payments } catch(e) { detailPayments.value = []; console.error(e) }
}

function statusLabel(s) { return { ACTIVE:'✅ ACTIVE', EXPIRING: '⚠️ SẮP HẾT', EXPIRED: '❌ HẾT HẠN', REVOKED: '🚫 REVOKED', PENDING: '⏳ PENDING' }[s] || s }
function typeLabel(t) { return { MONTHLY:'Tháng', YEARLY:'Năm', TRIAL:'Trial', lifetime:'Lifetime' }[t] || t }
function methodLabel(m) { return { BANK_TRANSFER:'Chuyển khoản', CASH:'Tiền mặt', MOMO:'MoMo' }[m] || m }
function remainLabel(l) { if (l.remaining_days == null) return '∞'; const d = l.remaining_days; return d >= 0 ? `${d} ngày` : `${d} ngày` }
function remainClass(l) { if (l.remaining_days == null) return ''; if (l.remaining_days < 0) return 'text-red'; if (l.remaining_days < 7) return 'text-amber'; return '' }
function rowClass(l) { return { 'row-expired': l.status==='EXPIRED', 'row-expiring': l.status==='EXPIRING', 'row-revoked': l.status==='REVOKED' } }
function fmtMoney(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }

onMounted(load)
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.stats-bar { display: flex; gap: 8px; margin-bottom: 20px; flex-wrap: wrap; }
.stat-pill { background: #F1F5F9; padding: 6px 16px; border-radius: 999px; font-size: 0.85rem; color: #64748B; }
.stat-pill .pill-n { font-weight: 800; margin-right: 4px; color: #1E293B; }
.stat-pill.active { background: #DCFCE7; color: #166534; }
.stat-pill.active .pill-n { color: #166534; }
.stat-pill.warn { background: #FEF3C7; color: #92400E; }
.stat-pill.warn .pill-n { color: #92400E; }
.stat-pill.danger { background: #FEE2E2; color: #991B1B; }
.stat-pill.danger .pill-n { color: #991B1B; }
.stat-pill.muted { background: #F1F5F9; color: #94A3B8; }

.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.85rem; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; }
.data-table tbody tr { cursor: pointer; transition: background 0.15s; }
.data-table tbody tr:hover { background: #F8FAFC; }
.mono { font-family: monospace; font-size: 0.85rem; }

.badge { padding: 2px 10px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; white-space: nowrap; }
.badge.ACTIVE { background: #DCFCE7; color: #166534; }
.badge.EXPIRING { background: #FEF3C7; color: #92400E; }
.badge.EXPIRED { background: #FEE2E2; color: #991B1B; }
.badge.PENDING { background: #E0E7FF; color: #3730A3; }
.badge.REVOKED { background: #F1F5F9; color: #64748B; text-decoration: line-through; }

.row-expired { background: #FFF1F2; }
.row-expiring { background: #FFFBEB; }
.row-revoked { opacity: 0.6; }
.text-red { color: #EF4444; font-weight: 700; }
.text-amber { color: #F59E0B; font-weight: 700; }
.pay-ok { font-size: 1rem; }
.pay-no { font-size: 1rem; opacity: 0.5; }

.actions { display: flex; gap: 4px; }
.btn-sm { padding: 4px 10px; border: 1px solid #E2E8F0; border-radius: 6px; background: white; cursor: pointer; font-size: 0.8rem; transition: all 0.15s; }
.btn-sm:hover { background: #F1F5F9; }
.btn-sm.red { color: #EF4444; border-color: #FECACA; }
.btn-sm.green { color: #10B981; border-color: #A7F3D0; }
.btn-sm.blue { color: #2563EB; border-color: #BFDBFE; }
.empty { text-align: center; color: #94A3B8; padding: 32px !important; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }

.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 200; display: flex; align-items: center; justify-content: center; }
.modal { max-width: 420px; width: 90%; padding: 32px; }
.form-group { margin-bottom: 16px; }
.form-group label { display: block; font-weight: 600; margin-bottom: 6px; }
.form-group input, .form-group select { width: 100%; padding: 10px 14px; border: 1px solid #E2E8F0; border-radius: 8px; font-family: inherit; font-size: 1rem; }
.success { color: #10B981; font-weight: 600; margin-bottom: 12px; font-family: monospace; }
.error-msg { color: #EF4444; font-weight: 600; margin-bottom: 12px; }
.duration-info { background: #F0F9FF; border: 1px solid #BAE6FD; border-radius: 8px; padding: 10px 14px; margin-bottom: 16px; color: #0369A1; font-size: 0.9rem; }

.detail-panel { max-width: 560px; width: 95%; padding: 32px; max-height: 80vh; overflow-y: auto; }
.detail-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
.detail-header h3 { font-family: monospace; font-size: 1.1rem; }
.detail-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; font-size: 0.9rem; }
.detail-actions { display: flex; gap: 8px; margin-top: 20px; }

@media (max-width: 768px) {
  .actions { flex-wrap: wrap; }
  .stats-bar { gap: 4px; }
  .detail-grid { grid-template-columns: 1fr; }
}
</style>
