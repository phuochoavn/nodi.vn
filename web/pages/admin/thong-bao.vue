<template>
  <div>
    <div class="page-header">
      <h2 class="page-title">📢 Thông báo</h2>
      <button class="btn-new" @click="showForm = !showForm">{{ showForm ? '✕ Đóng' : '+ Tạo thông báo' }}</button>
    </div>

    <!-- Create form -->
    <div v-if="showForm" class="card form-card">
      <div class="form-group">
        <label>Tiêu đề</label>
        <input v-model="form.title" class="input" placeholder="Tiêu đề thông báo..." />
      </div>
      <div class="form-group">
        <label>Nội dung</label>
        <textarea v-model="form.message" class="input textarea" rows="3" placeholder="Nội dung chi tiết..."></textarea>
      </div>
      <div class="form-group">
        <label>Đối tượng</label>
        <select v-model="form.target_type" class="input">
          <option value="all">Tất cả người dùng</option>
          <option value="pro">Chỉ Pro</option>
          <option value="free">Chỉ Free</option>
          <option value="store">Cửa hàng cụ thể</option>
        </select>
      </div>
      <div v-if="form.target_type === 'store'" class="form-group">
        <label>Store ID</label>
        <input v-model="form.target_id" class="input" placeholder="Store ID..." />
      </div>
      <button class="btn-send" @click="send" :disabled="!form.title || !form.message">📤 Gửi thông báo</button>
    </div>

    <!-- History -->
    <div class="card" style="margin-top: 20px;">
      <h3 class="section-title">📜 Lịch sử thông báo</h3>
      <div v-for="n in notifications" :key="n.id" class="notif-item">
        <div class="notif-header">
          <span class="notif-title">{{ n.title }}</span>
          <span class="notif-target">{{ targetLabel(n.target_type) }}</span>
          <button class="btn-del" @click="remove(n.id)">🗑️</button>
        </div>
        <div class="notif-body">{{ n.message }}</div>
        <div class="notif-time">{{ fmtDateTime(n.created_at) }}</div>
      </div>
      <div v-if="notifications.length === 0" class="empty">Chưa có thông báo nào</div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Thông báo — Admin' })
const { fetchApi } = useAuth()
const { fmtDateTime } = await import('~/utils/date')
const notifications = ref([])
const showForm = ref(false)
const form = ref({ title: '', message: '', target_type: 'all', target_id: '' })

async function load() {
  try { const r = await fetchApi('/api/admin/notifications'); notifications.value = r.notifications || [] } catch(e) { console.error(e) }
}

async function send() {
  try {
    const body = { ...form.value }
    if (body.target_type !== 'store') body.target_id = null
    await fetchApi('/api/admin/notifications', { method: 'POST', body: JSON.stringify(body), headers: { 'Content-Type': 'application/json' } })
    form.value = { title: '', message: '', target_type: 'all', target_id: '' }
    showForm.value = false
    await load()
  } catch (e) { console.error(e) }
}

async function remove(id) {
  if (!confirm('Xóa thông báo này?')) return
  try { await fetchApi(`/api/admin/notifications/${id}`, { method: 'DELETE' }); await load() } catch(e) { console.error(e) }
}

function targetLabel(t) {
  return { all: '📣 Tất cả', pro: '⭐ Pro', free: '🆓 Free', store: '🏪 Cửa hàng' }[t] || t
}


onMounted(load)
</script>

<style scoped>
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
.page-title { font-size: 1.5rem; font-weight: 800; }
.btn-new { padding: 8px 18px; border-radius: 10px; border: none; background: #7C3AED; color: white; font-weight: 600; cursor: pointer; font-size: 0.9rem; }
.btn-new:hover { background: #6D28D9; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.form-card { border: 2px solid #EDE9FE; }
.form-group { margin-bottom: 14px; }
.form-group label { display: block; font-size: 0.85rem; font-weight: 600; color: #475569; margin-bottom: 4px; }
.input { width: 100%; padding: 8px 12px; border: 1px solid #E2E8F0; border-radius: 8px; font-size: 0.9rem; box-sizing: border-box; }
.textarea { resize: vertical; font-family: inherit; }
.btn-send { padding: 10px 24px; border-radius: 10px; border: none; background: #10B981; color: white; font-weight: 600; cursor: pointer; font-size: 0.9rem; }
.btn-send:hover { background: #059669; }
.btn-send:disabled { opacity: 0.5; cursor: not-allowed; }
.section-title { font-weight: 700; font-size: 1rem; margin-bottom: 16px; }
.notif-item { padding: 14px 0; border-bottom: 1px solid #F1F5F9; }
.notif-item:last-child { border-bottom: none; }
.notif-header { display: flex; align-items: center; gap: 10px; }
.notif-title { font-weight: 700; flex: 1; }
.notif-target { font-size: 0.8rem; padding: 2px 10px; border-radius: 999px; background: #F1F5F9; color: #64748B; }
.btn-del { background: none; border: none; cursor: pointer; font-size: 0.9rem; opacity: 0.5; }
.btn-del:hover { opacity: 1; }
.notif-body { font-size: 0.9rem; color: #475569; margin-top: 6px; line-height: 1.5; }
.notif-time { font-size: 0.8rem; color: #94A3B8; margin-top: 4px; }
.empty { text-align: center; color: #94A3B8; padding: 32px; }

:root.dark .card, .dark .card { background: #1E293B; }
:root.dark .input, .dark .input { background: #334155; border-color: #475569; color: #E2E8F0; }
:root.dark .notif-body, .dark .notif-body { color: #CBD5E1; }
:root.dark .notif-target, .dark .notif-target { background: #334155; color: #CBD5E1; }
:root.dark .notif-item, .dark .notif-item { border-bottom-color: #334155; }
:root.dark .form-card, .dark .form-card { border-color: #334155; }
</style>
