<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup name="toast">
        <div v-for="toast in toasts" :key="toast.id"
             class="toast-item" :class="[toast.type, { 'toast-exit': !toast.visible }]"
             @click="removeToast(toast.id)">
          <span class="toast-icon">{{ icons[toast.type] }}</span>
          <span class="toast-msg">{{ toast.message }}</span>
          <button class="toast-close" @click.stop="removeToast(toast.id)">✕</button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup>
const { toasts, removeToast } = useToast()
const icons = { success: '✅', error: '❌', warning: '⚠️', info: 'ℹ️' }
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 16px;
  right: 16px;
  z-index: 99999;
  display: flex;
  flex-direction: column;
  gap: 8px;
  pointer-events: none;
  max-width: 400px;
}

.toast-item {
  pointer-events: auto;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  border-radius: 10px;
  background: white;
  box-shadow: 0 8px 24px rgb(0 0 0 / 0.12), 0 2px 8px rgb(0 0 0 / 0.06);
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 500;
  color: #1E293B;
  border-left: 4px solid #94A3B8;
  animation: slideIn 0.3s ease;
}

.toast-item.success { border-color: #10B981; background: #F0FDF4; }
.toast-item.error { border-color: #EF4444; background: #FEF2F2; }
.toast-item.warning { border-color: #F59E0B; background: #FFFBEB; }
.toast-item.info { border-color: #3B82F6; background: #EFF6FF; }

.toast-icon { font-size: 1.1rem; flex-shrink: 0; }
.toast-msg { flex: 1; line-height: 1.4; }
.toast-close {
  background: none; border: none; cursor: pointer;
  font-size: 0.85rem; color: #94A3B8; padding: 2px 4px;
  border-radius: 4px; transition: background 0.15s;
}
.toast-close:hover { background: rgba(0,0,0,0.06); color: #475569; }

@keyframes slideIn { from { opacity: 0; transform: translateX(80px); } to { opacity: 1; transform: translateX(0); } }

.toast-exit { opacity: 0; transform: translateX(80px); transition: all 0.3s ease; }
.toast-enter-active { animation: slideIn 0.3s ease; }
.toast-leave-active { transition: all 0.3s ease; }
.toast-leave-to { opacity: 0; transform: translateX(80px); }

:root.dark .toast-item { background: #1E293B; color: #E2E8F0; box-shadow: 0 8px 24px rgb(0 0 0 / 0.3); }
:root.dark .toast-item.success { background: #064E3B; }
:root.dark .toast-item.error { background: #450A0A; }
:root.dark .toast-item.warning { background: #451A03; }
:root.dark .toast-item.info { background: #172554; }
:root.dark .toast-close { color: #64748B; }
:root.dark .toast-close:hover { background: rgba(255,255,255,0.08); color: #CBD5E1; }
</style>
