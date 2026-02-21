<template>
  <div>
    <h2 class="page-title">🖥️ Hệ thống</h2>
    <div class="stats-grid">
      <div class="stat-card"><div class="stat-icon">⏱️</div><div class="stat-info"><span class="stat-value">{{ uptime }}</span><span class="stat-label">Uptime</span></div></div>
      <div class="stat-card"><div class="stat-icon">🗄️</div><div class="stat-info"><span class="stat-value">{{ sys.db_size_mb?.toFixed(1) || 0 }} MB</span><span class="stat-label">Database size</span></div></div>
      <div class="stat-card"><div class="stat-icon">🐳</div><div class="stat-info"><span class="stat-value">{{ sys.containers || 4 }}</span><span class="stat-label">Containers</span></div></div>
      <div class="stat-card"><div class="stat-icon">🔧</div><div class="stat-info"><span class="stat-value">v{{ sys.api_version || '0.1.0' }}</span><span class="stat-label">API Version</span></div></div>
    </div>
    <div class="card" style="margin-top:24px;">
      <h3>Container Status</h3>
      <div v-for="c in containers" :key="c" class="container-row">
        <span class="c-dot">🟢</span> {{ c }}
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Hệ thống — Admin' })
const { fetchApi } = useAuth()
const sys = ref({})
const containers = ['nodi-web (Nuxt SSR)', 'nodi-api (Axum)', 'nodi-nginx (Reverse Proxy)', 'nodi-postgres (Database)']
const uptime = computed(() => {
  const s = sys.value.uptime_seconds || 0
  const h = Math.floor(s/3600); const m = Math.floor((s%3600)/60)
  return `${h}h ${m}m`
})
onMounted(async () => { try { sys.value = await fetchApi('/api/admin/system') } catch(e){console.error(e)} })
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.stats-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px; }
.stat-card { background: white; border-radius: 12px; padding: 24px; display: flex; align-items: center; gap: 16px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.stat-icon { font-size: 2rem; }
.stat-value { display: block; font-size: 1.3rem; font-weight: 800; color: #1E293B; }
.stat-label { font-size: 0.85rem; color: #64748B; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.card h3 { font-weight: 700; margin-bottom: 16px; }
.container-row { display: flex; align-items: center; gap: 8px; padding: 8px 0; border-bottom: 1px solid #F1F5F9; }
.c-dot { font-size: 0.8rem; }
@media (max-width: 1024px) { .stats-grid { grid-template-columns: repeat(2, 1fr); } }
@media (max-width: 768px) { .stats-grid { grid-template-columns: 1fr; } }
</style>
