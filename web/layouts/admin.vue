<template>
  <div class="flex min-h-screen bg-slate-50 dark:bg-slate-900 transition-colors duration-300">
    <!-- Sidebar -->
    <aside class="fixed top-0 left-0 bottom-0 z-40 flex flex-col transition-all duration-300 border-r border-slate-200 dark:border-slate-700/50"
           :class="[
             collapsed ? 'w-[72px]' : 'w-60',
             sidebarOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
           ]"
           style="background: linear-gradient(180deg, #1a0a2e 0%, #2d1654 100%);">
      <!-- Logo -->
      <div class="flex items-center gap-2 px-4 py-5 border-b border-purple-900/50">
        <NuxtLink to="/admin" class="flex items-center gap-2 text-white font-extrabold text-lg overflow-hidden">
          <NodiLogo :size="32" />
          <Transition enter-active-class="transition-opacity duration-200" enter-from-class="opacity-0" enter-to-class="opacity-100"
                      leave-active-class="transition-opacity duration-200" leave-from-class="opacity-100" leave-to-class="opacity-0">
            <span v-if="!collapsed" class="whitespace-nowrap">
              Nodi<span class="text-amber-400">Admin</span>
            </span>
          </Transition>
        </NuxtLink>
      </div>

      <!-- Nav Items -->
      <nav class="flex-1 overflow-y-auto py-3 px-2 space-y-1">
        <NuxtLink v-for="item in navItems" :key="item.to" :to="item.to"
                  class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-purple-300/70 text-sm font-medium transition-all hover:bg-purple-800/30 hover:text-white"
                  active-class="!bg-purple-600 !text-white"
                  @click="sidebarOpen = false">
          <component :is="item.icon" :size="20" class="flex-shrink-0" />
          <Transition enter-active-class="transition-opacity duration-200" enter-from-class="opacity-0" enter-to-class="opacity-100"
                      leave-active-class="transition-opacity duration-200" leave-from-class="opacity-100" leave-to-class="opacity-0">
            <span v-if="!collapsed" class="whitespace-nowrap">{{ item.label }}</span>
          </Transition>
          <span v-if="item.badge && item.badge > 0 && !collapsed"
                class="ml-auto bg-red-500 text-white text-[10px] font-bold px-1.5 py-0.5 rounded-full min-w-[18px] text-center">
            {{ item.badge }}
          </span>
        </NuxtLink>

        <!-- Divider -->
        <div class="border-t border-purple-800/40 my-3"></div>

        <NuxtLink to="/dashboard"
                  class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-purple-300/70 text-sm font-medium hover:bg-purple-800/30 hover:text-white transition-colors"
                  @click="sidebarOpen = false">
          <ArrowLeft :size="20" class="flex-shrink-0" />
          <span v-if="!collapsed" class="whitespace-nowrap">User Dashboard</span>
        </NuxtLink>
      </nav>

      <!-- Sidebar Footer -->
      <div class="px-2 py-3 border-t border-purple-800/40 space-y-1">
        <button @click="collapsed = !collapsed"
                class="hidden md:flex w-full items-center gap-3 px-3 py-2.5 rounded-xl text-purple-400 text-sm hover:bg-purple-800/30 hover:text-white transition-colors">
          <PanelLeftClose v-if="!collapsed" :size="20" class="flex-shrink-0" />
          <PanelLeftOpen v-else :size="20" class="flex-shrink-0" />
          <span v-if="!collapsed" class="whitespace-nowrap">Thu gọn</span>
        </button>
        <button @click="logout"
                class="flex w-full items-center gap-3 px-3 py-2.5 rounded-xl text-purple-400 text-sm hover:bg-red-900/30 hover:text-red-400 transition-colors">
          <LogOut :size="20" class="flex-shrink-0" />
          <span v-if="!collapsed" class="whitespace-nowrap">Đăng xuất</span>
        </button>
      </div>
    </aside>

    <!-- Overlay -->
    <Transition enter-active-class="transition-opacity duration-300" enter-from-class="opacity-0" enter-to-class="opacity-100"
                leave-active-class="transition-opacity duration-200" leave-from-class="opacity-100" leave-to-class="opacity-0">
      <div v-if="sidebarOpen" class="fixed inset-0 z-30 bg-black/50 md:hidden" @click="sidebarOpen = false"></div>
    </Transition>

    <!-- Main Content -->
    <div class="flex-1 transition-all duration-300"
         :class="collapsed ? 'md:ml-[72px]' : 'md:ml-60'">
      <header class="sticky top-0 z-20 flex items-center justify-between px-4 md:px-8 py-3 bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl border-b border-slate-200 dark:border-slate-700/50">
        <div class="flex items-center gap-3">
          <button class="md:hidden p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-600 dark:text-slate-300"
                  @click="sidebarOpen = !sidebarOpen">
            <MenuIcon :size="20" />
          </button>
          <span class="bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300 text-xs font-bold px-3 py-1.5 rounded-full flex items-center gap-1.5">
            <Wrench :size="12" />
            Admin Panel
          </span>
        </div>
        <div class="flex items-center gap-2">
          <button @click="toggleDark"
                  class="p-2 rounded-lg text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors">
            <Sun v-if="$colorMode.value === 'dark'" :size="18" />
            <Moon v-else :size="18" />
          </button>
        </div>
      </header>
      <main class="p-4 md:p-8">
        <slot />
      </main>
    </div>
  </div>
</template>

<script setup>
import { LayoutDashboard, Key, Store, TrendingUp, MessageSquare, HardDrive, Server, ArrowLeft, LogOut, PanelLeftClose, PanelLeftOpen, Menu as MenuIcon, Wrench, Sun, Moon, Upload, Users, ClipboardList, Bell } from 'lucide-vue-next'

const { logout, fetchApi } = useAuth()
const colorMode = useColorMode()
const sidebarOpen = ref(false)
const collapsed = ref(false)
const supportUnread = ref(0)

const toggleDark = () => {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}

const navItems = computed(() => [
  { icon: LayoutDashboard, label: 'Tổng quan', to: '/admin' },
  { icon: Key, label: 'License', to: '/admin/license' },
  { icon: Store, label: 'Cửa hàng', to: '/admin/cua-hang' },
  { icon: Users, label: 'Tài khoản', to: '/admin/tai-khoan' },
  { icon: TrendingUp, label: 'Thị trường', to: '/admin/thi-truong' },
  { icon: MessageSquare, label: 'Hỗ trợ', to: '/admin/support', badge: supportUnread.value },
  { icon: Bell, label: 'Thông báo', to: '/admin/thong-bao' },
  { icon: ClipboardList, label: 'Nhật ký', to: '/admin/nhat-ky' },
  { icon: HardDrive, label: 'Backup', to: '/admin/backup' },
  { icon: Upload, label: 'Cập nhật', to: '/admin/cap-nhat' },
  { icon: Server, label: 'Hệ thống', to: '/admin/he-thong' },
])

async function loadUnread() {
  try { const r = await fetchApi('/api/admin/support/unread'); supportUnread.value = r.count || 0 } catch(e) {}
}

onMounted(() => {
  loadUnread()
  const intv = setInterval(loadUnread, 15000)
  onUnmounted(() => clearInterval(intv))
})
</script>
