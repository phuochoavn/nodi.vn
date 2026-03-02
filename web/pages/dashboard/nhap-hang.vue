<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">📦 Nhập hàng</h2>
      <div class="search-box">
        <input v-model="search" type="text" placeholder="Tìm NCC..." @input="debouncedLoad" class="form-input" />
      </div>
    </div>

    <div class="card">
      <div v-if="loading" class="skeleton-wrap">
        <div v-for="i in 5" :key="i" class="skeleton-row"></div>
      </div>
      <div v-else-if="orders.length === 0" class="empty">📭 Chưa có phiếu nhập hàng</div>
      <div v-else class="table-wrap">
        <table class="data-table">
          <thead>
            <tr>
              <th>Mã</th>
              <th>Nhà cung cấp</th>
              <th>Tổng tiền</th>
              <th>Trạng thái</th>
              <th>Ngày nhập</th>
              <th>Ghi chú</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="o in orders" :key="o.id">
              <td class="mono">#{{ o.id }}</td>
              <td>{{ o.supplier_name || '—' }}</td>
              <td class="num">{{ fmt(o.total_amount) }}</td>
              <td><span class="badge" :class="statusClass(o.status)">{{ statusLabel(o.status) }}</span></td>
              <td>{{ o.import_date || o.created_at?.slice(0, 10) || '—' }}</td>
              <td class="note-cell">{{ o.note || '—' }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-if="total > limit" class="pagination">
        <button :disabled="page <= 1" @click="page--; loadData()" class="btn-page">← Trước</button>
        <span class="page-info">Trang {{ page }} / {{ Math.ceil(total / limit) }}</span>
        <button :disabled="page >= Math.ceil(total / limit)" @click="page++; loadData()" class="btn-page">Sau →</button>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Nhập hàng — Dashboard' })

const { fetchApi } = useAuth()
const orders = ref([])
const total = ref(0)
const page = ref(1)
const limit = 50
const search = ref('')
const loading = ref(true)

let debounceTimer = null
function debouncedLoad() {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => { page.value = 1; loadData() }, 300)
}

async function loadData() {
  loading.value = true
  try {
    const q = new URLSearchParams({ page: page.value, limit })
    if (search.value) q.set('search', search.value)
    const r = await fetchApi(`/api/dashboard/purchase-orders?${q}`)
    orders.value = r.purchase_orders || []
    total.value = r.total || 0
  } catch (e) { console.error(e) }
  loading.value = false
}

function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }

function statusClass(s) {
  if (!s) return 'default'
  const l = s.toLowerCase()
  if (l.includes('complete') || l.includes('done') || l === 'completed') return 'success'
  if (l.includes('cancel') || l === 'cancelled') return 'danger'
  if (l.includes('pending') || l === 'draft') return 'warn'
  return 'default'
}

function statusLabel(s) {
  if (!s) return 'Mới'
  const map = { completed: 'Hoàn thành', done: 'Hoàn thành', cancelled: 'Đã hủy', pending: 'Chờ xử lý', draft: 'Nháp' }
  return map[s.toLowerCase()] || s
}

onMounted(loadData)
</script>

<style scoped>
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; flex-wrap: wrap; gap: 12px; }
.search-box .form-input { padding: 8px 16px; border: 1px solid #E2E8F0; border-radius: 8px; font-size: 0.9rem; min-width: 200px; }
.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-size: 0.8rem; font-weight: 600; color: #64748B; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; font-size: 0.9rem; }
.data-table tr:hover { background: #F8FAFC; }
.num { text-align: right; font-variant-numeric: tabular-nums; font-weight: 600; }
.mono { font-family: monospace; color: #64748B; }
.note-cell { max-width: 200px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; color: #94A3B8; }

.badge { padding: 4px 10px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }
.badge.success { background: #DCFCE7; color: #166534; }
.badge.danger { background: #FEE2E2; color: #991B1B; }
.badge.warn { background: #FEF3C7; color: #92400E; }
.badge.default { background: #F1F5F9; color: #475569; }

.pagination { display: flex; align-items: center; justify-content: center; gap: 16px; margin-top: 16px; padding-top: 16px; border-top: 1px solid #E2E8F0; }
.btn-page { padding: 6px 14px; border: 1px solid #E2E8F0; border-radius: 6px; font-size: 0.85rem; cursor: pointer; background: white; }
.btn-page:disabled { opacity: 0.4; cursor: not-allowed; }
.page-info { font-size: 0.85rem; color: #64748B; }

.skeleton-wrap { display: flex; flex-direction: column; gap: 12px; }
.skeleton-row { height: 48px; background: linear-gradient(90deg, #F1F5F9 25%, #E2E8F0 50%, #F1F5F9 75%); background-size: 200% 100%; animation: shimmer 1.5s infinite; border-radius: 8px; }
@keyframes shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }

:root.dark .data-table th { color: #94A3B8; border-color: #334155; }
:root.dark .data-table td { border-color: #1E293B; }
:root.dark .data-table tr:hover { background: #1E293B; }
:root.dark .search-box .form-input { background: #1E293B; border-color: #334155; color: #E2E8F0; }
:root.dark .pagination { border-color: #334155; }
:root.dark .btn-page { background: #1E293B; border-color: #334155; color: #E2E8F0; }
</style>
