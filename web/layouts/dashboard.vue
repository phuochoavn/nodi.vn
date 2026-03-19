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
          <NodiLogo :size="32" />
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
          <!-- Shop Switcher -->
          <div class="relative" ref="shopDropdownRef">
            <button @click="showShopDropdown = !showShopDropdown"
                    class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors">
              <Store :size="16" class="text-primary" />
              <span class="text-sm font-semibold text-slate-800 dark:text-slate-100 max-w-[200px] truncate">
                {{ currentStoreName || 'Dashboard' }}
              </span>
              <ChevronDown :size="14" class="text-slate-400" :class="{ 'rotate-180': showShopDropdown }" />
            </button>
            <!-- Dropdown -->
            <Transition enter-active-class="transition-all duration-200 ease-out" enter-from-class="opacity-0 -translate-y-1" enter-to-class="opacity-100 translate-y-0"
                        leave-active-class="transition-all duration-150 ease-in" leave-from-class="opacity-100 translate-y-0" leave-to-class="opacity-0 -translate-y-1">
              <div v-if="showShopDropdown"
                   class="absolute top-full left-0 mt-1 w-72 bg-white dark:bg-slate-800 rounded-xl shadow-lg border border-slate-200 dark:border-slate-700 py-1 z-50">
                <button v-for="s in stores" :key="s.store_id"
                        @click="handleSwitchStore(s.store_id)"
                        class="w-full flex items-center gap-3 px-4 py-3 hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors text-left"
                        :class="{ 'bg-primary/5': s.store_id === activeStoreId }">
                  <Store :size="16" :class="s.store_id === activeStoreId ? 'text-primary' : 'text-slate-400'" />
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-slate-800 dark:text-slate-100 truncate">{{ s.store_name }}</p>
                    <p class="text-xs text-slate-400">{{ s.store_id }} · {{ s.role }}</p>
                  </div>
                  <span v-if="s.store_id === activeStoreId" class="text-xs bg-primary/10 text-primary px-2 py-0.5 rounded-full">Đang chọn</span>
                </button>
                <!-- Create Store -->
                <div class="border-t border-slate-200 dark:border-slate-700 mt-1 pt-1">
                  <div v-if="showCreateForm" class="px-4 py-2">
                    <input v-model="newStoreName" type="text" placeholder="Tên cửa hàng mới..."
                           class="w-full px-3 py-2 text-sm border border-slate-200 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-100 focus:outline-none focus:border-primary"
                           @keyup.enter="handleCreateStore" />
                    <div class="flex gap-2 mt-2">
                      <button @click="handleCreateStore" :disabled="creatingStore"
                              class="flex-1 px-3 py-1.5 text-xs font-medium text-white bg-primary rounded-lg hover:bg-primary/90 disabled:opacity-50">
                        {{ creatingStore ? 'Đang tạo...' : 'Tạo' }}
                      </button>
                      <button @click="showCreateForm = false; newStoreName = ''"
                              class="px-3 py-1.5 text-xs text-slate-500 hover:text-slate-700 dark:hover:text-slate-300">
                        Hủy
                      </button>
                    </div>
                    <p v-if="createError" class="text-xs text-red-500 mt-1">{{ createError }}</p>
                  </div>
                  <button v-else @click="showCreateForm = true"
                          class="w-full flex items-center gap-3 px-4 py-3 hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors text-left text-primary">
                    <Plus :size="16" />
                    <span class="text-sm font-medium">Thêm cửa hàng</span>
                  </button>
                </div>
              </div>
            </Transition>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <!-- Notification Bell -->
          <div class="relative" ref="notifDropdownRef">
            <button @click="showNotifPanel = !showNotifPanel; if(!notifLoaded) loadNotifs()"
                    class="relative p-2 rounded-lg text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors">
              <Bell :size="18" />
              <span v-if="notifCount > 0" class="absolute -top-0.5 -right-0.5 w-4.5 h-4.5 bg-red-500 text-white text-[10px] font-bold rounded-full flex items-center justify-center">{{ notifCount > 9 ? '9+' : notifCount }}</span>
            </button>
            <Transition enter-active-class="transition-all duration-200 ease-out" enter-from-class="opacity-0 -translate-y-1" enter-to-class="opacity-100 translate-y-0"
                        leave-active-class="transition-all duration-150 ease-in" leave-from-class="opacity-100 translate-y-0" leave-to-class="opacity-0 -translate-y-1">
              <div v-if="showNotifPanel" class="absolute top-full right-0 mt-1 w-80 bg-white dark:bg-slate-800 rounded-xl shadow-lg border border-slate-200 dark:border-slate-700 z-50 overflow-hidden">
                <div class="px-4 py-3 border-b border-slate-100 dark:border-slate-700 flex justify-between items-center">
                  <span class="text-sm font-bold text-slate-800 dark:text-white">🔔 Thông báo</span>
                  <span class="text-xs bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-300 px-2 py-0.5 rounded-full">{{ notifCount }}</span>
                </div>
                <div class="max-h-80 overflow-y-auto">
                  <div v-if="notifItems.length === 0" class="px-4 py-8 text-center text-slate-400 text-sm">✅ Không có thông báo</div>
                  <div v-for="(n, i) in notifItems" :key="i" class="px-4 py-3 hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors border-b border-slate-50 dark:border-slate-700/30 last:border-0">
                    <p class="text-sm font-semibold text-slate-800 dark:text-slate-100">{{ n.title }}</p>
                    <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{{ n.message }}</p>
                  </div>
                </div>
              </div>
            </Transition>
          </div>
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
import { LayoutDashboard, ShoppingCart, Package, Wallet, BarChart3, HardDrive, Settings, LogOut, PanelLeftClose, PanelLeftOpen, Menu as MenuIcon, Store, Sun, Moon, ChevronDown, Plus, Calculator, Truck, Users, Bell } from 'lucide-vue-next'

const { user, stores, activeStoreId, activeStoreName, logout, switchStore, loadStores, createStore } = useAuth()
const colorMode = useColorMode()
const collapsed = ref(false)
const sidebarOpen = ref(false)
const showShopDropdown = ref(false)
const shopDropdownRef = ref(null)
const showCreateForm = ref(false)
const newStoreName = ref('')
const creatingStore = ref(false)
const createError = ref('')

// Notifications
const { fetchApi } = useAuth()
const showNotifPanel = ref(false)
const notifItems = ref([])
const notifCount = ref(0)
const notifLoaded = ref(false)
const notifDropdownRef = ref(null)

async function loadNotifs() {
  try {
    const r = await fetchApi('/api/dashboard/notifications')
    notifItems.value = r.notifications || []
    notifCount.value = r.count || 0
    notifLoaded.value = true
  } catch (e) { console.error(e) }
}

// Load stores and notifications on mount
onMounted(() => {
  loadStores()
  loadNotifs()
})

const currentStoreName = computed(() => {
  if (activeStoreName.value) return activeStoreName.value
  const active = stores.value?.find(s => s.store_id === activeStoreId.value)
  return active?.store_name || user.value?.display_name || 'Dashboard'
})

const handleSwitchStore = async (storeId) => {
  if (storeId === activeStoreId.value) {
    showShopDropdown.value = false
    return
  }
  showShopDropdown.value = false
  await switchStore(storeId)
}

const handleCreateStore = async () => {
  if (!newStoreName.value.trim()) return
  creatingStore.value = true
  createError.value = ''
  try {
    const res = await createStore(newStoreName.value.trim())
    if (res.success) {
      showCreateForm.value = false
      newStoreName.value = ''
    } else {
      createError.value = res.message || 'Lỗi tạo cửa hàng'
    }
  } catch (e) {
    createError.value = 'Lỗi kết nối'
  } finally {
    creatingStore.value = false
  }
}

// Close dropdown on click outside
const onClickOutside = (e) => {
  if (shopDropdownRef.value && !shopDropdownRef.value.contains(e.target)) {
    showShopDropdown.value = false
  }
}
onMounted(() => document.addEventListener('click', onClickOutside))
onUnmounted(() => document.removeEventListener('click', onClickOutside))

const toggleDark = () => {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}

const navItems = [
  { icon: LayoutDashboard, label: 'Tổng quan', to: '/dashboard' },
  { icon: ShoppingCart, label: 'Đơn hàng', to: '/dashboard/don-hang' },
  { icon: Package, label: 'Tồn kho', to: '/dashboard/ton-kho' },
  { icon: Truck, label: 'Nhập hàng', to: '/dashboard/nhap-hang' },
  { icon: Users, label: 'Nhân viên', to: '/dashboard/nhan-vien' },
  { icon: Wallet, label: 'Công nợ', to: '/dashboard/cong-no' },
  { icon: BarChart3, label: 'Báo cáo', to: '/dashboard/bao-cao' },
  { icon: Calculator, label: 'Kế toán & Thuế', to: '/dashboard/ke-toan' },
  { icon: HardDrive, label: 'Backup', to: '/dashboard/backup' },
  { icon: Settings, label: 'Cài đặt', to: '/dashboard/cai-dat' },
]
</script>
