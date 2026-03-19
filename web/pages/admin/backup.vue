<template>
  <div>
    <h2 class="page-title">💾 Backup Management</h2>
    <div class="summary card" style="margin-bottom:16px;">
      📁 {{ totalFiles }} files | 💾 {{ totalSize.toFixed(1) }} MB
    </div>
    <div class="table-wrap card">
      <table class="data-table">
        <thead><tr><th>Cửa hàng</th><th>File</th><th>Kích thước</th><th>Ngày tạo</th></tr></thead>
        <tbody>
          <tr v-for="b in backups" :key="b.id">
            <td>{{ b.store_name || 'Store #'+b.store_id }}</td>
            <td class="mono">{{ b.filename }}</td>
            <td>{{ (b.size_bytes/1024).toFixed(1) }} KB</td>
            <td>{{ fmtDateTime(b.created_at) }}</td>
          </tr>
          <tr v-if="backups.length===0"><td colspan="4" class="empty">Chưa có backup</td></tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Backup — Admin' })
const { fetchApi } = useAuth()
const { fmtDateTime } = await import('~/utils/date')
const backups = ref([])
const totalSize = ref(0)
const totalFiles = ref(0)
onMounted(async () => { try { const r = await fetchApi('/api/admin/backups'); backups.value = r.backups; totalSize.value = r.total_size_mb; totalFiles.value = r.total_files } catch(e){console.error(e)} })
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.summary { font-size: 1.1rem; font-weight: 600; }
.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.85rem; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; }
.mono { font-family: monospace; font-size: 0.85rem; }
.empty { text-align: center; color: #94A3B8; padding: 32px !important; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
</style>
