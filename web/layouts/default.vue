<template>
  <div id="app" class="flex flex-col min-h-screen bg-[var(--bg)] text-[var(--text)] transition-colors duration-300">
    <AppHeader />
    <main class="flex-1">
      <slot />
    </main>
    <AppFooter />
  </div>
</template>

<script setup>
// Scroll reveal logic moved to a Nuxt plugin to properly handle page transitions
</script>
