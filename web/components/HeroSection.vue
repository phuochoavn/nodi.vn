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

        <!-- Right Content (Stacked Screenshot Mockup) -->
        <div class="flex-1 w-full max-w-lg lg:max-w-xl relative reveal reveal-delay-2 hidden md:block">
          <!-- Glow behind mockup -->
          <div class="absolute -inset-8 bg-gradient-to-tr from-green-500/25 to-blue-500/15 rounded-[3rem] blur-3xl -z-10 animate-pulse-soft"></div>
          
          <!-- Stacked Screenshots Container -->
          <div class="relative w-full h-[420px] lg:h-[480px]">
            
            <!-- Back layer: Tax screen -->
            <div class="hero-screenshot hero-screenshot--back absolute top-0 right-0 w-[75%] h-auto rounded-2xl overflow-hidden shadow-2xl shadow-black/40 border border-slate-600/30 transform rotate-3 translate-x-4 -translate-y-2 hover:rotate-1 hover:translate-x-2 transition-all duration-700 ease-out z-10">
              <img src="/screenshots/hero-tax.png" alt="Thuế & Kế toán - Nodi POS" class="w-full h-auto block" loading="lazy" />
            </div>

            <!-- Middle layer: Cashflow screen -->
            <div class="hero-screenshot hero-screenshot--mid absolute top-8 right-8 w-[78%] h-auto rounded-2xl overflow-hidden shadow-2xl shadow-black/40 border border-slate-600/40 transform -rotate-2 hover:-rotate-1 transition-all duration-700 ease-out z-20">
              <img src="/screenshots/hero-cashflow.png" alt="Sổ Quỹ Cashflow - Nodi POS" class="w-full h-auto block" loading="lazy" />
            </div>

            <!-- Front layer: POS screen (Primary) -->
            <div class="hero-screenshot hero-screenshot--front absolute top-14 left-0 w-[85%] h-auto rounded-2xl overflow-hidden shadow-2xl shadow-green-900/50 border border-green-500/20 transform -rotate-1 hover:rotate-0 hover:-translate-y-1 transition-all duration-700 ease-out z-30">
              <!-- Green glow ring on primary -->
              <div class="absolute -inset-px rounded-2xl bg-gradient-to-tr from-green-500/30 to-emerald-500/10 -z-10 blur-sm"></div>
              <img src="/screenshots/hero-pos.png" alt="Bán Hàng POS - Nodi POS" class="w-full h-auto block" loading="lazy" />
            </div>

            <!-- Floating badges -->
            <div class="absolute -bottom-2 -left-4 z-40 bg-slate-800/90 backdrop-blur-md border border-slate-600/50 rounded-xl px-4 py-2.5 shadow-xl animate-float">
              <div class="flex items-center gap-2">
                <div class="w-8 h-8 rounded-lg bg-green-500/20 flex items-center justify-center">
                  <ShoppingCart :size="16" class="text-green-400" />
                </div>
                <div>
                  <div class="text-xs text-slate-400">Bán hàng</div>
                  <div class="text-sm font-bold text-white">POS nhanh 5s</div>
                </div>
              </div>
            </div>

            <div class="absolute -top-4 right-0 z-40 bg-slate-800/90 backdrop-blur-md border border-slate-600/50 rounded-xl px-4 py-2.5 shadow-xl animate-float" style="animation-delay: 1.5s;">
              <div class="flex items-center gap-2">
                <div class="w-8 h-8 rounded-lg bg-blue-500/20 flex items-center justify-center">
                  <BarChart3 :size="16" class="text-blue-400" />
                </div>
                <div>
                  <div class="text-xs text-slate-400">Báo cáo</div>
                  <div class="text-sm font-bold text-white">Realtime</div>
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
import { Download, ArrowRight, Sparkles, Zap, RefreshCw, ShieldCheck, ShoppingCart, BarChart3 } from 'lucide-vue-next'
import { ref } from 'vue'

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

/* Float animation for badges */
@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-8px); }
}

.animate-float {
  animation: float 4s ease-in-out infinite;
}

/* Subtle hover parallax on screenshot stack */
.hero-screenshot {
  will-change: transform;
  backface-visibility: hidden;
}

/* Stagger entrance animation */
.hero-screenshot--back {
  animation: slideInBack 1s ease-out 0.6s both;
}
.hero-screenshot--mid {
  animation: slideInMid 1s ease-out 0.4s both;
}
.hero-screenshot--front {
  animation: slideInFront 1s ease-out 0.2s both;
}

@keyframes slideInBack {
  from { opacity: 0; transform: rotate(3deg) translateX(40px) translateY(-30px) scale(0.9); }
  to { opacity: 1; transform: rotate(3deg) translateX(16px) translateY(-8px) scale(1); }
}
@keyframes slideInMid {
  from { opacity: 0; transform: rotate(-2deg) translateX(30px) translateY(20px) scale(0.9); }
  to { opacity: 1; transform: rotate(-2deg) translateX(0) translateY(0) scale(1); }
}
@keyframes slideInFront {
  from { opacity: 0; transform: rotate(-1deg) translateY(40px) scale(0.95); }
  to { opacity: 1; transform: rotate(-1deg) translateY(0) scale(1); }
}
</style>
