<template>
  <div class="relative flex flex-col h-full rounded-2xl p-6 md:p-8 transition-all duration-300 overflow-visible"
       :class="popular
         ? 'bg-gradient-to-b from-white to-green-50/50 dark:from-slate-800 dark:to-green-900/20 ring-2 ring-green-500 shadow-2xl shadow-green-500/15 scale-[1.03] z-10 md:-translate-y-3 hover:-translate-y-5 hover:shadow-[0_25px_60px_-12px_rgba(22,163,74,0.25)]'
         : 'bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 shadow-lg hover:shadow-xl hover:-translate-y-2 hover:border-green-500/30'">
    
    <!-- Popular Badge -->
    <div v-if="popular" class="absolute -top-4 left-0 right-0 flex justify-center">
      <span class="bg-gradient-to-r from-green-500 to-emerald-600 text-white text-xs font-bold px-5 py-2 rounded-full shadow-lg shadow-green-500/30 flex items-center gap-1.5 animate-pulse-soft">
        <Star :size="14" class="fill-white" /> Phổ biến nhất
      </span>
    </div>

    <!-- Plan Name -->
    <div class="mb-6">
      <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-1">{{ name }}</h3>
      <p class="text-[var(--text-muted)] text-sm leading-relaxed">{{ description }}</p>
    </div>

    <!-- Price -->
    <div class="mb-8 pb-6 border-b-2 border-slate-100 dark:border-slate-700/60">
      <div class="flex items-baseline gap-1">
        <span class="text-4xl font-black tracking-tight" :class="popular ? 'text-green-600 dark:text-green-400' : 'text-slate-900 dark:text-white'">{{ price }}</span>
        <span v-if="period" class="text-[var(--text-muted)] font-medium text-base">{{ period }}</span>
      </div>
    </div>

    <!-- Features List -->
    <ul class="space-y-3.5 mb-8 flex-1">
      <li v-for="(feature, i) in features" :key="i"
          class="flex items-start gap-3"
          :class="{ 'opacity-40': feature.disabled }">
        <div class="mt-0.5 flex-shrink-0">
          <div v-if="feature.disabled" class="w-5 h-5 rounded-full bg-slate-100 dark:bg-slate-700 flex items-center justify-center">
            <X :size="12" class="text-slate-400 dark:text-slate-500" />
          </div>
          <div v-else class="w-5 h-5 rounded-full flex items-center justify-center" :class="popular ? 'bg-green-500 text-white' : 'bg-green-100 dark:bg-green-900/40 text-green-600 dark:text-green-400'">
            <Check :size="12" />
          </div>
        </div>
        <span class="text-slate-700 dark:text-slate-300 text-sm leading-snug">{{ feature.text }}</span>
      </li>
    </ul>

    <!-- CTA Button -->
    <NuxtLink :to="cta.link"
              class="btn w-full justify-center transition-all duration-300 text-base font-bold"
              :class="popular
                ? 'bg-gradient-to-r from-green-500 to-emerald-600 text-white hover:shadow-lg hover:shadow-green-500/30 hover:-translate-y-0.5 py-4'
                : 'bg-slate-900 dark:bg-slate-700 text-white hover:bg-slate-800 dark:hover:bg-slate-600 hover:shadow-lg py-4'">
      {{ cta.text }}
    </NuxtLink>
  </div>
</template>

<script setup>
import { Check, X, Star } from 'lucide-vue-next'

defineProps({
  name: String,
  price: String,
  period: String,
  description: String,
  features: Array,
  cta: Object,
  popular: { type: Boolean, default: false },
})
</script>
