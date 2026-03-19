<template>
  <div class="min-h-screen">
    <!-- Hero Section -->
    <section class="relative overflow-hidden bg-gradient-to-br from-emerald-800 via-green-700 to-teal-600 pt-28 pb-16 md:pt-32 md:pb-20">
      <div class="absolute inset-0 overflow-hidden pointer-events-none">
        <div class="absolute -top-24 -right-24 w-96 h-96 bg-white/5 rounded-full blur-3xl"></div>
        <div class="absolute -bottom-32 -left-32 w-[500px] h-[500px] bg-emerald-400/10 rounded-full blur-3xl"></div>
      </div>
      <div class="container relative z-10 text-center">
        <div class="inline-flex items-center gap-2 px-4 py-1.5 bg-white/10 backdrop-blur-sm rounded-full text-emerald-100 text-sm font-medium mb-6">
          <span>📖</span> Help Center
        </div>
        <h1 class="text-3xl md:text-5xl font-extrabold text-white mb-4 leading-tight">
          Hướng Dẫn Sử Dụng <span class="text-emerald-300">Nodi POS</span>
        </h1>
        <p class="text-lg text-emerald-100/80 max-w-xl mx-auto mb-8">
          Tài liệu hướng dẫn chi tiết giúp bạn sử dụng Nodi POS hiệu quả nhất.<br>
          Hỗ trợ cả <strong class="text-white">PC</strong> và <strong class="text-white">Mobile</strong>.
        </p>

        <!-- Search bar -->
        <div class="relative max-w-lg mx-auto">
          <div class="absolute inset-y-0 left-4 flex items-center pointer-events-none">
            <Search :size="20" class="text-slate-400" />
          </div>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Tìm kiếm hướng dẫn... VD: bán hàng, công nợ, khuyến mãi"
            class="w-full pl-12 pr-4 py-4 rounded-2xl bg-white/95 dark:bg-slate-800/95 backdrop-blur-sm text-slate-800 dark:text-slate-200 placeholder-slate-400 shadow-xl border-0 focus:outline-none focus:ring-2 focus:ring-emerald-400 text-sm md:text-base"
          />
          <button
            v-if="searchQuery"
            @click="searchQuery = ''"
            class="absolute inset-y-0 right-4 flex items-center text-slate-400 hover:text-slate-600"
          >
            <X :size="18" />
          </button>
        </div>
      </div>
    </section>

    <!-- Guides listing -->
    <section class="py-12 md:py-16 bg-slate-50 dark:bg-slate-950">
      <div class="container max-w-5xl">
        <!-- No results message -->
        <div v-if="filteredGroups.length === 0" class="text-center py-16">
          <div class="text-5xl mb-4">🔍</div>
          <h3 class="text-xl font-bold text-slate-700 dark:text-slate-300 mb-2">Không tìm thấy kết quả</h3>
          <p class="text-slate-500 dark:text-slate-400">Thử tìm kiếm với từ khóa khác hoặc <button @click="searchQuery = ''" class="text-primary-500 hover:underline font-medium">xem tất cả</button></p>
        </div>

        <!-- Groups -->
        <div v-for="(group, gi) in filteredGroups" :key="gi" class="mb-10 last:mb-0">
          <h2 class="flex items-center gap-3 text-lg md:text-xl font-bold text-slate-800 dark:text-slate-100 mb-4 px-1">
            <span class="text-2xl">{{ group.icon }}</span>
            {{ group.title }}
            <span class="text-xs font-normal text-slate-400 dark:text-slate-500 bg-slate-200 dark:bg-slate-800 rounded-full px-2.5 py-0.5">{{ group.items.length }}</span>
          </h2>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            <NuxtLink
              v-for="item in group.items"
              :key="item.slug"
              :to="'/huong-dan/' + item.slug"
              class="group flex items-start gap-4 bg-white dark:bg-slate-900 rounded-2xl p-5 shadow-card hover:shadow-card-hover transition-all duration-300 hover:-translate-y-0.5 border border-slate-100 dark:border-slate-800"
            >
              <div class="flex-shrink-0 w-11 h-11 rounded-xl bg-gradient-to-br flex items-center justify-center text-xl"
                   :class="group.gradient || 'from-emerald-50 to-green-100 dark:from-emerald-900/40 dark:to-green-900/30'">
                {{ item.icon }}
              </div>
              <div class="flex-1 min-w-0">
                <h3 class="font-semibold text-slate-800 dark:text-slate-100 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors text-[0.95rem] mb-1">
                  {{ item.title }}
                </h3>
                <p class="text-sm text-slate-500 dark:text-slate-400 leading-relaxed line-clamp-2">{{ item.desc }}</p>
              </div>
              <ChevronRight :size="18" class="flex-shrink-0 mt-1 text-slate-300 dark:text-slate-600 group-hover:text-primary-500 group-hover:translate-x-1 transition-all" />
            </NuxtLink>
          </div>
        </div>

        <!-- CTA bottom -->
        <div class="mt-12 text-center bg-gradient-to-r from-emerald-50 to-teal-50 dark:from-emerald-900/20 dark:to-teal-900/20 rounded-2xl p-8 border border-emerald-100 dark:border-emerald-800/30">
          <h3 class="text-lg font-bold text-slate-800 dark:text-slate-100 mb-2">Cần hỗ trợ thêm?</h3>
          <p class="text-slate-500 dark:text-slate-400 text-sm mb-4">Liên hệ đội ngũ Nodi POS qua hotline hoặc trang Liên hệ</p>
          <NuxtLink to="/lien-he" class="inline-flex items-center gap-2 px-6 py-2.5 bg-primary-600 hover:bg-primary-700 text-white rounded-xl font-semibold text-sm transition-colors shadow-sm">
            <MessageCircle :size="16" />
            Liên hệ hỗ trợ
          </NuxtLink>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { Search, X, ChevronRight, MessageCircle } from 'lucide-vue-next'

definePageMeta({ layout: 'default' })
useHead({
  title: 'Hướng dẫn sử dụng Nodi POS - Phần mềm quản lý cửa hàng vật tư nông nghiệp',
  meta: [
    { name: 'description', content: 'Hướng dẫn chi tiết cách sử dụng Nodi POS: bán hàng, quản lý kho, công nợ, hóa đơn điện tử, AI chatbot, và nhiều tính năng khác. Hỗ trợ cả PC và Mobile.' },
    { name: 'keywords', content: 'nodi pos, hướng dẫn sử dụng, phần mềm bán hàng, vật tư nông nghiệp, quản lý kho, hóa đơn điện tử, pos offline' },
    { property: 'og:title', content: 'Hướng dẫn sử dụng Nodi POS' },
    { property: 'og:description', content: 'Hướng dẫn chi tiết cách sử dụng Nodi POS: bán hàng, quản lý kho, công nợ, hóa đơn điện tử, AI chatbot.' },
  ]
})

const searchQuery = ref('')

const allGroups = [
  {
    title: 'Bắt đầu',
    icon: '🚀',
    gradient: 'from-blue-50 to-indigo-100 dark:from-blue-900/40 dark:to-indigo-900/30',
    items: [
      { slug: 'bat-dau-nhanh', icon: '⚡', title: 'Bắt đầu nhanh', desc: 'Tải, cài đặt, kích hoạt license, thêm sản phẩm đầu tiên và bắt đầu bán hàng' },
    ]
  },
  {
    title: 'Bán hàng & Thanh toán',
    icon: '🛒',
    gradient: 'from-emerald-50 to-green-100 dark:from-emerald-900/40 dark:to-green-900/30',
    items: [
      { slug: 'ban-hang', icon: '🛒', title: 'Bán hàng (POS)', desc: 'Tìm SP, quét barcode, thêm giỏ, phím tắt, giao diện PC và Mobile' },
      { slug: 'thanh-toan', icon: '💳', title: 'Thanh toán (Checkout)', desc: 'Chọn KH, giảm giá, đổi điểm loyalty, tiền mặt/CK/ghi nợ/kết hợp, VietQR, HĐĐT' },
      { slug: 'lich-su-don-hang', icon: '📋', title: 'Lịch sử đơn hàng', desc: 'Xem, lọc, trả hàng, xuất Excel, theo dõi trạng thái HĐĐT' },
    ]
  },
  {
    title: 'Quản lý Kho & Sản phẩm',
    icon: '📦',
    gradient: 'from-amber-50 to-orange-100 dark:from-amber-900/40 dark:to-orange-900/30',
    items: [
      { slug: 'kho-hang', icon: '📦', title: 'Quản lý kho hàng', desc: 'Danh sách SP, bộ lọc thông minh, lô hàng, kiểm kê, gợi ý AI 5,700+ SP, xuất Excel' },
    ]
  },
  {
    title: 'Khách hàng & Nhà cung cấp',
    icon: '👥',
    gradient: 'from-violet-50 to-purple-100 dark:from-violet-900/40 dark:to-purple-900/30',
    items: [
      { slug: 'khach-hang-cong-no', icon: '👥', title: 'Khách hàng & Công nợ', desc: 'Quản lý KH, sổ nợ, thu nợ, hạn mức tín dụng, xuất Excel' },
      { slug: 'nha-cung-cap', icon: '🚛', title: 'Nhà cung cấp & Nhập hàng', desc: 'Quản lý NCC, phiếu nhập, trả nợ NCC, lịch sử giao dịch' },
    ]
  },
  {
    title: 'Tài chính & Thuế',
    icon: '💰',
    gradient: 'from-yellow-50 to-amber-100 dark:from-yellow-900/40 dark:to-amber-900/30',
    items: [
      { slug: 'so-quy', icon: '💰', title: 'Sổ quỹ (Thu Chi)', desc: 'Tạo phiếu thu/chi, biểu đồ, lọc theo ngày, xuất Excel' },
      { slug: 'bao-cao', icon: '📊', title: 'Báo cáo & Thống kê', desc: 'KPI doanh thu/lợi nhuận, biểu đồ, top SP bán chạy' },
      { slug: 'thue-ke-toan', icon: '🧾', title: 'Thuế & Kế toán', desc: 'Thuế khoán theo quý, TNCN, VAT breakdown, dual-mode trắng/xám' },
      { slug: 'hoa-don-dien-tu', icon: '📄', title: 'Hóa đơn điện tử', desc: 'Cài đặt VNPT/Viettel/MISA, xuất HĐĐT, theo dõi trạng thái' },
    ]
  },
  {
    title: 'Tính năng nâng cao',
    icon: '⭐',
    gradient: 'from-rose-50 to-pink-100 dark:from-rose-900/40 dark:to-pink-900/30',
    items: [
      { slug: 'khuyen-mai-voucher', icon: '🎁', title: 'Khuyến mãi & Voucher', desc: 'Tạo KM giảm giá %, cố định, mua X tặng Y, quản lý voucher' },
      { slug: 'tich-diem', icon: '⭐', title: 'Tích điểm Loyalty', desc: 'Cài đặt tích điểm, quy đổi điểm, hạng thẻ Bronze→Diamond' },
      { slug: 'chot-so', icon: '📋', title: 'Chốt sổ cuối ngày', desc: 'Tổng kết doanh thu, đối soát tiền mặt, lịch sử chốt sổ' },
      { slug: 'nhan-vien', icon: '👨‍💼', title: 'Nhân viên & Phân quyền', desc: 'Thêm NV, tạo PIN, phân quyền 14 flags, chuyển ca' },
    ]
  },
  {
    title: 'Công nghệ & Hệ thống',
    icon: '🔧',
    gradient: 'from-cyan-50 to-sky-100 dark:from-cyan-900/40 dark:to-sky-900/30',
    items: [
      { slug: 'ai-chatbot', icon: '🤖', title: 'AI Chatbot thông minh', desc: 'Bán hàng bằng giọng nói, tra cứu kho/nợ/giá, chẩn đoán bệnh cây, 100% offline' },
      { slug: 'cai-dat-may-in', icon: '⚙️', title: 'Cài đặt & Máy in', desc: 'Thông tin cửa hàng, máy in nhiệt 80mm, backup, QR thanh toán' },
      { slug: 'mobile-app', icon: '📱', title: 'Mobile App', desc: 'Bottom nav, FAB bán hàng, thông báo thông minh, dark mode' },
      { slug: 'cloud-dong-bo', icon: '☁️', title: 'Cloud & Đồng bộ', desc: 'Offline-first, multi-device sync 10 thiết bị, cloud backup tự động' },
      { slug: 'vietqr', icon: '💸', title: 'VietQR & Thanh toán', desc: 'Cài đặt STK, sinh QR tự động, hiển thị trên màn khách' },
    ]
  },
  {
    title: 'Trợ giúp',
    icon: '❓',
    gradient: 'from-slate-50 to-gray-100 dark:from-slate-900/40 dark:to-gray-900/30',
    items: [
      { slug: 'faq', icon: '❓', title: 'Câu hỏi thường gặp (FAQ)', desc: '15+ câu hỏi phổ biến: offline, free vs pro, import Excel, dark mode...' },
    ]
  },
]

const filteredGroups = computed(() => {
  if (!searchQuery.value.trim()) return allGroups

  const q = searchQuery.value.toLowerCase().trim()
  const result = []

  for (const group of allGroups) {
    const matched = group.items.filter(item =>
      item.title.toLowerCase().includes(q) ||
      item.desc.toLowerCase().includes(q) ||
      item.slug.includes(q)
    )
    if (matched.length > 0) {
      result.push({ ...group, items: matched })
    }
  }
  return result
})
</script>
