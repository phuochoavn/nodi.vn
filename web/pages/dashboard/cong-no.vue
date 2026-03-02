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
.total-debt { margin-bottom: 16px; font-size: 1.1rem; color: #1E293B; }
.debt-amount { color: #EF4444; font-weight: 700; }
:root.dark .total-debt { color: #E2E8F0; }
</style>
