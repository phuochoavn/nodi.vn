<template>
  <div>
    <h2 class="page-title">📈 Báo cáo</h2>

    <div class="report-section card">
      <h3>Doanh thu theo ngày</h3>
      <div class="chart-area">
        <div v-for="d in revenueData" :key="d.date" class="bar-row">
          <span class="bar-label">{{ d.date.slice(5) }}</span>
          <div class="bar-track">
            <div class="bar-fill" :style="{ width: barWidth(d.revenue) }"></div>
          </div>
          <span class="bar-value">{{ fmtShort(d.revenue) }}</span>
        </div>
        <div v-if="revenueData.length === 0" class="empty">Chưa có dữ liệu</div>
      </div>
      <div class="report-totals">
        <div>Tổng DT: <strong>{{ fmt(totals.total_revenue) }}</strong></div>
        <div>Tổng ĐH: <strong>{{ totals.total_orders }}</strong></div>
      </div>
    </div>

    <div class="report-section card" style="margin-top:24px;">
      <h3>🏆 Top sản phẩm bán chạy</h3>
      <div class="top-list">
        <div v-for="(p, i) in topProducts" :key="i" class="top-item">
          <span class="rank">#{{ i + 1 }}</span>
          <span class="top-name">{{ p.name }}</span>
          <span class="top-qty">{{ p.quantity_sold }} sp</span>
          <span class="top-rev">{{ fmtShort(p.revenue) }}</span>
        </div>
        <div v-if="topProducts.length === 0" class="empty">Chưa có dữ liệu</div>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Báo cáo — Dashboard' })

const { fetchApi } = useAuth()
const revenueData = ref([])
const totals = ref({ total_revenue: 0, total_orders: 0 })
const topProducts = ref([])
const maxRevenue = ref(1)

onMounted(async () => {
  try {
    const r = await fetchApi('/api/dashboard/reports/revenue?period=daily')
    revenueData.value = r.data; totals.value = r
    maxRevenue.value = Math.max(...r.data.map(d => d.revenue), 1)
  } catch (e) { console.error(e) }
  try {
    const r = await fetchApi('/api/dashboard/reports/top-products?limit=10')
    topProducts.value = r.products
  } catch (e) { console.error(e) }
})

function barWidth(v) { return Math.max((v / maxRevenue.value) * 100, 2) + '%' }
function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }
function fmtShort(v) {
  if (v >= 1e6) return (v / 1e6).toFixed(1) + 'M'
  if (v >= 1e3) return (v / 1e3).toFixed(0) + 'K'
  return v + 'đ'
}
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.report-section h3 { font-size: 1.1rem; font-weight: 700; margin-bottom: 16px; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }

.bar-row { display: flex; align-items: center; gap: 12px; margin-bottom: 8px; }
.bar-label { width: 50px; font-size: 0.85rem; color: #64748B; text-align: right; }
.bar-track { flex: 1; height: 24px; background: #F1F5F9; border-radius: 6px; overflow: hidden; }
.bar-fill { height: 100%; background: linear-gradient(90deg, #2563EB, #3B82F6); border-radius: 6px; transition: width 0.5s; }
.bar-value { width: 60px; font-size: 0.85rem; font-weight: 600; text-align: right; }

.report-totals { display: flex; gap: 32px; margin-top: 16px; padding-top: 16px; border-top: 1px solid #E2E8F0; }

.top-item { display: flex; align-items: center; gap: 12px; padding: 10px 0; border-bottom: 1px solid #F1F5F9; }
.rank { font-weight: 800; color: #F59E0B; width: 30px; }
.top-name { flex: 1; font-weight: 500; }
.top-qty { color: #64748B; width: 60px; text-align: right; }
.top-rev { font-weight: 700; color: #2563EB; width: 80px; text-align: right; }

.empty { text-align: center; color: #94A3B8; padding: 32px; }
</style>
