<template>
  <footer class="bg-slate-900 text-slate-300 pt-16 pb-8 relative overflow-hidden">
    <!-- Subtle top gradient border -->
    <div class="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-primary/40 to-transparent"></div>

    <div class="container">
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-10 pb-12 border-b border-slate-700/50">
        <!-- Brand -->
        <div class="sm:col-span-2 lg:col-span-1">
          <NuxtLink to="/" class="flex items-center gap-2 mb-4">
            <NodiLogo :size="24" :show-text="true" :text-size="18" />
          </NuxtLink>
          <p class="text-slate-400 text-sm leading-relaxed max-w-[280px] mb-5">
            Phần mềm quản lý cửa hàng vật tư nông nghiệp thông minh, dễ sử dụng.
          </p>
        </div>

        <!-- Sản phẩm -->
        <div>
          <h4 class="text-white font-semibold text-sm mb-4 uppercase tracking-wider">Sản phẩm</h4>
          <nav class="flex flex-col gap-3">
            <NuxtLink to="/tinh-nang" class="text-sm text-slate-400 hover:text-primary transition-colors">Tính năng</NuxtLink>
            <NuxtLink to="/bang-gia" class="text-sm text-slate-400 hover:text-primary transition-colors">Bảng giá</NuxtLink>
            <NuxtLink to="/tai-ung-dung" class="text-sm text-slate-400 hover:text-primary transition-colors">Tải ứng dụng</NuxtLink>
            <NuxtLink to="/blog" class="text-sm text-slate-400 hover:text-primary transition-colors">Blog</NuxtLink>
            <NuxtLink to="/huong-dan" class="text-sm text-slate-400 hover:text-primary transition-colors">Hướng dẫn</NuxtLink>
          </nav>
        </div>

        <!-- Hỗ trợ -->
        <div>
          <h4 class="text-white font-semibold text-sm mb-4 uppercase tracking-wider">Hỗ trợ</h4>
          <nav class="flex flex-col gap-3">
            <NuxtLink to="/lien-he" class="text-sm text-slate-400 hover:text-primary transition-colors">Liên hệ</NuxtLink>
            <NuxtLink to="/chinh-sach-bao-mat" class="text-sm text-slate-400 hover:text-primary transition-colors">Chính sách bảo mật</NuxtLink>
            <NuxtLink to="/dieu-khoan-su-dung" class="text-sm text-slate-400 hover:text-primary transition-colors">Điều khoản sử dụng</NuxtLink>
          </nav>
        </div>

        <!-- Liên hệ -->
        <div>
          <h4 class="text-white font-semibold text-sm mb-4 uppercase tracking-wider">Liên hệ</h4>
          <div class="flex flex-col gap-3">
            <a href="tel:0374222326" class="text-sm text-slate-400 hover:text-primary transition-colors flex items-center gap-2">
              <Phone :size="14" /> 0374.222.326
            </a>
            <a href="mailto:hoavn12345@gmail.com" class="text-sm text-slate-400 hover:text-primary transition-colors flex items-center gap-2">
              <Mail :size="14" /> hoavn12345@gmail.com
            </a>
            <a href="https://zalo.me/0374222326" target="_blank" rel="noopener" class="text-sm text-slate-400 hover:text-primary transition-colors flex items-center gap-2">
              <MessageCircle :size="14" /> Chat Zalo
            </a>
            <span class="text-sm text-slate-500 flex items-center gap-2">
              <Clock :size="14" /> 8:00 – 17:00 (T2 – T7)
            </span>
          </div>
        </div>
      </div>

      <!-- Bottom Bar -->
      <div class="pt-8 flex flex-col md:flex-row items-center justify-between gap-4">
        <p class="text-slate-500 text-sm">
          © 2026 Nodi POS. Phát triển bởi <strong class="text-slate-400">Hoa Digital</strong>.
        </p>
        <button @click="scrollToTop"
                class="flex items-center gap-2 text-sm text-slate-500 hover:text-primary transition-colors group">
          <ArrowUp :size="14" class="group-hover:-translate-y-0.5 transition-transform" />
          Về đầu trang
        </button>
      </div>
    </div>
  </footer>
</template>

<script setup>
import { Phone, Mail, MessageCircle, Clock, ArrowUp } from 'lucide-vue-next'

const scrollToTop = () => {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}
</script>
