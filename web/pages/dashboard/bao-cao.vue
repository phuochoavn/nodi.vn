<template>
  <div>
    <h2 class="page-title">📈 Báo cáo</h2>

    <div class="report-grid">
      <div class="card">
        <h3>📊 Doanh thu theo ngày</h3>
        <div v-if="revenueData.length" class="chart-container"><canvas ref="revChart"></canvas></div>
        <div v-else class="empty">Chưa có dữ liệu</div>
        <div v-if="revenueData.length" class="report-totals">
          <div>Tổng DT: <strong>{{ fmt(totals.total_revenue) }}</strong></div>
          <div>Tổng ĐH: <strong>{{ totals.total_orders }}</strong></div>
        </div>
      </div>
      <div class="card">
        <h3>🏆 Top sản phẩm bán chạy</h3>
        <div v-if="topProducts.length" class="chart-container"><canvas ref="topChart"></canvas></div>
        <div v-else class="empty">Chưa có dữ liệu</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Chart, registerables } from 'chart.js'
Chart.register(...registerables)

definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Báo cáo — Dashboard' })

const { fetchApi } = useAuth()
const revenueData = ref([])
const totals = ref({ total_revenue: 0, total_orders: 0 })
const topProducts = ref([])
const revChart = ref(null)
const topChart = ref(null)
let charts = {}

function destroyChart(name) { if (charts[name]) { charts[name].destroy(); delete charts[name] } }

onMounted(async () => {
  try {
    const r = await fetchApi('/api/dashboard/reports/revenue?period=daily')
    revenueData.value = r.data || []; totals.value = r
    await nextTick()
    if (revenueData.value.length && revChart.value) {
      destroyChart('rev')
      charts.rev = new Chart(revChart.value, {
        type: 'line',
        data: {
          labels: revenueData.value.map(d => d.date?.slice(5)),
          datasets: [{
            label: 'Doanh thu',
            data: revenueData.value.map(d => d.revenue),
            borderColor: '#2563EB',
            backgroundColor: 'rgba(37,99,235,0.1)',
            fill: true,
            tension: 0.3,
            pointRadius: 3,
          }]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          plugins: { legend: { display: false } },
          scales: { y: { beginAtZero: true, ticks: { callback: v => fmtShort(v) } } }
        }
      })
    }
  } catch (e) { console.error(e) }

  try {
    const r = await fetchApi('/api/dashboard/reports/top-products?limit=10')
    topProducts.value = r.products || []
    await nextTick()
    if (topProducts.value.length && topChart.value) {
      destroyChart('top')
      charts.top = new Chart(topChart.value, {
        type: 'bar',
        data: {
          labels: topProducts.value.map(p => p.name?.slice(0, 20)),
          datasets: [{
            label: 'Doanh thu',
            data: topProducts.value.map(p => p.revenue),
            backgroundColor: ['#2563EB', '#3B82F6', '#60A5FA', '#93C5FD', '#BFDBFE', '#10B981', '#34D399', '#F59E0B', '#FBBF24', '#F97316'],
            borderRadius: 6,
          }]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          indexAxis: 'y',
          plugins: { legend: { display: false } },
          scales: { x: { beginAtZero: true, ticks: { callback: v => fmtShort(v) } } }
        }
      })
    }
  } catch (e) { console.error(e) }
})

function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }
function fmtShort(v) {
  if (v >= 1e6) return (v / 1e6).toFixed(1) + 'M'
  if (v >= 1e3) return (v / 1e3).toFixed(0) + 'K'
  return v + 'đ'
}
</script>

<style scoped>
.report-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 24px; }
.chart-container { position: relative; height: 300px; }
.report-totals { display: flex; gap: 32px; margin-top: 16px; padding-top: 16px; border-top: 1px solid #E2E8F0; color: #1E293B; }

:root.dark .report-totals { border-color: #334155; color: #E2E8F0; }

@media (max-width: 768px) {
  .report-grid { grid-template-columns: 1fr; }
  .chart-container { height: 250px; }
}
</style>
