<template>
  <header class="fixed top-0 left-0 right-0 z-50 transition-all duration-300"
          :class="[
            isScrolled || !isHeroPage
              ? 'bg-white/90 dark:bg-slate-900/90 backdrop-blur-xl border-b border-slate-200/50 dark:border-slate-700/50 shadow-sm py-2'
              : 'bg-transparent py-4'
          ]">
    <div class="container flex items-center justify-between">
      <!-- Logo -->
      <NuxtLink to="/" class="flex items-center gap-2 group">
        <img src="/logo.svg" alt="Nodi POS" class="h-8 transition-all"
             :class="isHeroPage && !isScrolled ? 'brightness-0 invert' : 'dark:brightness-0 dark:invert'" />
      </NuxtLink>

      <!-- Desktop Nav -->
      <nav class="hidden md:flex items-center gap-8">
        <NuxtLink v-for="link in navLinks" :key="link.to" :to="link.to"
                  class="text-sm font-medium transition-colors duration-200"
                  :class="isScrolled || !isHeroPage
                    ? 'text-slate-600 dark:text-slate-300 hover:text-primary dark:hover:text-primary'
                    : 'text-white/80 hover:text-white'"
                  active-class="!text-primary">
          {{ link.label }}
        </NuxtLink>
      </nav>

      <!-- Actions -->
      <div class="flex items-center gap-2">
        <!-- Dark mode toggle -->
        <button @click="toggleColorMode"
                class="p-2 rounded-lg transition-colors duration-200"
                :class="isScrolled || !isHeroPage
                  ? 'text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800'
                  : 'text-white/70 hover:bg-white/10'"
                aria-label="Toggle dark mode">
          <Sun v-if="$colorMode.value === 'dark'" :size="18" />
          <Moon v-else :size="18" />
        </button>

        <!-- Auth buttons (desktop) -->
        <NuxtLink to="/login"
                  class="hidden md:inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200"
                  :class="isScrolled || !isHeroPage
                    ? 'text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800'
                    : 'text-white/80 hover:bg-white/10'">
          <LogIn :size="16" />
          Đăng nhập
        </NuxtLink>

        <!-- Register CTA -->
        <NuxtLink to="/register"
                  class="hidden md:inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-semibold transition-all duration-300"
                  :class="isScrolled || !isHeroPage
                    ? 'bg-gradient-to-r from-green-500 to-emerald-600 text-white hover:shadow-lg hover:shadow-green-500/25 shadow-sm'
                    : 'bg-white text-primary hover:bg-white/90 shadow-md'">
          <UserPlus :size="16" />
          Đăng ký miễn phí
        </NuxtLink>

        <!-- Mobile menu -->
        <button class="md:hidden p-2 rounded-lg"
                :class="isScrolled || !isHeroPage
                  ? 'text-slate-700 dark:text-slate-200'
                  : 'text-white'"
                @click="menuOpen = !menuOpen"
                aria-label="Menu">
          <X v-if="menuOpen" :size="22" />
          <Menu v-else :size="22" />
        </button>
      </div>
    </div>

    <!-- Mobile Menu -->
    <Transition
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 -translate-y-4"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition-all duration-200 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-4">
      <div v-if="menuOpen"
           class="md:hidden absolute top-full left-0 right-0 bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-700 shadow-lg p-4">
        <nav class="flex flex-col gap-1">
          <NuxtLink v-for="link in navLinks" :key="link.to" :to="link.to"
                    class="px-4 py-3 rounded-xl text-slate-700 dark:text-slate-200 font-medium hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                    active-class="!text-primary !bg-primary/5"
                    @click="menuOpen = false">
            {{ link.label }}
          </NuxtLink>
          <div class="border-t border-slate-100 dark:border-slate-800 my-2"></div>
          <NuxtLink to="/login"
                    class="px-4 py-3 rounded-xl text-slate-700 dark:text-slate-200 font-medium hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors flex items-center gap-2"
                    @click="menuOpen = false">
            <LogIn :size="16" />
            Đăng nhập
          </NuxtLink>
          <NuxtLink to="/register"
                    class="mt-2 btn bg-gradient-to-r from-green-500 to-emerald-600 text-white justify-center flex items-center gap-2"
                    @click="menuOpen = false">
            <UserPlus :size="16" />
            Đăng ký miễn phí
          </NuxtLink>
        </nav>
      </div>
    </Transition>
  </header>
</template>

<script setup>
import { Sun, Moon, Menu, X, UserPlus, LogIn } from 'lucide-vue-next'

const route = useRoute()
const colorMode = useColorMode()
const isScrolled = ref(false)
const menuOpen = ref(false)

// Pages that have a dark hero background (header should be transparent initially)
const heroPages = ['/', '/index']
const isHeroPage = computed(() => heroPages.includes(route.path))

const navLinks = [
  { label: 'Giới thiệu', to: '/gioi-thieu' },
  { label: 'Tính năng', to: '/tinh-nang' },
  { label: 'Bảng giá', to: '/bang-gia' },
  { label: 'Tải app', to: '/tai-ung-dung' },
  { label: 'Liên hệ', to: '/lien-he' },
]

const toggleColorMode = () => {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}

onMounted(() => {
  const handleScroll = () => { isScrolled.value = window.scrollY > 50 }
  window.addEventListener('scroll', handleScroll, { passive: true })
  handleScroll()
})

// Close mobile menu on route change
watch(() => route.path, () => { menuOpen.value = false })
</script>
