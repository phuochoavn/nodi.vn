<template>
  <div class="nodi-logo" :style="{ gap: gap + 'px' }">
    <img src="/logo.png" :width="size" :height="size" alt="Nodi POS Logo" class="nodi-logo-img" />
    <span v-if="showText" class="nodi-logo-text" :style="{ fontSize: textSize + 'px' }">NODI</span>
  </div>
</template>

<script setup>
const props = defineProps({
  size: { type: Number, default: 28 },
  showText: { type: Boolean, default: false },
  textSize: { type: Number, default: 24 },
  gap: { type: Number, default: 8 },
})
</script>

<style scoped>
.nodi-logo {
  display: inline-flex;
  align-items: center;
}
.nodi-logo-img {
  object-fit: contain;
}
.nodi-logo-text {
  font-family: 'Inter', sans-serif;
  font-weight: 800;
  color: #0fa470;
  line-height: 1;
}
</style>
