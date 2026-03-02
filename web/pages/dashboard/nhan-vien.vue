<template>
  <div>
    <h2 class="page-title">👥 Nhân viên</h2>

    <div class="card">
      <div v-if="loading" class="skeleton-wrap">
        <div v-for="i in 3" :key="i" class="skeleton-row"></div>
      </div>
      <div v-else-if="employees.length === 0" class="empty-state">
        <div class="empty-icon">👤</div>
        <p>Chưa có nhân viên nào</p>
        <p class="empty-sub">Thêm nhân viên từ ứng dụng Nodi POS để quản lý tại đây.</p>
      </div>
      <div v-else>
        <div class="stats-row">
          <div class="mini-stat">
            <div class="mini-value">{{ employees.length }}</div>
            <div class="mini-label">Tổng NV</div>
          </div>
          <div class="mini-stat">
            <div class="mini-value green">{{ employees.filter(e => e.is_active).length }}</div>
            <div class="mini-label">Đang làm</div>
          </div>
          <div class="mini-stat">
            <div class="mini-value red">{{ employees.filter(e => !e.is_active).length }}</div>
            <div class="mini-label">Nghỉ việc</div>
          </div>
        </div>
        <div class="table-wrap">
          <table class="data-table">
            <thead>
              <tr>
                <th>Tên</th>
                <th>SĐT</th>
                <th>Vai trò</th>
                <th>Trạng thái</th>
                <th>Ngày tạo</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="e in employees" :key="e.id" :class="{ inactive: !e.is_active }">
                <td class="name-cell">
                  <div class="avatar">{{ e.name?.charAt(0)?.toUpperCase() }}</div>
                  <span>{{ e.name }}</span>
                </td>
                <td>{{ e.phone || '—' }}</td>
                <td><span class="role-badge" :class="roleClass(e.role)">{{ roleLabel(e.role) }}</span></td>
                <td>
                  <span class="status-dot" :class="e.is_active ? 'active' : ''"></span>
                  {{ e.is_active ? 'Đang làm' : 'Nghỉ' }}
                </td>
                <td>{{ e.created_at?.slice(0, 10) || '—' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Nhân viên — Dashboard' })

const { fetchApi } = useAuth()
const employees = ref([])
const loading = ref(true)

onMounted(async () => {
  try {
    const r = await fetchApi('/api/dashboard/employees')
    employees.value = r.employees || []
  } catch (e) { console.error(e) }
  loading.value = false
})

function roleClass(role) {
  const map = { admin: 'admin', manager: 'manager', cashier: 'cashier' }
  return map[role?.toLowerCase()] || 'default'
}

function roleLabel(role) {
  const map = { admin: 'Quản trị', manager: 'Quản lý', cashier: 'Thu ngân', staff: 'Nhân viên' }
  return map[role?.toLowerCase()] || role || 'Nhân viên'
}
</script>

<style scoped>
.stats-row { display: flex; gap: 16px; margin-bottom: 20px; }
.mini-stat { flex: 1; text-align: center; padding: 12px; background: #F8FAFC; border-radius: 10px; }
.mini-value { font-size: 1.5rem; font-weight: 800; color: #1E293B; }
.mini-value.green { color: #10B981; }
.mini-value.red { color: #EF4444; }
.mini-label { font-size: 0.8rem; color: #64748B; margin-top: 2px; }

.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-size: 0.8rem; font-weight: 600; color: #64748B; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; font-size: 0.9rem; }
.data-table tr:hover { background: #F8FAFC; }
.data-table tr.inactive { opacity: 0.5; }

.name-cell { display: flex; align-items: center; gap: 10px; }
.avatar { width: 32px; height: 32px; border-radius: 50%; background: linear-gradient(135deg, #2563EB, #7C3AED); color: white; display: flex; align-items: center; justify-content: center; font-weight: 700; font-size: 0.85rem; flex-shrink: 0; }

.role-badge { padding: 4px 10px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }
.role-badge.admin { background: #EDE9FE; color: #6D28D9; }
.role-badge.manager { background: #DBEAFE; color: #1D4ED8; }
.role-badge.cashier { background: #DCFCE7; color: #166534; }
.role-badge.default { background: #F1F5F9; color: #475569; }

.status-dot { display: inline-block; width: 8px; height: 8px; border-radius: 50%; background: #CBD5E1; margin-right: 6px; }
.status-dot.active { background: #10B981; box-shadow: 0 0 6px rgba(16, 185, 129, 0.5); }

.empty-state { text-align: center; padding: 48px 24px; }
.empty-icon { font-size: 3rem; margin-bottom: 12px; }
.empty-state p { color: #64748B; font-size: 1rem; }
.empty-sub { font-size: 0.85rem; margin-top: 4px; }

.skeleton-wrap { display: flex; flex-direction: column; gap: 12px; }
.skeleton-row { height: 48px; background: linear-gradient(90deg, #F1F5F9 25%, #E2E8F0 50%, #F1F5F9 75%); background-size: 200% 100%; animation: shimmer 1.5s infinite; border-radius: 8px; }
@keyframes shimmer { 0% { background-position: 200% 0; } 100% { background-position: -200% 0; } }

:root.dark .mini-stat { background: #1E293B; }
:root.dark .mini-value { color: #E2E8F0; }
:root.dark .data-table th { color: #94A3B8; border-color: #334155; }
:root.dark .data-table td { border-color: #1E293B; }
:root.dark .data-table tr:hover { background: #1E293B; }
</style>
