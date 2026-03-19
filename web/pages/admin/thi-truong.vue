<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">📊 Market Intelligence</h2>
      <select v-model="months" class="month-filter" @change="loadTab">
        <option :value="3">3 tháng</option>
        <option :value="6">6 tháng</option>
        <option :value="12">12 tháng</option>
      </select>
    </div>

    <div class="tab-bar">
      <button v-for="t in tabs" :key="t.key" class="tab-btn" :class="{ active: tab === t.key }" @click="switchTab(t.key)">
        <span class="tab-icon">{{ t.icon }}</span><span class="tab-label">{{ t.label }}</span>
      </button>
    </div>

    <!-- TAB 1: Overview -->
    <div v-if="tab === 'overview'" class="tab-content">
      <div class="stats-grid">
        <div class="stat-card blue"><div class="stat-icon">💰</div><div class="stat-body"><div class="stat-value">{{ fmtCurrency(ov.total_revenue) }}</div><div class="stat-label">Tổng doanh thu</div></div></div>
        <div class="stat-card red"><div class="stat-icon">📋</div><div class="stat-body"><div class="stat-value">{{ fmtCurrency(ov.total_debt_customer + ov.total_debt_supplier) }}</div><div class="stat-label">Tổng nợ KH + NCC</div></div></div>
        <div class="stat-card green"><div class="stat-icon">📦</div><div class="stat-body"><div class="stat-value">{{ fmtNum(ov.total_products) }}</div><div class="stat-label">Sản phẩm đang bán</div></div></div>
        <div class="stat-card purple"><div class="stat-icon">🏪</div><div class="stat-body"><div class="stat-value">{{ fmtNum(ov.active_stores) }}</div><div class="stat-label">Cửa hàng hoạt động</div></div></div>
      </div>
      <div class="card" style="margin-top:24px;">
        <h3>📈 Doanh thu theo tháng</h3>
        <div v-if="ov.revenue_by_month?.length" class="chart-container"><canvas ref="revChart"></canvas></div>
        <div v-else class="empty-state">📭 Chưa có dữ liệu. Các cửa hàng cần sync để hiển thị phân tích.</div>
      </div>
    </div>

    <!-- TAB 2: Products -->
    <div v-if="tab === 'products'" class="tab-content">
      <div class="stats-grid stats-2">
        <div class="stat-card amber"><div class="stat-icon">📊</div><div class="stat-body"><div class="stat-value">{{ prod.avg_margin }}%</div><div class="stat-label">Biên lợi nhuận TB</div></div></div>
        <div class="stat-card blue"><div class="stat-icon">🏷️</div><div class="stat-body"><div class="stat-value">{{ prod.revenue_by_category?.length || 0 }}</div><div class="stat-label">Nhóm hàng</div></div></div>
      </div>
      <div class="two-col" style="margin-top:24px;">
        <div class="card">
          <h3>🔥 Top 10 SP bán chạy</h3>
          <div v-if="prod.top_products?.length">
            <div v-for="(p,i) in prod.top_products" :key="i" class="bar-row">
              <span class="rank">#{{ i+1 }}</span>
              <span class="bar-name">{{ p.name }}</span>
              <div class="bar-track"><div class="bar-fill purple" :style="{ width: barW(p.revenue, maxProdRev) }"></div></div>
              <span class="bar-value">{{ fmtShort(p.revenue) }}</span>
            </div>
          </div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
        <div class="card">
          <h3>🗂️ Doanh thu theo nhóm hàng</h3>
          <div v-if="prod.revenue_by_category?.length" class="chart-container"><canvas ref="catChart"></canvas></div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
      </div>
      <div class="card" style="margin-top:24px;">
        <h3>🏭 Top nhà sản xuất</h3>
        <div v-if="prod.top_manufacturers?.length">
          <div v-for="(m,i) in prod.top_manufacturers" :key="i" class="bar-row">
            <span class="rank">#{{ i+1 }}</span>
            <span class="bar-name">{{ m.name }}</span>
            <div class="bar-track"><div class="bar-fill blue" :style="{ width: barW(m.revenue, maxMfrRev) }"></div></div>
            <span class="bar-value">{{ fmtShort(m.revenue) }}</span>
            <span class="bar-sub">{{ m.products_count }} SP</span>
          </div>
        </div>
        <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
      </div>
    </div>

    <!-- TAB 3: Supply Chain -->
    <div v-if="tab === 'supply'" class="tab-content">
      <div class="stats-grid stats-2">
        <div class="stat-card red"><div class="stat-icon">💳</div><div class="stat-body"><div class="stat-value">{{ fmtCurrency(sc.total_supplier_debt) }}</div><div class="stat-label">Tổng nợ NCC</div></div></div>
        <div class="stat-card blue"><div class="stat-icon">🏢</div><div class="stat-body"><div class="stat-value">{{ sc.top_suppliers?.length || 0 }}</div><div class="stat-label">Nhà cung cấp</div></div></div>
      </div>
      <div class="two-col" style="margin-top:24px;">
        <div class="card">
          <h3>🔗 Kênh phân phối</h3>
          <div v-if="sc.channel_distribution?.length" class="chart-container"><canvas ref="channelChart"></canvas></div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
        <div class="card">
          <h3>💸 Chi trả NCC theo tháng</h3>
          <div v-if="sc.supplier_payments_by_month?.length" class="chart-container"><canvas ref="supPayChart"></canvas></div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
      </div>
      <div class="card" style="margin-top:24px;">
        <h3>🏆 Top NCC theo doanh số nhập</h3>
        <div v-if="sc.top_suppliers?.length" class="table-wrap">
          <table class="data-table">
            <thead><tr><th>NCC</th><th>Công ty</th><th>Tổng nhập</th><th>Số phiếu</th><th>Nợ hiện tại</th></tr></thead>
            <tbody>
              <tr v-for="s in sc.top_suppliers" :key="s.name">
                <td>{{ s.name }}</td><td>{{ s.company || '—' }}</td>
                <td class="num">{{ fmtCurrency(s.total_purchase) }}</td><td class="num">{{ s.orders }}</td>
                <td class="num" :class="{ 'text-red': s.current_debt > 0 }">{{ fmtCurrency(s.current_debt) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
      </div>
    </div>

    <!-- TAB 4: Credit -->
    <div v-if="tab === 'credit'" class="tab-content">
      <div class="stats-grid stats-3">
        <div class="stat-card red"><div class="stat-icon">🧾</div><div class="stat-body"><div class="stat-value">{{ fmtCurrency(cr.total_customer_debt) }}</div><div class="stat-label">Tổng nợ KH</div></div></div>
        <div class="stat-card green"><div class="stat-icon">📥</div><div class="stat-body"><div class="stat-value">{{ cr.collection_rate }}%</div><div class="stat-label">Tỷ lệ thu hồi</div></div></div>
        <div class="stat-card blue"><div class="stat-icon">👥</div><div class="stat-body"><div class="stat-value">{{ cr.top_debtors?.length || 0 }}</div><div class="stat-label">KH đang nợ</div></div></div>
      </div>
      <div class="two-col" style="margin-top:24px;">
        <div class="card">
          <h3>🌾 Nợ theo mùa vụ</h3>
          <div v-if="cr.debt_by_season?.length" class="chart-container"><canvas ref="seasonChart"></canvas></div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
        <div class="card">
          <h3>💳 Phương thức thanh toán</h3>
          <div v-if="cr.payment_methods?.length" class="chart-container"><canvas ref="payMethodChart"></canvas></div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
      </div>
      <div class="card" style="margin-top:24px;">
        <h3>📋 Top KH nợ nhiều nhất</h3>
        <div v-if="cr.top_debtors?.length" class="table-wrap">
          <table class="data-table">
            <thead><tr><th>Khách hàng</th><th>SĐT</th><th>Nợ hiện tại</th><th>Hạn mức</th><th>% sử dụng</th></tr></thead>
            <tbody>
              <tr v-for="d in cr.top_debtors" :key="d.name">
                <td>{{ d.name }}</td><td>{{ d.phone || '—' }}</td>
                <td class="num text-red">{{ fmtCurrency(d.debt) }}</td>
                <td class="num">{{ d.credit_limit ? fmtCurrency(d.credit_limit) : '—' }}</td>
                <td class="num"><span v-if="d.usage_pct > 0" class="badge" :class="d.usage_pct > 80 ? 'danger' : d.usage_pct > 50 ? 'warn' : 'ok'">{{ d.usage_pct }}%</span><span v-else>—</span></td>
              </tr>
            </tbody>
          </table>
        </div>
        <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
      </div>
    </div>

    <!-- TAB 5: Cash Flow -->
    <div v-if="tab === 'cashflow'" class="tab-content">
      <div class="stats-grid stats-3">
        <div class="stat-card green"><div class="stat-icon">📥</div><div class="stat-body"><div class="stat-value">{{ fmtCurrency(cf.total_in) }}</div><div class="stat-label">Tổng thu</div></div></div>
        <div class="stat-card red"><div class="stat-icon">📤</div><div class="stat-body"><div class="stat-value">{{ fmtCurrency(cf.total_out) }}</div><div class="stat-label">Tổng chi</div></div></div>
        <div class="stat-card" :class="cf.net_profit >= 0 ? 'blue' : 'red'"><div class="stat-icon">💎</div><div class="stat-body"><div class="stat-value">{{ fmtCurrency(cf.net_profit) }}</div><div class="stat-label">Lãi ròng</div></div></div>
      </div>
      <div class="two-col" style="margin-top:24px;">
        <div class="card">
          <h3>📊 Dòng tiền theo tháng</h3>
          <div v-if="cf.cashflow_by_month?.length" class="chart-container"><canvas ref="cfChart"></canvas></div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
        <div class="card">
          <h3>🧩 Cơ cấu chi</h3>
          <div v-if="cf.expense_breakdown?.length" class="chart-container"><canvas ref="expChart"></canvas></div>
          <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
        </div>
      </div>
      <div class="card" style="margin-top:24px;">
        <h3>🏦 Số dư quỹ cửa hàng</h3>
        <div v-if="cf.store_balances?.length" class="table-wrap">
          <table class="data-table">
            <thead><tr><th>Cửa hàng</th><th>Số dư</th><th>Cập nhật</th></tr></thead>
            <tbody>
              <tr v-for="b in cf.store_balances" :key="b.store_id">
                <td>{{ b.store_name || `Store #${b.store_id}` }}</td>
                <td class="num" :class="{ 'text-green': b.balance > 0, 'text-red': b.balance < 0 }">{{ fmtCurrency(b.balance) }}</td>
                <td>{{ fmtDateOnly(b.updated_at) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <div v-else class="empty-state">📭 Chưa có dữ liệu</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Chart, registerables } from 'chart.js'
Chart.register(...registerables)

definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Thị trường — Admin' })

const { fetchApi } = useAuth()
const { fmtDateOnly } = await import('~/utils/date')
const tab = ref('overview')
const months = ref(12)

const tabs = [
  { key: 'overview', icon: '📊', label: 'Tổng quan' },
  { key: 'products', icon: '📦', label: 'Sản phẩm' },
  { key: 'supply', icon: '🔗', label: 'Cung ứng' },
  { key: 'credit', icon: '💳', label: 'Tín dụng' },
  { key: 'cashflow', icon: '💰', label: 'Dòng tiền' },
]

// Data refs
const ov = ref({ total_revenue: 0, total_debt_customer: 0, total_debt_supplier: 0, total_products: 0, active_stores: 0, revenue_by_month: [] })
const prod = ref({ top_products: [], revenue_by_category: [], top_manufacturers: [], avg_margin: 0 })
const sc = ref({ channel_distribution: [], top_suppliers: [], total_supplier_debt: 0, supplier_payments_by_month: [] })
const cr = ref({ total_customer_debt: 0, debt_by_season: [], collection_rate: 0, top_debtors: [], payment_methods: [] })
const cf = ref({ total_in: 0, total_out: 0, net_profit: 0, cashflow_by_month: [], expense_breakdown: [], store_balances: [] })

// Chart refs
const revChart = ref(null)
const catChart = ref(null)
const channelChart = ref(null)
const supPayChart = ref(null)
const seasonChart = ref(null)
const payMethodChart = ref(null)
const cfChart = ref(null)
const expChart = ref(null)

// Chart instances
const charts = {}

const COLORS = ['#2563EB', '#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#EC4899', '#06B6D4', '#F97316']

function destroyChart(name) { if (charts[name]) { charts[name].destroy(); delete charts[name] } }

function createLineChart(ref, name, labels, datasets) {
  destroyChart(name)
  if (!ref) return
  charts[name] = new Chart(ref, {
    type: 'line',
    data: { labels, datasets },
    options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: datasets.length > 1 } },
      scales: { y: { beginAtZero: true, ticks: { callback: v => fmtShort(v) } } } }
  })
}

function createBarChart(ref, name, labels, data, colors) {
  destroyChart(name)
  if (!ref) return
  charts[name] = new Chart(ref, {
    type: 'bar',
    data: { labels, datasets: [{ data, backgroundColor: colors || COLORS.slice(0, data.length), borderRadius: 6 }] },
    options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } },
      scales: { y: { beginAtZero: true, ticks: { callback: v => fmtShort(v) } } } }
  })
}

function createDoughnut(ref, name, labels, data) {
  destroyChart(name)
  if (!ref) return
  charts[name] = new Chart(ref, {
    type: 'doughnut',
    data: { labels, datasets: [{ data, backgroundColor: COLORS.slice(0, data.length), borderWidth: 2, borderColor: '#fff' }] },
    options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'bottom', labels: { padding: 16, usePointStyle: true } } } }
  })
}

function createStackedBar(ref, name, labels, ds1, ds2) {
  destroyChart(name)
  if (!ref) return
  charts[name] = new Chart(ref, {
    type: 'bar',
    data: { labels, datasets: [
      { label: 'Thu', data: ds1, backgroundColor: '#10B981', borderRadius: 4 },
      { label: 'Chi', data: ds2, backgroundColor: '#EF4444', borderRadius: 4 },
    ] },
    options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'top' } },
      scales: { x: { stacked: true }, y: { stacked: true, beginAtZero: true, ticks: { callback: v => fmtShort(v) } } } }
  })
}

const maxProdRev = computed(() => Math.max(...(prod.value.top_products || []).map(p => p.revenue), 1))
const maxMfrRev = computed(() => Math.max(...(prod.value.top_manufacturers || []).map(m => m.revenue), 1))

function barW(v, max) { return Math.max((v / max) * 100, 2) + '%' }
function fmtCurrency(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }
function fmtNum(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) }
function fmtShort(v) { if (v >= 1e9) return (v / 1e9).toFixed(1) + 'B'; if (v >= 1e6) return (v / 1e6).toFixed(1) + 'M'; if (v >= 1e3) return (v / 1e3).toFixed(0) + 'K'; return v + '' }

async function loadTab() {
  const m = months.value
  try {
    if (tab.value === 'overview') {
      ov.value = await fetchApi(`/api/admin/market/overview?months=${m}`)
      await nextTick()
      if (ov.value.revenue_by_month?.length) {
        createLineChart(revChart.value, 'rev',
          ov.value.revenue_by_month.map(r => r.month),
          [{ label: 'Doanh thu', data: ov.value.revenue_by_month.map(r => r.revenue), borderColor: '#2563EB', backgroundColor: 'rgba(37,99,235,0.1)', fill: true, tension: 0.3 }]
        )
      }
    } else if (tab.value === 'products') {
      prod.value = await fetchApi(`/api/admin/market/products?months=${m}`)
      await nextTick()
      if (prod.value.revenue_by_category?.length) {
        createDoughnut(catChart.value, 'cat',
          prod.value.revenue_by_category.map(c => c.category),
          prod.value.revenue_by_category.map(c => c.revenue)
        )
      }
    } else if (tab.value === 'supply') {
      sc.value = await fetchApi(`/api/admin/market/supply-chain?months=${m}`)
      await nextTick()
      if (sc.value.channel_distribution?.length) {
        createDoughnut(channelChart.value, 'channel',
          sc.value.channel_distribution.map(c => c.channel),
          sc.value.channel_distribution.map(c => c.amount)
        )
      }
      if (sc.value.supplier_payments_by_month?.length) {
        createBarChart(supPayChart.value, 'supPay',
          sc.value.supplier_payments_by_month.map(p => p.month),
          sc.value.supplier_payments_by_month.map(p => p.amount),
          sc.value.supplier_payments_by_month.map(() => '#F59E0B')
        )
      }
    } else if (tab.value === 'credit') {
      cr.value = await fetchApi(`/api/admin/market/credit?months=${m}`)
      await nextTick()
      if (cr.value.debt_by_season?.length) {
        const labels = cr.value.debt_by_season.map(s => s.season)
        createBarChart(seasonChart.value, 'season', labels, cr.value.debt_by_season.map(s => s.debit), labels.map((_, i) => COLORS[i]))
      }
      if (cr.value.payment_methods?.length) {
        createDoughnut(payMethodChart.value, 'payMethod',
          cr.value.payment_methods.map(p => p.method),
          cr.value.payment_methods.map(p => p.amount)
        )
      }
    } else if (tab.value === 'cashflow') {
      cf.value = await fetchApi(`/api/admin/market/cashflow?months=${m}`)
      await nextTick()
      if (cf.value.cashflow_by_month?.length) {
        createStackedBar(cfChart.value, 'cf',
          cf.value.cashflow_by_month.map(c => c.month),
          cf.value.cashflow_by_month.map(c => c.income),
          cf.value.cashflow_by_month.map(c => c.expense)
        )
      }
      if (cf.value.expense_breakdown?.length) {
        createDoughnut(expChart.value, 'exp',
          cf.value.expense_breakdown.map(e => e.category),
          cf.value.expense_breakdown.map(e => e.amount)
        )
      }
    }
  } catch (e) { console.error('Market load error:', e) }
}

function switchTab(t) { tab.value = t; loadTab() }
onMounted(loadTab)
</script>

<style scoped>
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }
.page-title { font-size: 1.5rem; font-weight: 800; }
.month-filter { padding: 8px 16px; border: 1px solid #E2E8F0; border-radius: 8px; font-size: 0.9rem; background: white; cursor: pointer; }

.tab-bar { display: flex; gap: 4px; background: white; padding: 6px; border-radius: 12px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); margin-bottom: 24px; overflow-x: auto; }
.tab-btn {
  display: flex; align-items: center; gap: 6px; padding: 10px 18px; border-radius: 8px;
  border: none; background: transparent; cursor: pointer; font-size: 0.9rem; font-weight: 600;
  color: #64748B; transition: all 0.2s; white-space: nowrap; font-family: inherit;
}
.tab-btn:hover { background: #F1F5F9; color: #1E293B; }
.tab-btn.active { background: #7C3AED; color: white; }
.tab-icon { font-size: 1.1rem; }

.tab-content { animation: fadeIn 0.3s ease; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }

.stats-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
.stats-2 { grid-template-columns: repeat(2, 1fr); }
.stats-3 { grid-template-columns: repeat(3, 1fr); }

.stat-card {
  background: white; border-radius: 12px; padding: 20px; display: flex; align-items: center; gap: 16px;
  box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); border-left: 4px solid #E2E8F0;
}
.stat-card.blue { border-color: #2563EB; }
.stat-card.green { border-color: #10B981; }
.stat-card.red { border-color: #EF4444; }
.stat-card.purple { border-color: #7C3AED; }
.stat-card.amber { border-color: #F59E0B; }
.stat-icon { font-size: 1.8rem; }
.stat-value { font-size: 1.3rem; font-weight: 800; color: #1E293B; }
.stat-label { font-size: 0.8rem; color: #64748B; margin-top: 2px; }

.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.card h3 { font-size: 1.05rem; font-weight: 700; margin-bottom: 16px; }
.two-col { display: grid; grid-template-columns: 1fr 1fr; gap: 24px; }

.chart-container { position: relative; height: 300px; }

.bar-row { display: flex; align-items: center; gap: 10px; margin-bottom: 10px; }
.rank { font-weight: 800; color: #F59E0B; width: 30px; }
.bar-name { width: 140px; font-size: 0.85rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.bar-track { flex: 1; height: 22px; background: #F1F5F9; border-radius: 6px; overflow: hidden; }
.bar-fill { height: 100%; border-radius: 6px; transition: width 0.5s; }
.bar-fill.purple { background: linear-gradient(90deg, #7C3AED, #A78BFA); }
.bar-fill.blue { background: linear-gradient(90deg, #2563EB, #60A5FA); }
.bar-value { width: 60px; font-size: 0.85rem; font-weight: 600; text-align: right; }
.bar-sub { width: 50px; font-size: 0.75rem; color: #94A3B8; text-align: right; }

.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 10px 12px; font-size: 0.8rem; font-weight: 600; color: #64748B; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 10px 12px; border-bottom: 1px solid #F1F5F9; font-size: 0.9rem; }
.num { text-align: right; font-variant-numeric: tabular-nums; }
.text-red { color: #EF4444; }
.text-green { color: #10B981; }

.badge { padding: 2px 8px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }
.badge.ok { background: #DCFCE7; color: #166534; }
.badge.warn { background: #FEF3C7; color: #92400E; }
.badge.danger { background: #FEE2E2; color: #991B1B; }

.empty-state { text-align: center; padding: 48px 24px; color: #94A3B8; font-size: 0.95rem; }

@media (max-width: 1024px) { .stats-grid { grid-template-columns: repeat(2, 1fr); } }
@media (max-width: 768px) {
  .stats-grid, .stats-2, .stats-3 { grid-template-columns: 1fr; }
  .two-col { grid-template-columns: 1fr; }
  .bar-name { width: 100px; }
  .tab-btn { padding: 8px 12px; font-size: 0.8rem; }
  .tab-label { display: none; }
  .tab-icon { font-size: 1.3rem; }
}
</style>
