<template>
  <div>
    <h2 class="page-title">🏪 Cửa hàng</h2>
    <div class="table-wrap card">
      <table class="data-table">
        <thead><tr><th>Cửa hàng</th><th>License</th><th>Gói</th><th>Trạng thái</th><th>Sync cuối</th><th>DT tổng</th></tr></thead>
        <tbody>
          <tr v-for="s in stores" :key="s.id" @click="openDetail(s)" class="clickable">
            <td><strong>{{ s.name || 'Store #' + s.id }}</strong></td>
            <td class="mono">{{ s.license_key?.slice(0,14) }}...</td>
            <td>{{ typeLabel(s.license_type) }}</td>
            <td><span class="badge" :class="s.status">{{ s.status }}</span></td>
            <td><span :class="syncClass(s.last_sync_at)">●</span> {{ syncLabel(s.last_sync_at) }}</td>
            <td class="mono">{{ fmtShort(s.revenue_total) }}</td>
          </tr>
          <tr v-if="stores.length===0"><td colspan="6" class="empty">Chưa có cửa hàng nào</td></tr>
        </tbody>
      </table>
    </div>

    <!-- Detail Modal -->
    <div v-if="detail" class="modal-overlay" @click.self="detail=null">
      <div class="detail-panel card">
        <h3>{{ detail.store?.name || '—' }}</h3>
        <div class="detail-info">
          <div>📞 {{ detail.store?.phone || '—' }}</div>
          <div>📍 {{ detail.store?.address || '—' }}</div>
          <div>🔑 {{ detail.license?.key }} — <span class="badge" :class="detail.license?.status">{{ detail.license?.status }}</span></div>
          <div v-if="detail.license?.expires_at">📅 Hết hạn: {{ detail.license.expires_at?.slice(0,10) }}</div>
          <div>🔄 Sync: {{ detail.sync?.last_synced_at?.slice(0,16) || 'Chưa bao giờ' }} ({{ detail.sync?.total_syncs }} lần)</div>
        </div>
        <div class="stats-grid">
          <div class="stat-card blue"><div class="stat-icon">📦</div><div class="stat-info"><span class="stat-value">{{ fmt(detail.stats?.total_products) }}</span><span class="stat-label">Sản phẩm</span></div></div>
          <div class="stat-card green"><div class="stat-icon">👥</div><div class="stat-info"><span class="stat-value">{{ fmt(detail.stats?.total_customers) }}</span><span class="stat-label">Khách hàng</span></div></div>
          <div class="stat-card purple"><div class="stat-icon">📋</div><div class="stat-info"><span class="stat-value">{{ fmt(detail.stats?.total_orders) }}</span><span class="stat-label">Đơn hàng</span></div></div>
          <div class="stat-card amber"><div class="stat-icon">💰</div><div class="stat-info"><span class="stat-value">{{ fmtShort(detail.stats?.total_revenue) }}</span><span class="stat-label">Doanh thu</span></div></div>
          <div class="stat-card red"><div class="stat-icon">📝</div><div class="stat-info"><span class="stat-value">{{ fmtShort(detail.stats?.total_customer_debt) }}</span><span class="stat-label">Nợ KH</span></div></div>
          <div class="stat-card teal"><div class="stat-icon">🏭</div><div class="stat-info"><span class="stat-value">{{ fmtShort(detail.stats?.total_supplier_debt) }}</span><span class="stat-label">Nợ NCC</span></div></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Cửa hàng — Admin' })
const { fetchApi } = useAuth()
const stores = ref([])
const detail = ref(null)

onMounted(async () => { try { const r = await fetchApi('/api/admin/stores'); stores.value = r.stores } catch(e) { console.error(e) } })

async function openDetail(s) {
  try { detail.value = await fetchApi(`/api/admin/stores/${s.id}`) } catch(e) { console.error(e) }
}

function typeLabel(t) { return { MONTHLY:'Tháng', YEARLY:'Năm', TRIAL:'Trial', lifetime:'Lifetime' }[t] || t }
function fmtShort(v) { if (v >= 1e9) return (v/1e9).toFixed(1)+'B'; if (v >= 1e6) return (v/1e6).toFixed(1)+'M'; return new Intl.NumberFormat('vi-VN').format(v||0)+'đ' }
function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) }
function syncClass(dt) { if (!dt) return 'sync-never'; const h = (Date.now() - new Date(dt).getTime())/36e5; return h < 1 ? 'sync-ok' : h < 24 ? 'sync-warn' : 'sync-old' }
function syncLabel(dt) { if (!dt) return 'Chưa bao giờ'; const h = Math.floor((Date.now() - new Date(dt).getTime())/36e5); if (h < 1) return 'Vừa xong'; if (h < 24) return `${h} giờ trước`; return `${Math.floor(h/24)} ngày trước` }
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.85rem; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; }
.clickable { cursor: pointer; transition: background 0.15s; }
.clickable:hover { background: #F8FAFC; }
.mono { font-family: monospace; font-size: 0.85rem; }
.badge { padding: 2px 10px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }
.badge.ACTIVE { background: #DCFCE7; color: #166534; }
.badge.PENDING { background: #FEF3C7; color: #92400E; }
.badge.SUSPENDED, .badge.REVOKED { background: #FEE2E2; color: #991B1B; }
.badge.EXPIRING { background: #FEF3C7; color: #92400E; }
.badge.EXPIRED { background: #FEE2E2; color: #991B1B; }
.sync-ok { color: #10B981; } .sync-warn { color: #F59E0B; } .sync-old { color: #EF4444; } .sync-never { color: #94A3B8; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.empty { text-align: center; color: #94A3B8; padding: 48px !important; }

.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 200; display: flex; align-items: center; justify-content: center; }
.detail-panel { max-width: 640px; width: 95%; padding: 32px; max-height: 80vh; overflow-y: auto; }
.detail-panel h3 { font-size: 1.3rem; font-weight: 800; margin-bottom: 16px; }
.detail-info { display: flex; flex-direction: column; gap: 8px; font-size: 0.9rem; color: #475569; margin-bottom: 24px; padding-bottom: 16px; border-bottom: 1px solid #F1F5F9; }
.stats-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 12px; }
.stat-card { background: #F8FAFC; border-radius: 10px; padding: 16px; display: flex; align-items: center; gap: 12px; border-left: 3px solid transparent; }
.stat-card.blue { border-left-color: #2563EB; } .stat-card.green { border-left-color: #10B981; }
.stat-card.purple { border-left-color: #7C3AED; } .stat-card.amber { border-left-color: #F59E0B; }
.stat-card.red { border-left-color: #EF4444; } .stat-card.teal { border-left-color: #14B8A6; }
.stat-icon { font-size: 1.5rem; }
.stat-value { display: block; font-size: 1.2rem; font-weight: 800; color: #1E293B; }
.stat-label { font-size: 0.8rem; color: #64748B; }

@media (max-width: 768px) { .stats-grid { grid-template-columns: repeat(2, 1fr); } }
</style>
