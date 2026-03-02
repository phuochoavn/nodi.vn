<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">📦 Tồn kho</h2>
      <button class="btn-export" @click="exportExcel" :disabled="exporting">
        {{ exporting ? '⏳ Đang xuất...' : '📥 Xuất Excel' }}
      </button>
    </div>
    <div class="filters">
      <input v-model="search" type="text" placeholder="🔍 Tìm sản phẩm..." @input="debounceLoad">
      <select v-model="filter" @change="load()">
        <option value="">Tất cả</option>
        <option value="low_stock">⚠️ Sắp hết</option>
      </select>
    </div>
    <div class="table-wrap card">
      <table class="data-table">
        <thead>
          <tr><th>Tên</th><th>Danh mục</th><th>Tồn kho</th><th>ĐVT</th><th>Giá bán</th><th>HSD</th></tr>
        </thead>
        <tbody>
          <tr v-for="p in products" :key="p.id">
            <td>{{ p.name }}</td>
            <td>{{ p.category }}</td>
            <td class="num">
              <span :class="{ 'low-stock': p.is_low_stock }">{{ p.stock_quantity }}</span>
            </td>
            <td>{{ p.base_unit }}</td>
            <td class="num">{{ fmt(p.sell_price) }}</td>
            <td>{{ p.expiry_date || '—' }}</td>
          </tr>
          <tr v-if="products.length === 0"><td colspan="6" class="empty">Chưa có sản phẩm</td></tr>
        </tbody>
      </table>
      <div class="pagination">
        <button :disabled="page <= 1" @click="page--; load()">← Trước</button>
        <span>{{ total }} sản phẩm | ⚠️ {{ lowCount }} sắp hết</span>
        <button :disabled="page * limit >= total" @click="page++; load()">Sau →</button>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Tồn kho — Dashboard' })

const { fetchApi, token } = useAuth()
const { success, error: showError } = useToast()
const products = ref([])
const total = ref(0)
const lowCount = ref(0)
const page = ref(1)
const limit = 50
const search = ref('')
const filter = ref('')
const exporting = ref(false)
let timer = null

async function load() {
  try {
    const r = await fetchApi(`/api/dashboard/inventory?page=${page.value}&limit=${limit}&search=${search.value}&filter=${filter.value}`)
    products.value = r.products; total.value = r.total; lowCount.value = r.low_stock_count
  } catch (e) { console.error(e) }
}

function debounceLoad() { clearTimeout(timer); timer = setTimeout(load, 300) }
onMounted(load)
function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }

async function exportExcel() {
  exporting.value = true
  try {
    const res = await fetch('/api/dashboard/inventory/export', {
      headers: { Authorization: `Bearer ${token.value}` }
    })
    if (!res.ok) throw new Error('Export failed')
    const blob = await res.blob()
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'ton-kho.xlsx'
    a.click()
    URL.revokeObjectURL(url)
    success('Đã xuất file Excel!')
  } catch (e) {
    showError('Không thể xuất Excel: ' + (e.message || 'Lỗi'))
  }
  exporting.value = false
}
</script>

<style scoped>
.low-stock { color: #EF4444; font-weight: 700; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; flex-wrap: wrap; gap: 12px; }
.btn-export {
  padding: 8px 18px; border: none; border-radius: 8px;
  background: linear-gradient(135deg, #10B981, #059669); color: white;
  font-weight: 600; font-size: 0.9rem; cursor: pointer;
  transition: transform 0.15s, box-shadow 0.15s;
}
.btn-export:hover { transform: translateY(-1px); box-shadow: 0 4px 12px rgba(16,185,129,0.3); }
.btn-export:disabled { opacity: 0.5; cursor: not-allowed; transform: none; box-shadow: none; }
:root.dark .btn-export { background: linear-gradient(135deg, #059669, #047857); }
</style>
