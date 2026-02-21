<template>
  <div>
    <h2 class="page-title">📊 Admin Overview</h2>

    <!-- Alerts -->
    <div v-if="alertList.length > 0" class="alerts-section">
      <div class="alerts-header">⚠️ {{ alertList.length }} cảnh báo</div>
      <div v-for="(a, i) in alertList" :key="i" class="alert-row" :class="a.severity">
        <span class="alert-icon">{{ a.severity==='error'?'❌':a.severity==='warning'?'⚠️':'ℹ️' }}</span>
        <span class="alert-msg">{{ a.message }}</span>
        <button v-if="a.type==='LICENSE_EXPIRING'||a.type==='LICENSE_EXPIRED'" class="btn-sm blue" @click="navigateTo('/admin/license')">Gia hạn</button>
        <button v-if="a.type==='STORE_INACTIVE'" class="btn-sm" @click="navigateTo('/admin/cua-hang')">Xem</button>
      </div>
    </div>

    <!-- Stats -->
    <div class="stats-grid">
      <div class="stat-card purple"><div class="stat-icon">🏪</div><div class="stat-info"><span class="stat-value">{{ d.active_stores }}/{{ d.total_stores }}</span><span class="stat-label">Cửa hàng hoạt động</span></div></div>
      <div class="stat-card amber"><div class="stat-icon">🔑</div><div class="stat-info"><span class="stat-value">{{ d.active_licenses }}/{{ d.total_licenses }}</span><span class="stat-label">License active</span></div></div>
      <div class="stat-card blue"><div class="stat-icon">💰</div><div class="stat-info"><span class="stat-value">{{ fmtShort(d.total_revenue_all_stores) }}</span><span class="stat-label">Doanh thu toàn hệ thống</span></div></div>
      <div class="stat-card green"><div class="stat-icon">📋</div><div class="stat-info"><span class="stat-value">{{ fmt(d.total_orders_all_stores) }}</span><span class="stat-label">Tổng đơn hàng</span></div></div>
    </div>
    <div class="stats-grid" style="margin-top:16px;">
      <div class="stat-card blue"><div class="stat-icon">📦</div><div class="stat-info"><span class="stat-value">{{ fmt(d.total_products_all_stores) }}</span><span class="stat-label">Tổng sản phẩm</span></div></div>
      <div class="stat-card green"><div class="stat-icon">👥</div><div class="stat-info"><span class="stat-value">{{ fmt(d.total_customers_all_stores) }}</span><span class="stat-label">Tổng khách hàng</span></div></div>
    </div>
    <div class="card" style="margin-top:24px;">
      <h3>🕐 Sync gần nhất</h3>
      <p v-if="d.last_sync?.store_name">{{ d.last_sync.store_name }} — {{ d.last_sync.synced_at }}</p>
      <p v-else class="empty">Chưa có store nào sync</p>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Admin Overview — Nodi POS' })
const { fetchApi } = useAuth()
const d = ref({ total_stores: 0, active_stores: 0, total_licenses: 0, active_licenses: 0, total_revenue_all_stores: 0, total_orders_all_stores: 0, total_products_all_stores: 0, total_customers_all_stores: 0, last_sync: null })
const alertList = ref([])

onMounted(async () => {
  try { d.value = await fetchApi('/api/admin/overview') } catch (e) { console.error(e) }
  try { const r = await fetchApi('/api/admin/alerts'); alertList.value = r.alerts } catch (e) { console.error(e) }
})
function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) }
function fmtShort(v) { if (v >= 1e9) return (v/1e9).toFixed(1)+'B'; if (v >= 1e6) return (v/1e6).toFixed(1)+'M'; return fmt(v)+'đ' }
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.alerts-section { background: white; border-radius: 12px; padding: 16px 20px; margin-bottom: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); border: 1px solid #FDE68A; }
.alerts-header { font-weight: 700; font-size: 0.95rem; color: #92400E; margin-bottom: 12px; }
.alert-row { display: flex; align-items: center; gap: 10px; padding: 8px 0; border-top: 1px solid #FEF3C7; }
.alert-row:first-of-type { border-top: none; }
.alert-icon { font-size: 1rem; flex-shrink: 0; }
.alert-msg { flex: 1; font-size: 0.9rem; color: #475569; }
.alert-row.error .alert-msg { color: #991B1B; }
.alert-row.warning .alert-msg { color: #92400E; }
.btn-sm { padding: 4px 12px; border: 1px solid #E2E8F0; border-radius: 6px; background: white; cursor: pointer; font-size: 0.8rem; white-space: nowrap; }
.btn-sm.blue { color: #2563EB; border-color: #BFDBFE; }

.stats-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
.stat-card { background: white; border-radius: 12px; padding: 24px; display: flex; align-items: center; gap: 16px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); border-left: 4px solid transparent; }
.stat-card.purple { border-left-color: #7C3AED; }
.stat-card.amber { border-left-color: #F59E0B; }
.stat-card.blue { border-left-color: #2563EB; }
.stat-card.green { border-left-color: #10B981; }
.stat-icon { font-size: 2rem; }
.stat-value { display: block; font-size: 1.5rem; font-weight: 800; color: #1E293B; }
.stat-label { font-size: 0.85rem; color: #64748B; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.card h3 { font-weight: 700; margin-bottom: 12px; }
.empty { color: #94A3B8; }
@media (max-width: 1024px) { .stats-grid { grid-template-columns: repeat(2, 1fr); } }
@media (max-width: 768px) { .stats-grid { grid-template-columns: 1fr; } }
</style>
