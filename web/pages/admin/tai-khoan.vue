<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">👤 Tài khoản</h2>
      <div class="header-actions">
        <select v-model="filter" class="filter-select" @change="load">
          <option value="all">Tất cả</option>
          <option value="active">Hoạt động</option>
          <option value="inactive">Vô hiệu</option>
        </select>
        <button class="btn-export" @click="exportCSV">📥 Xuất CSV</button>
      </div>
    </div>

    <div class="stats-row">
      <div class="stat-chip purple">🔢 {{ summary.total }} tổng</div>
      <div class="stat-chip green">✅ {{ summary.active }} active</div>
      <div class="stat-chip red">🚫 {{ summary.inactive }} inactive</div>
    </div>

    <div class="table-wrap card">
      <table class="data-table">
        <thead>
          <tr>
            <th>Username</th>
            <th>Tên hiển thị</th>
            <th>SĐT</th>
            <th>Stores</th>
            <th>HWID</th>
            <th>Trạng thái</th>
            <th>Ngày tạo</th>
            <th>Thao tác</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="a in accounts" :key="a.id" @click="selectedAccount = a" class="clickable">
            <td class="mono">{{ a.username }}</td>
            <td>{{ a.display_name || '—' }}</td>
            <td class="mono">{{ a.phone || '—' }}</td>
            <td>
              <span class="stores-badge">{{ a.stores_count }}</span>
              <span v-if="a.store_names?.length" class="store-list">{{ a.store_names.join(', ') }}</span>
            </td>
            <td class="mono small">{{ a.hwid ? a.hwid.slice(0, 12) + '...' : '—' }}</td>
            <td><span class="badge" :class="a.is_active ? 'active' : 'inactive'">{{ a.is_active ? 'Active' : 'Inactive' }}</span></td>
            <td class="small">{{ fmtDateOnly(a.created_at) }}</td>
            <td @click.stop>
              <button class="btn-toggle" :class="a.is_active ? 'danger' : 'success'" @click="toggleAccount(a)">
                {{ a.is_active ? 'Vô hiệu' : 'Kích hoạt' }}
              </button>
            </td>
          </tr>
          <tr v-if="accounts.length === 0">
            <td colspan="8" class="empty">Chưa có tài khoản nào</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Detail modal -->
    <div v-if="selectedAccount" class="modal-overlay" @click.self="selectedAccount = null">
      <div class="detail-panel card">
        <div class="detail-header">
          <h3>{{ selectedAccount.username }}</h3>
          <button class="close-btn" @click="selectedAccount = null">✕</button>
        </div>
        <div class="detail-info">
          <div>👤 Tên: {{ selectedAccount.display_name || '—' }}</div>
          <div>📞 SĐT: {{ selectedAccount.phone || '—' }}</div>
          <div>🏪 Store ID: {{ selectedAccount.store_id }}</div>
          <div>💻 HWID: {{ selectedAccount.hwid || 'Chưa liên kết' }}</div>
          <div>📅 Tạo: {{ selectedAccount.created_at || '—' }}</div>
          <div>🔄 Cập nhật: {{ selectedAccount.updated_at || '—' }}</div>
        </div>
        <div v-if="selectedAccount.store_names?.length" class="stores-section">
          <h4>🏪 Cửa hàng ({{ selectedAccount.stores_count }})</h4>
          <div v-for="(name, i) in selectedAccount.store_names" :key="i" class="store-item">
            {{ i + 1 }}. {{ name }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Tài khoản — Admin' })
const { fetchApi } = useAuth()
const { fmtDateOnly } = await import('~/utils/date')
const accounts = ref([])
const filter = ref('all')
const selectedAccount = ref(null)
const summary = ref({ total: 0, active: 0, inactive: 0 })

async function load() {
  try {
    const params = filter.value !== 'all' ? `?status=${filter.value}` : ''
    const r = await fetchApi(`/api/admin/accounts${params}`)
    accounts.value = r.accounts || []
    summary.value = { total: r.total, active: r.active, inactive: r.inactive }
  } catch (e) { console.error(e) }
}

async function toggleAccount(a) {
  if (!confirm(`${a.is_active ? 'Vô hiệu hóa' : 'Kích hoạt'} tài khoản ${a.username}?`)) return
  try {
    await fetchApi(`/api/admin/accounts/${a.id}/toggle`, { method: 'PUT' })
    await load()
  } catch (e) { console.error(e) }
}

function exportCSV() {
  window.open('/api/admin/export/accounts')
}



onMounted(load)
</script>

<style scoped>
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; flex-wrap: wrap; gap: 12px; }
.page-title { font-size: 1.5rem; font-weight: 800; }
.header-actions { display: flex; gap: 8px; align-items: center; }
.filter-select { padding: 6px 12px; border: 1px solid #E2E8F0; border-radius: 8px; font-size: 0.85rem; background: white; }
.btn-export { padding: 6px 14px; border: 1px solid #E2E8F0; border-radius: 8px; background: white; cursor: pointer; font-size: 0.85rem; }
.btn-export:hover { background: #F8FAFC; }

.stats-row { display: flex; gap: 10px; margin-bottom: 20px; flex-wrap: wrap; }
.stat-chip { padding: 6px 14px; border-radius: 999px; font-size: 0.85rem; font-weight: 600; }
.stat-chip.purple { background: #EDE9FE; color: #6D28D9; }
.stat-chip.green { background: #DCFCE7; color: #166534; }
.stat-chip.red { background: #FEE2E2; color: #991B1B; }

.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.8rem; border-bottom: 2px solid #E2E8F0; white-space: nowrap; }
.data-table td { padding: 10px 12px; border-bottom: 1px solid #F1F5F9; font-size: 0.9rem; }
.clickable { cursor: pointer; transition: background 0.15s; }
.clickable:hover { background: #F8FAFC; }
.mono { font-family: monospace; font-size: 0.85rem; }
.small { font-size: 0.8rem; color: #64748B; }
.badge { padding: 2px 10px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }
.badge.active { background: #DCFCE7; color: #166534; }
.badge.inactive { background: #FEE2E2; color: #991B1B; }
.stores-badge { background: #EDE9FE; color: #6D28D9; padding: 2px 8px; border-radius: 6px; font-size: 0.8rem; font-weight: 700; margin-right: 6px; }
.store-list { font-size: 0.8rem; color: #64748B; }
.btn-toggle { padding: 4px 12px; border-radius: 6px; font-size: 0.8rem; font-weight: 600; cursor: pointer; border: none; }
.btn-toggle.danger { background: #FEE2E2; color: #991B1B; }
.btn-toggle.danger:hover { background: #FECACA; }
.btn-toggle.success { background: #DCFCE7; color: #166534; }
.btn-toggle.success:hover { background: #BBF7D0; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.empty { text-align: center; color: #94A3B8; padding: 48px !important; }

.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 200; display: flex; align-items: center; justify-content: center; }
.detail-panel { max-width: 560px; width: 95%; max-height: 80vh; overflow-y: auto; }
.detail-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
.detail-header h3 { font-size: 1.3rem; font-weight: 800; }
.close-btn { background: none; border: none; font-size: 1.2rem; cursor: pointer; color: #94A3B8; }
.detail-info { display: flex; flex-direction: column; gap: 8px; font-size: 0.9rem; color: #475569; padding-bottom: 16px; border-bottom: 1px solid #F1F5F9; }
.stores-section { margin-top: 16px; }
.stores-section h4 { font-weight: 700; margin-bottom: 8px; font-size: 0.95rem; }
.store-item { padding: 6px 12px; background: #F8FAFC; border-radius: 8px; margin-bottom: 4px; font-size: 0.85rem; }

:root.dark .card, .dark .card { background: #1E293B; }
:root.dark .data-table th, .dark .data-table th { color: #94A3B8; border-bottom-color: #334155; }
:root.dark .data-table td, .dark .data-table td { border-bottom-color: #334155; }
:root.dark .clickable:hover, .dark .clickable:hover { background: #334155; }
:root.dark .filter-select, .dark .filter-select { background: #334155; border-color: #475569; color: #E2E8F0; }
:root.dark .btn-export, .dark .btn-export { background: #334155; border-color: #475569; color: #E2E8F0; }
:root.dark .detail-info, .dark .detail-info { color: #CBD5E1; }
:root.dark .store-item, .dark .store-item { background: #334155; color: #E2E8F0; }
:root.dark .stat-value, .dark .stat-value { color: #F1F5F9; }
</style>
