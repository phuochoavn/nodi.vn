<template>
  <section ref="heroRef" class="hero-glow relative min-h-screen flex items-center justify-center overflow-hidden pt-20"
           @mousemove="handleMouseMove">
    <!-- Animated Gradient Background -->
    <div class="absolute inset-0 bg-gradient-to-br from-slate-900 via-[#0a1128] to-slate-900"></div>

    <!-- Grid Dots Pattern -->
    <div class="absolute inset-0 bg-grid-dots opacity-30 mix-blend-overlay"></div>

    <!-- Animated Blobs Behind the content -->
    <div class="absolute top-1/4 left-1/4 w-[500px] h-[500px] bg-primary/20 rounded-full blur-[100px] animate-blob mix-blend-screen pointer-events-none"></div>
    <div class="absolute bottom-1/4 right-1/4 w-[600px] h-[600px] bg-secondary/10 rounded-full blur-[120px] animate-blob mix-blend-screen pointer-events-none" style="animation-delay: 2s;"></div>
    <div class="absolute top-1/2 left-1/2 w-[400px] h-[400px] bg-emerald-500/15 rounded-full blur-[90px] animate-blob mix-blend-screen pointer-events-none" style="animation-delay: 4s;"></div>

    <div class="container relative z-10 py-16 md:py-24">
      <div class="flex flex-col lg:flex-row items-center gap-12 lg:gap-16">
        
        <!-- Left Content (Text) -->
        <div class="flex-1 text-center lg:text-left max-w-2xl mx-auto lg:mx-0">
          <div class="reveal inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white/5 border border-white/10 text-sm text-green-400 mb-8 backdrop-blur-md shadow-[0_0_15px_rgba(34,197,94,0.1)]">
            <Sparkles :size="14" />
            <span class="font-medium text-white/90">Phiên bản mới v1.1.0 — Tốc độ x3</span>
          </div>

          <h1 class="reveal reveal-delay-1 text-4xl sm:text-5xl lg:text-6xl font-black text-white leading-[1.15] tracking-tight mb-6">
            Quản lý vật tư nông nghiệp
            <span class="block mt-2 text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-emerald-300">
              đơn giản, nhanh chóng
            </span>
          </h1>

          <p class="reveal reveal-delay-2 text-lg sm:text-xl text-slate-300 mb-10 leading-relaxed font-light">
            Bán hàng, công nợ, hóa đơn điện tử, tồn kho — tất cả trong 1 phần mềm.
            Thiết kế <strong class="text-white font-medium">chuẩn UI/UX</strong> dành riêng cho đại lý vật tư Việt Nam.
          </p>

          <div class="reveal reveal-delay-3 flex flex-col sm:flex-row gap-4 justify-center lg:justify-start mb-14">
            <NuxtLink to="/tai-ung-dung"
                      class="btn btn-lg bg-gradient-to-r from-green-500 to-emerald-600 text-white shadow-xl shadow-green-500/25 hover:shadow-2xl hover:shadow-green-500/40 hover:-translate-y-1 transition-all">
              <Download :size="20" />
              Tải dùng thử miễn phí
            </NuxtLink>
            <NuxtLink to="/tinh-nang" class="btn btn-lg bg-white/5 text-white hover:bg-white/10 border border-white/10 backdrop-blur-sm transition-all hover:border-white/20">
              Xem tính năng
              <ArrowRight :size="18" />
            </NuxtLink>
          </div>

          <!-- Stats Row -->
          <div class="reveal reveal-delay-4 flex items-center justify-center lg:justify-start gap-6 sm:gap-10">
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-full bg-green-500/20 flex items-center justify-center text-green-400"><Zap :size="18" /></div>
              <div>
                <div class="text-2xl font-black text-white">5s</div>
                <div class="text-xs text-slate-400 uppercase tracking-wider font-semibold">Tạo hóa đơn</div>
              </div>
            </div>
            <div class="w-px h-10 bg-slate-700/50"></div>
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-full bg-blue-500/20 flex items-center justify-center text-blue-400"><RefreshCw :size="18" /></div>
              <div>
                <div class="text-2xl font-black text-white">14</div>
                <div class="text-xs text-slate-400 uppercase tracking-wider font-semibold">Entity Sync</div>
              </div>
            </div>
            <div class="w-px h-10 bg-slate-700/50 hidden sm:block"></div>
            <div class="flex items-center gap-3 hidden sm:flex">
              <div class="w-10 h-10 rounded-full bg-amber-500/20 flex items-center justify-center text-amber-400"><ShieldCheck :size="18" /></div>
              <div>
                <div class="text-2xl font-black text-white">100%</div>
                <div class="text-xs text-slate-400 uppercase tracking-wider font-semibold">Offline-first</div>
              </div>
            </div>
          </div>
        </div>

        <!-- Right Content (Mockup) -->
        <div class="flex-1 w-full max-w-lg lg:max-w-none relative perspective-1000 reveal reveal-delay-2 hidden md:block">
          <!-- Glow behind mockup -->
          <div class="absolute -inset-4 bg-gradient-to-tr from-green-500/20 to-blue-500/20 rounded-[2rem] blur-2xl -z-10 animate-pulse-soft"></div>
          
          <!-- CSS-only App Mockup -->
          <div class="relative w-full rounded-2xl bg-[#0f172a] p-2 sm:p-3 shadow-2xl shadow-green-900/50 border border-slate-700/50 transform rotate-y-[-5deg] rotate-x-[5deg] hover:rotate-y-0 hover:rotate-x-0 transition-transform duration-700 ease-out">
            <!-- Screen container -->
            <div class="bg-[#1e293b] rounded-xl p-3 sm:p-5 h-[350px] sm:h-[420px] shadow-inner flex flex-col relative overflow-hidden">
              <div class="absolute inset-0 noise-overlay"></div>
              
              <!-- Top bar -->
              <div class="flex items-center gap-4 mb-5 border-b border-slate-700/50 pb-4 relative z-10">
                <div class="flex gap-1.5">
                  <div class="w-3 h-3 rounded-full bg-red-500/80 shadow-sm shadow-red-500/50"></div>
                  <div class="w-3 h-3 rounded-full bg-yellow-500/80 shadow-sm shadow-yellow-500/50"></div>
                  <div class="w-3 h-3 rounded-full bg-green-500/80 shadow-sm shadow-green-500/50"></div>
                </div>
                <!-- Search bar fake -->
                <div class="flex-1 bg-slate-800/80 rounded-lg h-7 border border-slate-700 flex items-center px-3">
                  <div class="w-3 h-3 rounded-full border-2 border-slate-500 mr-2"></div>
                  <div class="w-24 h-2 bg-slate-600 rounded-full"></div>
                </div>
              </div>

              <!-- Sidebar + Content -->
              <div class="flex gap-4 flex-1 relative z-10">
                <!-- Sidebar -->
                <div class="w-1/4 max-w-[120px] space-y-3 border-r border-slate-700/50 pr-3">
                  <div class="h-6 bg-green-500/20 border border-green-500/30 rounded flex items-center px-2 gap-2">
                    <div class="w-3 h-3 bg-green-400 rounded-sm"></div>
                    <div class="w-10 h-1.5 bg-green-400/80 rounded-full"></div>
                  </div>
                  <div class="h-6 hover:bg-slate-800 rounded flex items-center px-2 gap-2">
                    <div class="w-3 h-3 bg-slate-500 rounded-sm"></div>
                    <div class="w-12 h-1.5 bg-slate-500 rounded-full"></div>
                  </div>
                  <div class="h-6 hover:bg-slate-800 rounded flex items-center px-2 gap-2">
                    <div class="w-3 h-3 bg-slate-500 rounded-sm"></div>
                    <div class="w-16 h-1.5 bg-slate-500 rounded-full"></div>
                  </div>
                  <div class="h-6 hover:bg-slate-800 rounded flex items-center px-2 gap-2">
                    <div class="w-3 h-3 bg-slate-500 rounded-sm"></div>
                    <div class="w-8 h-1.5 bg-slate-500 rounded-full"></div>
                  </div>
                </div>

                <!-- Main area -->
                <div class="flex-1 flex flex-col gap-4">
                  <!-- Header stats -->
                  <div class="grid grid-cols-3 gap-3">
                    <div class="h-16 bg-slate-800 rounded-xl border border-slate-700 p-2 flex flex-col justify-center">
                      <div class="w-6 h-1.5 bg-slate-500 mb-2 rounded-full"></div>
                      <div class="w-16 h-3 bg-green-400 rounded-full"></div>
                    </div>
                    <div class="h-16 bg-slate-800 rounded-xl border border-slate-700 p-2 flex flex-col justify-center">
                      <div class="w-8 h-1.5 bg-slate-500 mb-2 rounded-full"></div>
                      <div class="w-12 h-3 bg-slate-200 rounded-full"></div>
                    </div>
                    <div class="h-16 bg-slate-800 rounded-xl border border-slate-700 p-2 flex flex-col justify-center">
                      <div class="w-5 h-1.5 bg-slate-500 mb-2 rounded-full"></div>
                      <div class="w-14 h-3 bg-slate-200 rounded-full"></div>
                    </div>
                  </div>

                  <!-- Chart placeholder -->
                  <div class="flex-1 bg-slate-800 rounded-xl border border-slate-700 p-3 relative overflow-hidden flex items-end gap-2">
                    <!-- Fake chart bars -->
                    <div v-for="i in 8" :key="i" class="flex-1 bg-gradient-to-t from-green-500/80 to-green-400 rounded-t-sm" :style="{ height: `${Math.random() * 60 + 20}%` }"></div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom curve -->
    <div class="absolute bottom-[-1px] left-0 right-0 pointer-events-none z-20">
      <svg viewBox="0 0 1440 80" fill="none" preserveAspectRatio="none" class="w-full h-12 md:h-16 lg:h-20">
        <path d="M0 80L1440 80L1440 30C1200 0 960 60 720 30C480 0 240 60 0 30L0 80Z"
              class="fill-white dark:fill-[var(--bg)] transition-colors duration-300"/>
      </svg>
    </div>
  </section>
</template>

<script setup>
import { Download, ArrowRight, Sparkles, Zap, RefreshCw, ShieldCheck } from 'lucide-vue-next'
import { ref, onMounted } from 'vue'

const heroRef = ref(null)

const handleMouseMove = (e) => {
  if (!heroRef.value) return
  const rect = heroRef.value.getBoundingClientRect()
  const x = e.clientX - rect.left
  const y = e.clientY - rect.top
  heroRef.value.style.setProperty('--mouse-x', `${x}px`)
  heroRef.value.style.setProperty('--mouse-y', `${y}px`)
}
</script>

<style scoped>
.perspective-1000 {
  perspective: 1000px;
}
</style>
