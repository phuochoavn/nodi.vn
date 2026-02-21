<template>
  <div>
    <h2 class="page-title">💾 Backup</h2>
    <div class="card">
      <h3>Danh sách bản sao lưu</h3>
      <table class="data-table">
        <thead><tr><th>Tên file</th><th>Kích thước</th><th>Ngày tạo</th><th></th></tr></thead>
        <tbody>
          <tr v-for="b in backups" :key="b.id">
            <td>{{ b.filename }}</td>
            <td>{{ (b.size_bytes / 1024).toFixed(1) }} KB</td>
            <td>{{ b.created_at?.slice(0, 19) }}</td>
            <td><button class="btn-sm" @click="download">⬇️ Tải</button></td>
          </tr>
          <tr v-if="backups.length === 0"><td colspan="4" class="empty">Chưa có backup. Hãy backup từ ứng dụng POS.</td></tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Backup — Dashboard' })

const { fetchApi, token } = useAuth()
const backups = ref([])

onMounted(async () => {
  try {
    const r = await fetchApi('/api/backup/list')
    backups.value = r.backups || []
  } catch (e) { console.error(e) }
})

async function download() {
  window.open(`/api/backup/download?token=${token.value}`, '_blank')
}
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.card h3 { font-size: 1.1rem; font-weight: 700; margin-bottom: 16px; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.85rem; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; }
.empty { text-align: center; color: #94A3B8; padding: 32px !important; }
.btn-sm { padding: 6px 14px; border: 1px solid #E2E8F0; border-radius: 8px; background: white; cursor: pointer; font-size: 0.85rem; }
.btn-sm:hover { background: #F1F5F9; }
</style>
