<template>
  <div>
    <div class="page-header">
      <div class="container">
        <h1>Bảng giá Nodi POS</h1>
        <p>Chọn gói phù hợp với cửa hàng của bạn</p>
      </div>
    </div>

    <!-- Pricing Cards -->
    <section class="section">
      <div class="container">
        <div class="grid-3 items-start">
          <PricingCard v-for="p in plans" :key="p.name" v-bind="p" class="reveal" />
        </div>
      </div>
    </section>

    <!-- FAQ -->
    <section class="section section-alt">
      <div class="container">
        <h2 class="section-title reveal">Câu hỏi thường gặp</h2>
        <div class="max-w-2xl mx-auto space-y-3 mt-10">
          <div v-for="(faq, i) in faqs" :key="i"
               class="card p-0 overflow-hidden reveal"
               :class="`reveal-delay-${Math.min(i+1, 4)}`">
            <button @click="toggleFaq(i)"
                    class="w-full flex items-center justify-between px-6 py-5 text-left font-semibold text-slate-800 dark:text-slate-100 hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors">
              <span>{{ faq.q }}</span>
              <ChevronDown :size="18"
                           class="text-[var(--text-muted)] transition-transform duration-300 flex-shrink-0 ml-4"
                           :class="{ 'rotate-180': openFaq === i }" />
            </button>
            <Transition
              enter-active-class="transition-all duration-300 ease-out"
              enter-from-class="opacity-0 max-h-0"
              enter-to-class="opacity-100 max-h-40"
              leave-active-class="transition-all duration-200 ease-in"
              leave-from-class="opacity-100 max-h-40"
              leave-to-class="opacity-0 max-h-0">
              <div v-if="openFaq === i" class="overflow-hidden">
                <p class="px-6 pb-5 text-[var(--text-muted)] leading-relaxed">{{ faq.a }}</p>
              </div>
            </Transition>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ChevronDown } from 'lucide-vue-next'

useHead({
  title: 'Bảng giá — Nodi POS',
  meta: [{ name: 'description', content: 'Bảng giá Nodi POS: Dùng thử miễn phí 30 ngày, gói tháng 299.000đ, gói năm 1.990.000đ (tiết kiệm 44%).' }],
  script: [{ type: 'application/ld+json', innerHTML: JSON.stringify({
    '@context': 'https://schema.org', '@type': 'Product', name: 'Nodi POS',
    description: 'Phần mềm quản lý cửa hàng vật tư nông nghiệp',
    offers: [
      { '@type': 'Offer', name: 'Dùng thử', price: '0', priceCurrency: 'VND' },
      { '@type': 'Offer', name: 'Theo tháng', price: '299000', priceCurrency: 'VND' },
      { '@type': 'Offer', name: 'Theo năm', price: '1990000', priceCurrency: 'VND' },
    ]
  }) }]
})

const openFaq = ref(null)
const toggleFaq = (i) => { openFaq.value = openFaq.value === i ? null : i }

const plans = [
  {
    name: 'Dùng thử', price: 'Miễn phí', period: '', description: '30 ngày đầy đủ tính năng. Sau 30 ngày: miễn phí giới hạn 20 đơn/ngày',
    features: [
      { text: 'Bán hàng POS (giới hạn 20 đơn/ngày sau 30 ngày)' },
      { text: 'Quản lý sản phẩm, khách hàng, NCC' },
      { text: 'Nhập hàng, tồn kho' },
      { text: 'Báo cáo cơ bản' },
      { text: 'Xuất Excel' },
      { text: 'AI Chatbot', disabled: true },
      { text: 'Hóa đơn điện tử', disabled: true },
      { text: 'Cloud backup & khôi phục', disabled: true },
      { text: 'Hỗ trợ kỹ thuật', disabled: true },
    ],
    cta: { text: 'Tải dùng thử', link: '/tai-ung-dung' },
  },
  {
    name: 'Theo tháng', price: '299.000đ', period: '/tháng', description: 'Linh hoạt, hủy bất cứ lúc nào',
    features: [
      { text: 'Bán hàng không giới hạn' },
      { text: 'Quản lý sản phẩm, khách hàng, NCC' },
      { text: 'Nhập hàng, tồn kho, lô hàng' },
      { text: 'Báo cáo chi tiết (doanh thu, lãi/lỗ, top SP)' },
      { text: 'Xuất Excel' },
      { text: 'AI Chatbot thông minh' },
      { text: 'Hóa đơn điện tử (VNPT)' },
      { text: 'Cloud backup tự động' },
      { text: 'Khôi phục data khi đổi máy' },
      { text: 'Cập nhật phiên bản mới' },
      { text: 'Hỗ trợ kỹ thuật qua Zalo' },
    ],
    cta: { text: 'Mua ngay', link: '/lien-he' },
  },
  {
    name: 'Theo năm', price: '1.990.000đ', period: '/năm', description: '≈ 166k/tháng — Tiết kiệm 44%', popular: true,
    features: [
      { text: 'Tất cả tính năng gói tháng' },
      { text: 'Tiết kiệm 44% so với gói tháng' },
      { text: 'Ưu tiên hỗ trợ kỹ thuật' },
    ],
    cta: { text: 'Mua ngay', link: '/lien-he' },
  },
]

const faqs = [
  { q: 'Cài được mấy máy?', a: '1 license = 1 máy tính. Nếu bạn cần dùng nhiều máy, liên hệ để mua thêm license.' },
  { q: 'Đổi máy tính được không?', a: 'Được. Liên hệ hỗ trợ qua Zalo 0374.222.326 để reset HWID, chuyển license sang máy mới.' },
  { q: 'Dữ liệu cũ có mất không?', a: 'Không! Dữ liệu được backup lên cloud. Cài máy mới, đăng nhập lại và khôi phục chỉ 1 click.' },
  { q: 'Thanh toán như thế nào?', a: 'Chuyển khoản ngân hàng hoặc liên hệ qua Zalo. Kích hoạt license trong vòng 24 giờ sau thanh toán.' },
  { q: 'Sau 30 ngày dùng thử thì sao?', a: 'Phần mềm vẫn sử dụng được miễn phí nhưng giới hạn 20 đơn hàng/ngày. Nâng cấp PRO để không giới hạn.' },
]
</script>
