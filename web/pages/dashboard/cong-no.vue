<template>
  <div>
    <h2 class="page-title">💰 Công nợ</h2>
    <div class="tabs">
      <button :class="{ active: tab === 'customer' }" @click="tab = 'customer'; load()">👥 Khách hàng</button>
      <button :class="{ active: tab === 'supplier' }" @click="tab = 'supplier'; load()">🏭 Nhà cung cấp</button>
    </div>
    <div class="total-debt card">
      Tổng nợ: <strong>{{ fmt(totalDebt) }}</strong> ({{ count }} {{ tab === 'customer' ? 'khách' : 'NCC' }})
    </div>
    <div class="table-wrap card">
      <table class="data-table">
        <thead>
          <tr><th>Tên</th><th>SĐT</th><th>Số nợ</th><th v-if="tab==='customer'">Hạn mức</th><th v-else>Loại</th></tr>
        </thead>
        <tbody>
          <tr v-for="d in debts" :key="d.id">
            <td>{{ d.name }}</td>
            <td>{{ d.phone || '—' }}</td>
            <td class="num debt-amount">{{ fmt(d.current_debt) }}</td>
            <td v-if="tab==='customer'" class="num">{{ fmt(d.credit_limit) }}</td>
            <td v-else>{{ d.type }}</td>
          </tr>
          <tr v-if="debts.length === 0"><td :colspan="4" class="empty">Không có công nợ</td></tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Công nợ — Dashboard' })

const { fetchApi } = useAuth()
const tab = ref('customer')
const debts = ref([])
const totalDebt = ref(0)
const count = ref(0)

async function load() {
  try {
    const r = await fetchApi(`/api/dashboard/debts?type=${tab.value}`)
    debts.value = r.debts; totalDebt.value = r.total_debt; count.value = r.count
  } catch (e) { console.error(e) }
}

onMounted(load)
function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.tabs { display: flex; gap: 8px; margin-bottom: 16px; }
.tabs button {
  padding: 10px 24px; border: 1px solid #E2E8F0; border-radius: 10px;
  background: white; cursor: pointer; font-weight: 600; font-family: inherit; font-size: 0.95rem;
}
.tabs button.active { background: #2563EB; color: white; border-color: #2563EB; }
.total-debt { margin-bottom: 16px; font-size: 1.1rem; }
.debt-amount { color: #EF4444; font-weight: 700; }
.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.85rem; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; }
.num { text-align: right; font-variant-numeric: tabular-nums; }
.empty { text-align: center; color: #94A3B8; padding: 32px !important; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
</style>
