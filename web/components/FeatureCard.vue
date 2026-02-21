<template>
  <div class="card-glass group cursor-default text-center p-8 border hover:border-green-500/50 hover:shadow-xl hover:shadow-green-500/5 hover:-translate-y-1 transition-all duration-300">
    <div class="w-14 h-14 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-green-500/10 to-emerald-500/20 flex items-center justify-center text-primary group-hover:scale-110 transition-transform duration-300 relative overflow-hidden">
      <!-- subtle hover overlay in the icon container -->
      <div class="absolute inset-0 bg-white/20 dark:bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity"></div>
      <component :is="iconComponent" :size="28" class="relative z-10" />
    </div>
    <h3 class="text-xl font-bold mb-3 text-slate-900 dark:text-slate-100">{{ title }}</h3>
    <p class="text-[var(--text-muted)] leading-relaxed">{{ description }}</p>
  </div>
</template>

<script setup>
import {
  ShoppingCart, Package, Wallet, FileText, BarChart3, Bot,
  Zap, CloudUpload, RefreshCw, ClipboardList, FileSpreadsheet, Scan
} from 'lucide-vue-next'

const props = defineProps({
  icon: { type: String, required: true },
  title: { type: String, required: true },
  description: { type: String, default: '' },
})

const iconMap = {
  'shopping-cart': ShoppingCart,
  'package': Package,
  'wallet': Wallet,
  'file-text': FileText,
  'bar-chart': BarChart3,
  'bot': Bot,
  'zap': Zap,
  'cloud': CloudUpload,
  'refresh': RefreshCw,
  'clipboard': ClipboardList,
  'spreadsheet': FileSpreadsheet,
  'scan': Scan,
}

const iconComponent = computed(() => iconMap[props.icon] || Zap)
</script>
