<template>
  <div class="card flex items-center gap-4 p-5">
    <div class="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0"
         :class="colorClasses">
      <span v-if="isEmoji" class="text-2xl leading-none">{{ icon }}</span>
      <component v-else :is="iconComponent" :size="22" />
    </div>
    <div class="flex-1 min-w-0">
      <span class="block text-sm text-[var(--text-muted)] mb-0.5">{{ label }}</span>
      <span class="block text-xl font-extrabold text-slate-900 dark:text-white truncate">{{ formatted }}</span>
    </div>
  </div>
</template>

<script setup>
import { DollarSign, ShoppingCart, Package, Wallet, Users, Factory, Landmark, AlertTriangle } from 'lucide-vue-next'

const props = defineProps({
  icon: String,
  label: String,
  value: { type: [Number, String], default: 0 },
  format: { type: String, default: 'number' },
  color: { type: String, default: 'blue' },
})

const iconMap = {
  'dollar-sign': DollarSign,
  'shopping-cart': ShoppingCart,
  'package': Package,
  'wallet': Wallet,
  'users': Users,
  'factory': Factory,
  'landmark': Landmark,
  'alert-triangle': AlertTriangle,
}

// Check if the icon is an emoji (not found in iconMap)
const isEmoji = computed(() => !iconMap[props.icon])
const iconComponent = computed(() => iconMap[props.icon] || DollarSign)

const colorMap = {
  blue: 'bg-secondary/10 dark:bg-secondary/20 text-secondary',
  green: 'bg-primary/10 dark:bg-primary/20 text-primary',
  amber: 'bg-accent/10 dark:bg-accent/20 text-accent',
  red: 'bg-red-50 dark:bg-red-900/20 text-red-500',
}
const colorClasses = computed(() => colorMap[props.color] || colorMap.blue)

const formatted = computed(() => {
  if (props.format === 'currency') {
    return new Intl.NumberFormat('vi-VN').format(props.value) + 'đ'
  }
  return new Intl.NumberFormat('vi-VN').format(props.value)
})
</script>
