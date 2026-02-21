<template>
  <div>
    <h2 class="page-title">📦 Tồn kho</h2>
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

const { fetchApi } = useAuth()
const products = ref([])
const total = ref(0)
const lowCount = ref(0)
const page = ref(1)
const limit = 50
const search = ref('')
const filter = ref('')
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
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.filters { display: flex; gap: 12px; margin-bottom: 16px; }
.filters input, .filters select {
  padding: 10px 16px; border: 1px solid #E2E8F0; border-radius: 10px; font-size: 0.95rem; font-family: inherit;
}
.filters input { flex: 1; }
.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.85rem; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; }
.num { text-align: right; font-variant-numeric: tabular-nums; }
.low-stock { color: #EF4444; font-weight: 700; }
.empty { text-align: center; color: #94A3B8; padding: 32px !important; }
.pagination { display: flex; justify-content: center; align-items: center; gap: 16px; padding: 16px; }
.pagination button { padding: 8px 16px; border: 1px solid #E2E8F0; border-radius: 8px; background: white; cursor: pointer; }
.pagination button:disabled { opacity: 0.4; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
</style>
