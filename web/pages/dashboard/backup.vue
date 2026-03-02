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
            <td><button class="btn-sm" @click="download(b)">⬇️ Tải</button></td>
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

function download(b) {
  window.open(`/api/backup/download?id=${b.id}&token=${token.value}`, '_blank')
}
</script>

<style scoped>
/* All styles inherited from dashboard-global.css */
</style>
