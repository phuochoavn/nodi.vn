<template>
  <div class="flex min-h-screen bg-slate-50 dark:bg-slate-900 transition-colors duration-300">
    <!-- Sidebar -->
    <aside class="fixed top-0 left-0 bottom-0 z-40 flex flex-col transition-all duration-300 border-r border-slate-200 dark:border-slate-700/50"
           :class="[
             collapsed ? 'w-[72px]' : 'w-60',
             sidebarOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
           ]"
           style="background: linear-gradient(180deg, #0f172a 0%, #1a2744 100%);">
      <!-- Logo -->
      <div class="flex items-center gap-2 px-4 py-5 border-b border-slate-700/50">
        <NuxtLink to="/dashboard" class="flex items-center gap-2 text-white font-extrabold text-lg overflow-hidden">
          <img src="/favicon.svg" class="w-8 h-8 flex-shrink-0 brightness-0 invert" alt="" />
          <Transition enter-active-class="transition-opacity duration-200" enter-from-class="opacity-0" enter-to-class="opacity-100"
                      leave-active-class="transition-opacity duration-200" leave-from-class="opacity-100" leave-to-class="opacity-0">
            <span v-if="!collapsed" class="whitespace-nowrap">
              Nodi<span class="text-primary-400">POS</span>
            </span>
          </Transition>
        </NuxtLink>
      </div>

      <!-- Nav Items -->
      <nav class="flex-1 overflow-y-auto py-3 px-2 space-y-1">
        <NuxtLink v-for="item in navItems" :key="item.to" :to="item.to"
                  class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-slate-400 text-sm font-medium transition-all hover:bg-slate-700/50 hover:text-white"
                  active-class="!bg-primary/20 !text-primary-300"
                  @click="sidebarOpen = false">
          <component :is="item.icon" :size="20" class="flex-shrink-0" />
          <Transition enter-active-class="transition-opacity duration-200" enter-from-class="opacity-0" enter-to-class="opacity-100"
                      leave-active-class="transition-opacity duration-200" leave-from-class="opacity-100" leave-to-class="opacity-0">
            <span v-if="!collapsed" class="whitespace-nowrap">{{ item.label }}</span>
          </Transition>
        </NuxtLink>
      </nav>

      <!-- Sidebar Footer -->
      <div class="px-2 py-3 border-t border-slate-700/50 space-y-1">
        <!-- Collapse toggle (desktop only) -->
        <button @click="collapsed = !collapsed"
                class="hidden md:flex w-full items-center gap-3 px-3 py-2.5 rounded-xl text-slate-500 text-sm hover:bg-slate-700/50 hover:text-white transition-colors">
          <PanelLeftClose v-if="!collapsed" :size="20" class="flex-shrink-0" />
          <PanelLeftOpen v-else :size="20" class="flex-shrink-0" />
          <span v-if="!collapsed" class="whitespace-nowrap">Thu gọn</span>
        </button>
        <button @click="logout"
                class="flex w-full items-center gap-3 px-3 py-2.5 rounded-xl text-slate-500 text-sm hover:bg-red-900/30 hover:text-red-400 transition-colors">
          <LogOut :size="20" class="flex-shrink-0" />
          <span v-if="!collapsed" class="whitespace-nowrap">Đăng xuất</span>
        </button>
      </div>
    </aside>

    <!-- Overlay (mobile) -->
    <Transition enter-active-class="transition-opacity duration-300" enter-from-class="opacity-0" enter-to-class="opacity-100"
                leave-active-class="transition-opacity duration-200" leave-from-class="opacity-100" leave-to-class="opacity-0">
      <div v-if="sidebarOpen" class="fixed inset-0 z-30 bg-black/50 md:hidden" @click="sidebarOpen = false"></div>
    </Transition>

    <!-- Main Content -->
    <div class="flex-1 transition-all duration-300"
         :class="collapsed ? 'md:ml-[72px]' : 'md:ml-60'">
      <!-- Top Header -->
      <header class="sticky top-0 z-20 flex items-center justify-between px-4 md:px-8 py-3 bg-white/80 dark:bg-slate-900/80 backdrop-blur-xl border-b border-slate-200 dark:border-slate-700/50">
        <div class="flex items-center gap-3">
          <button class="md:hidden p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-600 dark:text-slate-300"
                  @click="sidebarOpen = !sidebarOpen">
            <MenuIcon :size="20" />
          </button>
          <span class="text-sm font-semibold text-slate-800 dark:text-slate-100 flex items-center gap-2">
            <Store :size="16" class="text-primary" />
            {{ user?.display_name || 'Dashboard' }}
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

      <!-- Page Content -->
      <main class="p-4 md:p-8">
        <slot />
      </main>
    </div>
  </div>
</template>

<script setup>
import { LayoutDashboard, ShoppingCart, Package, Wallet, BarChart3, HardDrive, Settings, LogOut, PanelLeftClose, PanelLeftOpen, Menu as MenuIcon, Store, Sun, Moon } from 'lucide-vue-next'

const { user, logout } = useAuth()
const colorMode = useColorMode()
const sidebarOpen = ref(false)
const collapsed = ref(false)

const toggleDark = () => {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}

const navItems = [
  { icon: LayoutDashboard, label: 'Tổng quan', to: '/dashboard' },
  { icon: ShoppingCart, label: 'Đơn hàng', to: '/dashboard/don-hang' },
  { icon: Package, label: 'Tồn kho', to: '/dashboard/ton-kho' },
  { icon: Wallet, label: 'Công nợ', to: '/dashboard/cong-no' },
  { icon: BarChart3, label: 'Báo cáo', to: '/dashboard/bao-cao' },
  { icon: HardDrive, label: 'Backup', to: '/dashboard/backup' },
  { icon: Settings, label: 'Cài đặt', to: '/dashboard/cai-dat' },
]
</script>
