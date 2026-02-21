<template>
  <div>
    <h2 class="page-title">📊 Tổng quan</h2>

    <div class="stats-grid">
      <StatCard icon="💰" label="Doanh thu hôm nay" :value="data.revenue_today" format="currency" color="blue" />
      <StatCard icon="📋" label="Đơn hàng hôm nay" :value="data.orders_today" color="green" />
      <StatCard icon="⚠️" label="Sản phẩm sắp hết" :value="data.low_stock_count" color="amber" />
      <StatCard icon="💳" label="Công nợ khách hàng" :value="data.total_debt_customers" format="currency" color="red" />
    </div>

    <div class="stats-grid" style="margin-top:16px;">
      <StatCard icon="📦" label="Tổng sản phẩm" :value="data.total_products" color="blue" />
      <StatCard icon="👥" label="Tổng khách hàng" :value="data.total_customers" color="green" />
      <StatCard icon="🏭" label="Nợ nhà cung cấp" :value="data.total_debt_suppliers" format="currency" color="amber" />
      <StatCard icon="🏦" label="Quỹ cửa hàng" :value="data.store_balance" format="currency" color="blue" />
    </div>

    <div class="dashboard-grid">
      <div class="card">
        <h3>📈 Doanh thu tháng này</h3>
        <p class="big-number">{{ formatCurrency(data.revenue_this_month) }}</p>
        <p class="sub-stat">{{ data.orders_this_month }} đơn hàng</p>
      </div>
      <div class="card">
        <h3>🕐 Sync lần cuối</h3>
        <p class="sync-time">{{ data.last_sync_at || 'Chưa có dữ liệu' }}</p>
        <p v-if="data.last_sync_at" class="sync-ok">✅ Đã đồng bộ</p>
        <p v-else class="sync-warn">⚠️ Hãy sync từ ứng dụng</p>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Tổng quan — Dashboard' })

const { fetchApi } = useAuth()
const data = ref({
  revenue_today: 0, revenue_this_month: 0, orders_today: 0, orders_this_month: 0,
  total_products: 0, low_stock_count: 0, expiring_soon_count: 0,
  total_customers: 0, total_debt_customers: 0, total_debt_suppliers: 0,
  store_balance: 0, last_sync_at: null,
})

onMounted(async () => {
  try { data.value = await fetchApi('/api/dashboard/overview') } catch (e) { console.error(e) }
})

function formatCurrency(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.stats-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
.dashboard-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-top: 24px; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.card h3 { font-size: 1rem; font-weight: 700; margin-bottom: 12px; }
.big-number { font-size: 2rem; font-weight: 800; color: #2563EB; }
.sub-stat { color: #64748B; margin-top: 4px; }
.sync-time { font-size: 1.1rem; font-weight: 600; color: #1E293B; }
.sync-ok { color: #10B981; margin-top: 8px; }
.sync-warn { color: #F59E0B; margin-top: 8px; }
@media (max-width: 1024px) { .stats-grid { grid-template-columns: repeat(2, 1fr); } }
@media (max-width: 768px) {
  .stats-grid { grid-template-columns: 1fr; }
  .dashboard-grid { grid-template-columns: 1fr; }
}
</style>
