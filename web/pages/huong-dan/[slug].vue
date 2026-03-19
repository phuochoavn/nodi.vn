<template>
  <div v-if="guide" class="min-h-screen">
    <!-- Hero -->
    <section class="relative overflow-hidden bg-gradient-to-br from-emerald-800 via-green-700 to-teal-600 pt-24 pb-10 md:pt-28 md:pb-14">
      <div class="absolute inset-0 overflow-hidden pointer-events-none">
        <div class="absolute -top-24 -right-24 w-96 h-96 bg-white/5 rounded-full blur-3xl"></div>
      </div>
      <div class="container max-w-5xl relative z-10">
        <nav class="mb-4">
          <NuxtLink to="/huong-dan" class="inline-flex items-center gap-1.5 text-emerald-200 hover:text-white text-sm font-medium transition-colors">
            <ArrowLeft :size="16" /> Tất cả hướng dẫn
          </NuxtLink>
        </nav>
        <div class="flex items-center gap-3">
          <span class="text-3xl">{{ guide.icon }}</span>
          <h1 class="text-2xl md:text-3xl font-extrabold text-white">{{ guide.title }}</h1>
        </div>
      </div>
    </section>

    <!-- Content -->
    <section class="py-8 md:py-12 bg-slate-50 dark:bg-slate-950">
      <div class="container max-w-5xl">
        <div class="grid grid-cols-1 lg:grid-cols-[220px_1fr] gap-8">
          <!-- Sidebar TOC -->
          <aside class="hidden lg:block">
            <div class="sticky top-20">
              <h4 class="text-xs font-bold text-slate-400 uppercase tracking-wider mb-3">📋 Mục lục</h4>
              <ul class="space-y-1.5">
                <li v-for="(h, i) in guide.headings" :key="i">
                  <a :href="'#s' + i"
                     class="block text-sm text-slate-500 dark:text-slate-400 hover:text-primary-600 dark:hover:text-primary-400 transition-colors py-0.5 border-l-2 border-transparent hover:border-primary-500 pl-3"
                     :class="{ '!text-primary-600 dark:!text-primary-400 !border-primary-500 font-medium': activeSection === i }">
                    {{ h }}
                  </a>
                </li>
              </ul>
              <!-- Prev / Next -->
              <div class="mt-6 pt-4 border-t border-slate-200 dark:border-slate-800 space-y-2">
                <NuxtLink v-if="guide.prev" :to="'/huong-dan/' + guide.prev" class="flex items-center gap-1.5 text-sm text-primary-600 dark:text-primary-400 hover:underline font-medium">
                  <ArrowLeft :size="14" /> Bài trước
                </NuxtLink>
                <NuxtLink v-if="guide.next" :to="'/huong-dan/' + guide.next" class="flex items-center gap-1.5 text-sm text-primary-600 dark:text-primary-400 hover:underline font-medium">
                  Bài tiếp <ArrowRight :size="14" />
                </NuxtLink>
              </div>
            </div>
          </aside>

          <!-- Article -->
          <article class="guide-article bg-white dark:bg-slate-900 rounded-2xl p-6 md:p-10 shadow-card border border-slate-100 dark:border-slate-800" v-html="guide.content"></article>
        </div>

        <!-- Mobile prev/next -->
        <div class="flex justify-between mt-6 lg:hidden">
          <NuxtLink v-if="guide.prev" :to="'/huong-dan/' + guide.prev" class="inline-flex items-center gap-1.5 px-4 py-2.5 bg-white dark:bg-slate-900 rounded-xl text-sm font-medium text-primary-600 shadow-sm border border-slate-200 dark:border-slate-700">
            <ArrowLeft :size="14" /> Bài trước
          </NuxtLink>
          <span v-else></span>
          <NuxtLink v-if="guide.next" :to="'/huong-dan/' + guide.next" class="inline-flex items-center gap-1.5 px-4 py-2.5 bg-white dark:bg-slate-900 rounded-xl text-sm font-medium text-primary-600 shadow-sm border border-slate-200 dark:border-slate-700">
            Bài tiếp <ArrowRight :size="14" />
          </NuxtLink>
        </div>
      </div>
    </section>
  </div>

  <!-- Not found -->
  <div v-else class="min-h-screen flex flex-col items-center justify-center bg-slate-50 dark:bg-slate-950 pt-20">
    <div class="text-6xl mb-4">📖</div>
    <h2 class="text-xl font-bold text-slate-700 dark:text-slate-300 mb-2">Hướng dẫn không tồn tại</h2>
    <NuxtLink to="/huong-dan" class="text-primary-600 hover:underline font-medium flex items-center gap-1.5 mt-2">
      <ArrowLeft :size="16" /> Quay lại trang hướng dẫn
    </NuxtLink>
  </div>
</template>

<script setup>
import { ArrowLeft, ArrowRight } from 'lucide-vue-next'
import { useGuideData } from '~/composables/useGuideData'
import { useGuideDataPart2 } from '~/composables/useGuideData2'

definePageMeta({ layout: 'default' })

const route = useRoute()
const slug = route.params.slug

// Merge both guide sets
const { guides: guides1 } = useGuideData()
const { guides: guides2 } = useGuideDataPart2()
const allGuides = { ...guides1, ...guides2 }

const guide = allGuides[slug] || null

if (guide) {
  useHead({
    title: guide.title + ' — Hướng dẫn Nodi POS',
    meta: [
      { name: 'description', content: guide.title + ' — Hướng dẫn chi tiết sử dụng phần mềm quản lý bán hàng Nodi POS.' },
      { property: 'og:title', content: guide.title + ' — Hướng dẫn Nodi POS' },
    ]
  })
}

// Active section tracking (intersection observer)
const activeSection = ref(0)

onMounted(() => {
  if (!guide) return
  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          const id = entry.target.id
          if (id && id.startsWith('s')) {
            activeSection.value = parseInt(id.slice(1))
          }
        }
      })
    },
    { rootMargin: '-80px 0px -60% 0px', threshold: 0.1 }
  )
  setTimeout(() => {
    document.querySelectorAll('.guide-article h2[id]').forEach(el => observer.observe(el))
  }, 100)
})
</script>

<style scoped>
/* Article typography */
.guide-article :deep(h2) {
  @apply text-xl font-extrabold text-slate-800 dark:text-slate-100 mt-8 mb-4 pb-2 border-b border-slate-100 dark:border-slate-800;
}
.guide-article :deep(h2:first-child) {
  @apply mt-0;
}
.guide-article :deep(p) {
  @apply leading-relaxed text-slate-600 dark:text-slate-300 mb-4;
}
.guide-article :deep(ul), .guide-article :deep(ol) {
  @apply mb-4 pl-6 space-y-1.5;
}
.guide-article :deep(ul) { @apply list-disc; }
.guide-article :deep(ol) { @apply list-decimal; }
.guide-article :deep(li) {
  @apply text-slate-600 dark:text-slate-300 leading-relaxed;
}
.guide-article :deep(strong) {
  @apply text-slate-800 dark:text-slate-100;
}
.guide-article :deep(code) {
  @apply bg-slate-100 dark:bg-slate-800 text-primary-700 dark:text-primary-400 px-1.5 py-0.5 rounded text-sm;
}
.guide-article :deep(a) {
  @apply text-primary-600 dark:text-primary-400 hover:underline font-medium;
}
.guide-article :deep(em) {
  @apply text-slate-500 dark:text-slate-400;
}
.guide-article :deep(kbd) {
  @apply inline-flex items-center px-2 py-0.5 bg-slate-100 dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded text-xs font-mono font-bold text-slate-700 dark:text-slate-300 shadow-sm;
}

/* Tables */
.guide-article :deep(table) {
  @apply w-full mb-6 text-sm rounded-xl overflow-hidden border border-slate-200 dark:border-slate-700;
}
.guide-article :deep(thead) {
  @apply bg-slate-50 dark:bg-slate-800;
}
.guide-article :deep(th) {
  @apply text-left px-4 py-2.5 font-semibold text-slate-700 dark:text-slate-200 border-b border-slate-200 dark:border-slate-700;
}
.guide-article :deep(td) {
  @apply px-4 py-2.5 text-slate-600 dark:text-slate-300 border-b border-slate-100 dark:border-slate-800;
}
.guide-article :deep(tbody tr:hover) {
  @apply bg-slate-50/50 dark:bg-slate-800/50;
}

/* Tip box */
.guide-article :deep(.tip-box) {
  @apply bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800/40 rounded-xl px-4 py-3 text-sm text-emerald-800 dark:text-emerald-200 mb-4;
}

/* FAQ items */
.guide-article :deep(.faq-item) {
  @apply bg-slate-50 dark:bg-slate-800/50 rounded-xl p-4 mb-3 border border-slate-100 dark:border-slate-700;
}
.guide-article :deep(.faq-item strong) {
  @apply text-slate-800 dark:text-slate-100 text-base;
}
.guide-article :deep(.faq-item p) {
  @apply mt-2 mb-0 text-sm;
}

/* Color helpers */
.guide-article :deep(.text-red) {
  @apply text-red-600 dark:text-red-400 font-semibold;
}
</style>
