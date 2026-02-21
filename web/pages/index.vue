<template>
  <div>
    <!-- Hero -->
    <HeroSection />


    <!-- USP Cards -->
    <section class="py-16 md:py-24">
      <div class="container">
        <div class="text-center mb-12 reveal">
          <h2 class="section-title">Tại sao chọn <span class="text-gradient hover:opacity-80 transition-opacity cursor-default">Nodi POS?</span></h2>
          <p class="section-subtitle">Thiết kế riêng cho đại lý vật tư nông nghiệp — đơn giản, nhanh, hiệu quả</p>
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <div v-for="(usp, i) in usps" :key="usp.title"
               class="card-glass group cursor-default text-center p-8 hover:border-green-500/50 hover:shadow-xl hover:shadow-green-500/5 hover:-translate-y-1 transition-all duration-300 reveal"
               :class="`reveal-delay-${i + 1}`">
            <div class="w-16 h-16 mx-auto mb-6 rounded-2xl bg-gradient-to-br from-green-500/10 to-emerald-500/20 flex items-center justify-center text-primary group-hover:scale-110 transition-transform duration-300 relative overflow-hidden">
              <div class="absolute inset-0 bg-white/20 dark:bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity"></div>
              <component :is="usp.iconComponent" :size="30" class="relative z-10" />
            </div>
            <h3 class="text-xl font-bold mb-3 text-slate-900 dark:text-slate-100">{{ usp.title }}</h3>
            <p class="text-sm text-[var(--text-muted)] leading-relaxed">{{ usp.desc }}</p>
          </div>
        </div>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- Features Overview -->
    <section class="py-16 md:py-24 bg-slate-50 dark:bg-[#0f172a] relative overflow-hidden">
      <!-- subtle background element -->
      <div class="absolute top-0 right-0 w-[800px] h-[800px] bg-primary/5 rounded-full blur-[120px] -translate-y-1/2 translate-x-1/2 pointer-events-none"></div>

      <div class="container relative z-10">
        <div class="text-center mb-16 reveal">
          <h2 class="section-title">Tính năng nổi bật</h2>
          <p class="section-subtitle">Tất cả công cụ bạn cần để quản lý cửa hàng hiệu quả</p>
        </div>
        <div class="grid-3 mb-12">
          <FeatureCard
            v-for="(f, i) in features" :key="f.title"
            :icon="f.icon" :title="f.title" :description="f.desc"
            class="reveal"
            :class="`reveal-delay-${(i % 3) + 1}`"
          />
        </div>
        <div class="text-center reveal">
          <NuxtLink to="/tinh-nang" class="btn btn-outline border-slate-300 dark:border-slate-600 hover:border-primary">
            Xem chi tiết 9+ tính năng
            <ArrowRight :size="16" />
          </NuxtLink>
        </div>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- Pricing Preview -->
    <section class="py-16 md:py-28 relative">
      <!-- another subtle element -->
      <div class="absolute bottom-0 left-0 w-[600px] h-[600px] bg-secondary/5 rounded-full blur-[100px] translate-y-1/2 -translate-x-1/2 pointer-events-none"></div>
      
      <div class="container relative z-10">
        <div class="text-center mb-16 reveal">
          <h2 class="section-title">Bảng giá minh bạch</h2>
          <p class="section-subtitle">Không phí ẩn, không cam kết. Hủy bất cứ lúc nào.</p>
        </div>
        <div class="grid-3 items-start max-w-6xl mx-auto">
          <PricingCard
            v-for="(p, i) in pricingPreview" :key="p.name"
            v-bind="p"
            class="reveal"
            :class="`reveal-delay-${i + 1}`"
          />
        </div>
        <div class="text-center mt-12 reveal">
          <NuxtLink to="/bang-gia" class="text-primary hover:text-green-600 dark:hover:text-green-400 font-semibold inline-flex items-center gap-1 group transition-colors">
            Xem đầy đủ bảng giá
            <ArrowRight :size="16" class="group-hover:translate-x-1 transition-transform" />
          </NuxtLink>
        </div>
      </div>
    </section>

    <!-- CTA -->
    <section class="py-12 md:py-20 mb-10">
      <div class="container reveal">
        <div class="relative overflow-hidden rounded-[2.5rem] bg-gradient-to-br from-[#0f172a] via-[#1a2e4c] to-[#0f172a] border border-slate-700 p-10 md:p-14 text-center text-white shadow-2xl">
          <!-- Decorative elements -->
          <div class="absolute inset-0 noise-overlay opacity-10"></div>
          <div class="absolute top-0 right-0 w-80 h-80 bg-green-500/20 rounded-full -translate-y-1/2 translate-x-1/3 blur-3xl mix-blend-screen pointer-events-none"></div>
          <div class="absolute bottom-0 left-0 w-64 h-64 bg-blue-500/20 rounded-full translate-y-1/2 -translate-x-1/3 blur-3xl mix-blend-screen pointer-events-none"></div>

          <h2 class="text-3xl md:text-5xl font-black mb-4 relative z-10 tracking-tight">Sẵn sàng nâng cấp cửa hàng?</h2>
          <p class="text-lg md:text-xl text-slate-300/90 max-w-xl mx-auto mb-10 relative z-10 font-light">
            Cài đặt trong 1 phút, dùng thử miễn phí 30 ngày. Không yêu cầu thẻ tín dụng.
          </p>
          <NuxtLink to="/tai-ung-dung"
                    class="btn btn-lg bg-gradient-to-r from-green-500 to-emerald-500 text-white hover:shadow-2xl hover:shadow-green-500/40 hover:-translate-y-1 hover:scale-105 active:scale-95 transition-all relative z-10 w-full sm:w-auto justify-center">
            <Download :size="20" />
            Tải miễn phí cho Windows
          </NuxtLink>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { Zap, BarChart3, Bot, CloudUpload, ArrowRight, Download } from 'lucide-vue-next'

useHead({
  title: 'Nodi POS — Phần mềm quản lý cửa hàng vật tư nông nghiệp',
})

const usps = [
  { iconComponent: Zap, title: 'Bán hàng dưới 5 giây', desc: 'Quét mã, tìm kiếm nhanh, tính tiền tự động' },
  { iconComponent: BarChart3, title: 'Công nợ thông minh', desc: 'Theo dõi nợ khách hàng, nhà cung cấp. Thông báo tự động' },
  { iconComponent: Bot, title: 'Trợ lý AI', desc: 'Hỏi doanh thu, tồn kho, công nợ — AI trả lời ngay' },
  { iconComponent: CloudUpload, title: 'Backup đám mây', desc: 'Máy hư không mất dữ liệu. Khôi phục 1 click' },
]

const features = [
  { icon: 'shopping-cart', title: 'Bán hàng POS', desc: 'Quét mã, tính tiền, in hóa đơn — nhanh và chính xác' },
  { icon: 'package', title: 'Nhập hàng & NCC', desc: 'Quản lý nhà cung cấp, phiếu nhập, lô hàng' },
  { icon: 'wallet', title: 'Công nợ KH & NCC', desc: 'Theo dõi công nợ, phiếu thu chi, lịch sử giao dịch' },
  { icon: 'file-text', title: 'Hóa đơn điện tử', desc: 'Tích hợp VNPT S-Invoice, tuân thủ NĐ 123/2020' },
  { icon: 'bar-chart', title: 'Báo cáo & Thống kê', desc: 'Doanh thu, lãi/lỗ, tồn kho, top sản phẩm' },
  { icon: 'bot', title: 'Trợ lý AI Chatbot', desc: 'Hỏi bằng tiếng Việt tự nhiên, trả lời tức thì' },
]

const pricingPreview = [
  {
    name: 'Dùng thử', price: 'Miễn phí', period: '', description: '30 ngày đầy đủ tính năng',
    features: [
      { text: 'Bán hàng POS (20 đơn/ngày sau 30 ngày)' },
      { text: 'Quản lý SP, KH, NCC' },
      { text: 'Báo cáo cơ bản' },
      { text: 'AI Chatbot', disabled: true },
      { text: 'Hóa đơn điện tử', disabled: true },
      { text: 'Cloud backup', disabled: true },
      { text: 'Hỗ trợ kỹ thuật', disabled: true },
    ],
    cta: { text: 'Tải dùng thử', link: '/tai-ung-dung' },
  },
  {
    name: 'Theo tháng', price: '299.000đ', period: '/tháng', description: 'Linh hoạt, hủy bất cứ lúc nào',
    features: [
      { text: 'Bán hàng không giới hạn' },
      { text: 'AI Chatbot thông minh' },
      { text: 'Hóa đơn điện tử (VNPT)' },
      { text: 'Cloud backup tự động' },
      { text: 'Khôi phục khi đổi máy' },
      { text: 'Cập nhật phiên bản mới' },
      { text: 'Hỗ trợ kỹ thuật qua Zalo' },
    ],
    cta: { text: 'Mua ngay', link: '/lien-he' },
  },
  {
    name: 'Theo năm', price: '1.990.000đ', period: '/năm', description: '≈ 166k/tháng — Tiết kiệm 44%', popular: true,
    features: [
      { text: 'Tất cả tính năng gói tháng' },
      { text: 'Tiết kiệm 44%' },
      { text: 'Ưu tiên hỗ trợ kỹ thuật' },
    ],
    cta: { text: 'Mua ngay', link: '/lien-he' },
  },
]
</script>
