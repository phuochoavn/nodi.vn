<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">📋 Nhật ký hoạt động</h2>
      <div class="header-actions">
        <select v-model="actionFilter" class="filter-select" @change="load">
          <option value="">Tất cả</option>
          <option value="ACCOUNT_TOGGLE">Toggle tài khoản</option>
          <option value="NOTIFICATION_CREATED">Gửi thông báo</option>
          <option value="EXPORT_CSV">Xuất CSV</option>
          <option value="LICENSE_CREATED">Tạo license</option>
          <option value="LICENSE_REVOKED">Thu hồi license</option>
        </select>
        <span class="total-badge">{{ total }} bản ghi</span>
      </div>
    </div>

    <div class="table-wrap card">
      <table class="data-table">
        <thead>
          <tr>
            <th>Thời gian</th>
            <th>Hành động</th>
            <th>Actor</th>
            <th>Target</th>
            <th>Chi tiết</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="log in logs" :key="log.id">
            <td class="small mono">{{ fmtTime(log.created_at) }}</td>
            <td><span class="action-badge" :class="actionClass(log.action)">{{ log.action }}</span></td>
            <td>{{ log.actor }}</td>
            <td class="mono small">{{ log.target_type ? `${log.target_type}#${log.target_id}` : '—' }}</td>
            <td class="small">{{ log.details || '—' }}</td>
          </tr>
          <tr v-if="logs.length === 0">
            <td colspan="5" class="empty">Chưa có nhật ký nào</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="total > logs.length" class="load-more">
      <button class="btn-load" @click="loadMore">Tải thêm</button>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Nhật ký — Admin' })
const { fetchApi } = useAuth()
const logs = ref([])
const total = ref(0)
const actionFilter = ref('')
const offset = ref(0)

async function load() {
  offset.value = 0
  try {
    const params = new URLSearchParams({ limit: '100', offset: '0' })
    if (actionFilter.value) params.set('action', actionFilter.value)
    const r = await fetchApi(`/api/admin/audit-log?${params}`)
    logs.value = r.logs || []
    total.value = r.total || 0
  } catch (e) { console.error(e) }
}

async function loadMore() {
  offset.value += 100
  try {
    const params = new URLSearchParams({ limit: '100', offset: String(offset.value) })
    if (actionFilter.value) params.set('action', actionFilter.value)
    const r = await fetchApi(`/api/admin/audit-log?${params}`)
    logs.value.push(...(r.logs || []))
  } catch (e) { console.error(e) }
}

function fmtTime(d) { return d ? d.replace('T', ' ').slice(0, 19) : '—' }
function actionClass(a) {
  if (a?.includes('TOGGLE')) return 'warning'
  if (a?.includes('CREATE') || a?.includes('NOTIFICATION')) return 'info'
  if (a?.includes('REVOKE') || a?.includes('DELETE')) return 'danger'
  if (a?.includes('EXPORT')) return 'neutral'
  return 'neutral'
}

onMounted(load)
</script>

<style scoped>
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; flex-wrap: wrap; gap: 12px; }
.page-title { font-size: 1.5rem; font-weight: 800; }
.header-actions { display: flex; gap: 10px; align-items: center; }
.filter-select { padding: 6px 12px; border: 1px solid #E2E8F0; border-radius: 8px; font-size: 0.85rem; background: white; }
.total-badge { background: #F1F5F9; color: #475569; padding: 4px 12px; border-radius: 999px; font-size: 0.8rem; font-weight: 600; }

.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.8rem; border-bottom: 2px solid #E2E8F0; white-space: nowrap; }
.data-table td { padding: 10px 12px; border-bottom: 1px solid #F1F5F9; font-size: 0.9rem; }
.mono { font-family: monospace; font-size: 0.85rem; }
.small { font-size: 0.8rem; color: #64748B; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.empty { text-align: center; color: #94A3B8; padding: 48px !important; }
.action-badge { padding: 2px 10px; border-radius: 6px; font-size: 0.75rem; font-weight: 600; font-family: monospace; }
.action-badge.warning { background: #FEF3C7; color: #92400E; }
.action-badge.info { background: #DBEAFE; color: #1E40AF; }
.action-badge.danger { background: #FEE2E2; color: #991B1B; }
.action-badge.neutral { background: #F1F5F9; color: #475569; }
.load-more { text-align: center; margin-top: 20px; }
.btn-load { padding: 8px 24px; border: 1px solid #E2E8F0; border-radius: 8px; background: white; cursor: pointer; font-size: 0.85rem; }
.btn-load:hover { background: #F8FAFC; }

:root.dark .card, .dark .card { background: #1E293B; }
:root.dark .data-table th, .dark .data-table th { color: #94A3B8; border-bottom-color: #334155; }
:root.dark .data-table td, .dark .data-table td { border-bottom-color: #334155; }
:root.dark .filter-select, .dark .filter-select { background: #334155; border-color: #475569; color: #E2E8F0; }
:root.dark .btn-load, .dark .btn-load { background: #334155; border-color: #475569; color: #E2E8F0; }
:root.dark .total-badge, .dark .total-badge { background: #334155; color: #CBD5E1; }
</style>
