<template>
  <div>
    <div class="page-header">
      <div class="container">
        <h1>Tải Nodi POS — Miễn phí</h1>
        <p>Cài đặt trong 2 phút, bắt đầu bán hàng ngay</p>
      </div>
    </div>

    <!-- Download Box -->
    <section class="section">
      <div class="container">
        <div class="max-w-xl mx-auto card text-center p-10 reveal">
          <span class="badge badge-green mb-4">Phiên bản mới nhất</span>
          <h2 class="text-2xl font-extrabold mb-4 text-slate-900 dark:text-white">Nodi POS v1.1.0</h2>
          <div class="space-y-2 mb-6 text-[var(--text-muted)]">
            <p><span class="font-semibold text-[var(--text)]">Nền tảng:</span> Windows (64-bit)</p>
            <p><span class="font-semibold text-[var(--text)]">Kích thước:</span> ~7 MB</p>
          </div>
          <a href="https://zalo.me/0374222326" target="_blank" rel="noopener"
             class="btn btn-primary btn-lg w-full justify-center">
            <MessageCircle :size="20" />
            Liên hệ qua Zalo để nhận link tải
          </a>
        </div>
      </div>
    </section>

    <!-- System Requirements -->
    <section class="section section-alt">
      <div class="container">
        <h2 class="section-title reveal">Yêu cầu hệ thống</h2>
        <div class="grid-3 mt-10">
          <div v-for="req in sysReqs" :key="req.label"
               class="card text-center p-8 reveal">
            <div class="w-14 h-14 mx-auto mb-4 rounded-2xl bg-secondary/10 dark:bg-secondary/20 flex items-center justify-center text-secondary">
              <component :is="req.iconComponent" :size="28" />
            </div>
            <h3 class="text-lg font-bold mb-1 text-slate-800 dark:text-slate-100">{{ req.label }}</h3>
            <p class="text-[var(--text-muted)]">{{ req.value }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Install Guide -->
    <section class="section">
      <div class="container">
        <h2 class="section-title reveal">Hướng dẫn cài đặt</h2>
        <div class="max-w-2xl mx-auto mt-10 relative">
          <!-- Vertical line connector -->
          <div class="absolute left-6 top-6 bottom-6 w-px bg-slate-200 dark:bg-slate-700 hidden md:block"></div>

          <div v-for="(step, i) in steps" :key="i"
               class="flex gap-5 mb-8 last:mb-0 reveal"
               :class="`reveal-delay-${Math.min(i+1,4)}`">
            <div class="relative z-10 w-12 h-12 rounded-full bg-primary text-white flex items-center justify-center font-extrabold text-lg flex-shrink-0 shadow-md">
              {{ i + 1 }}
            </div>
            <div class="pt-2">
              <h3 class="text-lg font-bold mb-1 text-slate-800 dark:text-slate-100">{{ step.title }}</h3>
              <p class="text-[var(--text-muted)]">{{ step.desc }}</p>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { Monitor, Cpu, HardDrive, MessageCircle } from 'lucide-vue-next'

useHead({
  title: 'Tải ứng dụng — Nodi POS',
  meta: [{ name: 'description', content: 'Tải Nodi POS miễn phí cho Windows. Cài đặt trong 2 phút, bắt đầu quản lý cửa hàng vật tư nông nghiệp ngay.' }],
})

const sysReqs = [
  { iconComponent: Monitor, label: 'Hệ điều hành', value: 'Windows 10/11 (64-bit)' },
  { iconComponent: Cpu, label: 'RAM', value: '4 GB trở lên' },
  { iconComponent: HardDrive, label: 'Ổ cứng', value: '500 MB trống' },
]

const steps = [
  { title: 'Tải file cài đặt', desc: 'Liên hệ Zalo 0374.222.326 để nhận link tải file .exe (~7 MB)' },
  { title: 'Chạy file cài đặt', desc: 'Mở file đã tải → nhấn Next → Install. Quá trình cài đặt chỉ mất 1 phút.' },
  { title: 'Nhập license key', desc: 'Mở ứng dụng → nhập license key (nếu có). Chưa có key? Dùng thử miễn phí 30 ngày!' },
  { title: 'Bắt đầu bán hàng!', desc: 'Thêm sản phẩm, thiết lập cửa hàng, và bắt đầu bán hàng ngay lập tức.' },
]
</script>
