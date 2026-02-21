<template>
  <div class="bg-slate-50 dark:bg-[#0f172a] min-h-screen">
    <!-- Page Header -->
    <div class="page-header relative overflow-hidden">
      <div class="absolute inset-0 bg-grid-dots opacity-20 mix-blend-overlay"></div>
      <div class="absolute top-1/2 left-1/2 w-[800px] h-[800px] bg-green-500/10 rounded-full blur-[100px] -translate-y-1/2 -translate-x-1/2 pointer-events-none"></div>
      <div class="container relative z-10">
        <h1 class="text-4xl md:text-5xl lg:text-6xl font-black mb-4 tracking-tight text-white">Tính năng chi tiết</h1>
        <p class="text-lg md:text-xl text-slate-300 font-light max-w-2xl mx-auto">Mọi công cụ bạn cần để quản lý cửa hàng vật tư nông nghiệp hiệu quả, chuyên nghiệp.</p>
      </div>
    </div>

    <!-- Sticky Sidebar + Content -->
    <div class="py-12 md:py-20 relative">
      <div class="container">
        
        <!-- Mobile Horizontal Tabs -->
        <nav class="lg:hidden flex overflow-x-auto gap-2 pb-4 mb-8 snap-x scrollbar-hide sticky top-[72px] bg-slate-50/80 dark:bg-[#0f172a]/80 backdrop-blur-md z-30 px-4 -mx-4 pt-4 border-b border-slate-200 dark:border-slate-800">
          <a v-for="section in sections" :key="section.id"
             :href="`#${section.id}`"
             class="whitespace-nowrap snap-center px-4 py-2 rounded-xl text-sm font-semibold transition-all duration-300"
             :class="activeSection === section.id 
               ? 'bg-gradient-to-r from-green-500 to-emerald-500 text-white shadow-md' 
               : 'bg-white dark:bg-slate-800 text-[var(--text-muted)] hover:bg-slate-100 dark:hover:bg-slate-700'">
            {{ section.title }}
          </a>
        </nav>

        <div class="flex flex-col lg:flex-row gap-12 xl:gap-20 items-start">
          
          <!-- Desktop Sidebar Navigation -->
          <aside class="hidden lg:block w-[280px] flex-shrink-0 sticky top-32 z-30">
            <nav class="flex flex-col gap-1 pr-6 border-r border-slate-200 dark:border-slate-800">
              <a v-for="section in sections" :key="section.id"
                 :href="`#${section.id}`"
                 class="group flex items-center gap-3 px-4 py-3 rounded-l-xl text-sm transition-all duration-300 relative"
                 :class="activeSection === section.id 
                   ? 'font-bold text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-500/10 before:absolute before:right-[-1px] before:top-0 before:bottom-0 before:w-1 before:bg-green-500 before:rounded-l' 
                   : 'font-medium text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-slate-200'">
                <div class="w-8 h-8 rounded-lg flex items-center justify-center transition-colors"
                     :class="activeSection === section.id ? 'bg-green-500/20 text-green-600 dark:text-green-400' : 'bg-slate-100 dark:bg-slate-800 text-slate-400'">
                  <component :is="section.iconComponent" :size="16" />
                </div>
                {{ section.title }}
              </a>
            </nav>
          </aside>

          <!-- Sections Content -->
          <div class="flex-1 space-y-24 md:space-y-32 w-full max-w-4xl">
            <section v-for="(section, i) in sections" :key="section.id" :id="section.id"
                     class="scroll-mt-32 reveal group"
                     :class="`reveal-delay-${(i % 3) + 1}`">
              
              <div class="flex flex-col md:flex-row gap-8 lg:gap-12 items-center"
                   :class="{ 'md:flex-row-reverse': i % 2 !== 0 }">
                
                <!-- Graphic Side -->
                <div class="w-full md:w-5/12 aspect-square md:aspect-auto md:h-80 rounded-[2rem] bg-gradient-to-br from-slate-200 to-slate-100 dark:from-slate-800 dark:to-[#1a233a] border border-white dark:border-slate-700 shadow-xl relative overflow-hidden flex flex-col items-center justify-center p-8 transition-transform duration-500 group-hover:scale-[1.02]">
                  <!-- Noise and Glow -->
                  <div class="absolute inset-0 noise-overlay opacity-50"></div>
                  <div class="absolute -top-10 -right-10 w-40 h-40 bg-green-500/20 rounded-full blur-2xl"></div>
                  
                  <div class="w-24 h-24 sm:w-32 sm:h-32 rounded-3xl bg-gradient-to-br from-green-500 to-emerald-500 flex items-center justify-center text-white shadow-2xl shadow-green-500/30 transform -rotate-3 group-hover:rotate-0 transition-transform duration-500 relative z-10">
                    <component :is="section.iconComponent" class="w-12 h-12 sm:w-16 sm:h-16 drop-shadow-md" />
                  </div>
                  <div class="mt-8 text-center relative z-10 w-full">
                    <div class="h-2 bg-slate-300/50 dark:bg-slate-600/50 rounded-full w-3/4 mx-auto mb-3"></div>
                    <div class="h-2 bg-slate-200/50 dark:bg-slate-700/50 rounded-full w-1/2 mx-auto"></div>
                  </div>
                </div>

                <!-- Text Content Side -->
                <div class="w-full md:w-7/12 flex flex-col justify-center">
                  <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-green-500/10 text-green-600 dark:text-green-400 text-xs font-bold uppercase tracking-wider mb-5 w-fit">
                    <component :is="section.iconComponent" :size="14" />
                    {{ section.title }}
                  </div>
                  
                  <h2 class="text-3xl font-black mb-4 text-slate-900 dark:text-white tracking-tight">{{ section.title }}</h2>
                  <p class="text-[var(--text-muted)] text-lg mb-8 leading-relaxed font-light">{{ section.desc }}</p>
                  
                  <ul class="space-y-4">
                    <li v-for="item in section.items" :key="item"
                        class="flex items-start gap-4">
                      <div class="w-6 h-6 rounded-full bg-green-500/20 flex items-center justify-center flex-shrink-0 mt-0.5">
                        <Check :size="14" class="text-green-600 dark:text-green-400" />
                      </div>
                      <span class="text-slate-700 dark:text-slate-300 font-medium">{{ item }}</span>
                    </li>
                  </ul>
                </div>

              </div>
            </section>
          </div>
        </div>
      </div>
    </div>

    <!-- CTA -->
    <section class="py-12 md:py-20">
      <div class="container reveal">
        <div class="rounded-[3rem] bg-gradient-to-br from-slate-900 via-[#1a2e4c] to-slate-900 p-10 md:p-20 text-center text-white relative overflow-hidden shadow-2xl">
          <div class="absolute inset-0 noise-overlay opacity-10"></div>
          <div class="absolute top-0 right-0 w-80 h-80 bg-green-500/20 rounded-full -translate-y-1/2 translate-x-1/2 blur-3xl mix-blend-screen pointer-events-none"></div>
          <h2 class="text-3xl md:text-5xl font-black mb-6 relative z-10 tracking-tight">Trải nghiệm Nodi POS ngay hôm nay</h2>
          <p class="text-lg md:text-xl text-slate-300 max-w-2xl mx-auto mb-10 relative z-10 font-light">
            Không cần thẻ tín dụng. Tải về dùng thử đầy đủ tính năng trong 30 ngày.
          </p>
          <NuxtLink to="/tai-ung-dung"
                    class="btn btn-lg bg-gradient-to-r from-green-500 to-emerald-500 text-white hover:shadow-2xl hover:-translate-y-1 transition-all relative z-10 font-bold px-8">
            <Download :size="20" />
            Tải miễn phí cho Windows
          </NuxtLink>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ShoppingCart, Package, Wallet, FileText, BarChart3, Bot, RefreshCw, ClipboardList, FileSpreadsheet, Check, Download } from 'lucide-vue-next'
import { ref, onMounted, onUnmounted } from 'vue'

useHead({
  title: 'Tính năng — Nodi POS',
  meta: [
    { name: 'description', content: 'Tính năng chi tiết của Nodi POS: Bán hàng POS, nhập hàng NCC, công nợ, hóa đơn điện tử, báo cáo, AI chatbot' },
  ],
})

const activeSection = ref('pos')
let observer = null

const sections = [
  {
    id: 'pos', iconComponent: ShoppingCart, title: 'Bán hàng POS',
    desc: 'Bán hàng thao tác dưới 5 giây, hỗ trợ tìm kiếm nhanh, quét mã vạch chuyên nghiệp.',
    items: [
      'Quét mã vạch, tìm kiếm sản phẩm thông minh',
      'Đa đơn vị tính linh hoạt (chai, thùng, lít, kg)',
      'Thanh toán đa phương thức: tiền mặt, chuyển khoản, ghi nợ',
      'In hóa đơn (máy in nhiệt siêu tốc, A4 Laser)',
    ],
  },
  {
    id: 'nhap-hang', iconComponent: Package, title: 'Nhập hàng',
    desc: 'Quản lý toàn diện quy trình nhập hàng, kiểm soát nhà cung cấp chặt chẽ.',
    items: [
      'Quản lý danh sách nhà cung cấp (Công ty, Đại lý)',
      'Tạo phiếu nhập hàng chi tiết, dễ dàng đối soát',
      'Kiểm soát lô hàng, cảnh báo hạn sử dụng',
      'Thanh toán phiếu nhập tự động trừ công nợ',
    ],
  },
  {
    id: 'cong-no', iconComponent: Wallet, title: 'Công nợ',
    desc: 'Phần mềm nhắc nợ tự động, hiển thị chi tiết lịch sử thu chi.',
    items: [
      'Theo dõi công nợ khách hàng theo từng mùa vụ',
      'Quản lý công nợ nhà cung cấp minh bạch',
      'Lịch sử giao dịch, đối soát công nợ chi tiết',
      'Tạo phiếu thu/chi nợ cực kì dễ dàng',
    ],
  },
  {
    id: 'hddt', iconComponent: FileText, title: 'Hóa đơn điện tử',
    desc: 'Giải pháp khởi tạo hóa đơn trực tiếp từ máy tính tiền tiết kiệm thời gian.',
    items: [
      'Tích hợp liền mạch hệ thống VNPT S-Invoice',
      'Phát hành HĐĐT ngay tại màn hình POS',
      'Cho phép tra cứu, gửi lại HĐĐT trực tiếp',
      'Đảm bảo tuân thủ tuyệt đối Nghị định 123/2020/NĐ-CP',
    ],
  },
  {
    id: 'bao-cao', iconComponent: BarChart3, title: 'Báo cáo',
    desc: 'Biểu đồ trực quan giúp nắm bắt sức khỏe kinh doanh tức thời.',
    items: [
      'Thống kê doanh thu theo ngày / tháng / năm',
      'Phân tích lãi gộp / Lỗ chi tiết theo từng hóa đơn',
      'Báo cáo tự động vinh danh top sản phẩm bán chạy',
      'Kiểm soát tồn kho rủi ro: sắp hết, sắp hết hạn',
    ],
  },
  {
    id: 'ai', iconComponent: Bot, title: 'Trợ lý AI',
    desc: 'Báo cáo thông minh dạng hội thoại, trò chuyện tự nhiên như người thật.',
    items: [
      '"Hôm nay bán được bao nhiêu?" → AI phản hồi số liệu ngay',
      '"Liệt kê danh sách nợ xấu?" → AI lập danh sách',
      '"Trong kho còn thuốc A không?" → AI kiểm tra kho',
      'Tiết kiệm vô số thời gian gõ tìm kiếm thủ công',
    ],
  },
  {
    id: 'tra-hang', iconComponent: RefreshCw, title: 'Trả hàng',
    desc: 'Xử lý tình huống trả hàng chuyên nghiệp mà không gây lệch sổ sách.',
    items: [
      'Chọn, quét sản phẩm cần trả từ hóa đơn gốc',
      'Phân loại mã lý do trả: hư hỏng, hết hạn, lỗi NCC',
      'Xử lý hoàn trả bằng tiền mặt hoặc cấn trừ công nợ',
      'Quản lý toàn bộ lịch sử hoàn trả hàng',
    ],
  },
  {
    id: 'chot-so', iconComponent: ClipboardList, title: 'Chốt sổ',
    desc: 'Quy trình cuối ngày an toàn, bảo đảm dòng tiền thực tế khớp với hệ thống.',
    items: [
      'Tổng kết doanh thu tổng hợp chỉ bằng 1 nút bấm',
      'Đối soát sự chênh lệch tiền mặt thực tế vs hệ thống',
      'Khóa dữ liệu an toàn theo ngày',
    ],
  },
  {
    id: 'import', iconComponent: FileSpreadsheet, title: 'Import Excel',
    desc: 'Bắt đầu dùng thử chỉ sau 5 phút với dữ liệu hàng hóa đẩy lên tự động.',
    items: [
      'Tự động map cột hệ thống với cột Excel tiếng Việt',
      'Nhập nhanh kho danh mục lên đến 4500+ sản phẩm BVTV',
      'Preview trực quan trước khi xác nhận lưu dữ liệu',
    ],
  },
]

onMounted(() => {
  // Setup intersection observer for scroll spy
  observer = new IntersectionObserver((entries) => {
    // get visible entries and pick the top one
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        activeSection.value = entry.target.id
      }
    })
  }, {
    rootMargin: '-20% 0px -70% 0px'
  })

  sections.forEach(s => {
    const el = document.getElementById(s.id)
    if (el) observer.observe(el)
  })
})

onUnmounted(() => {
  if (observer) observer.disconnect()
})
</script>

<style scoped>
/* Hidden scrollbar for mobile tabs */
.scrollbar-hide::-webkit-scrollbar {
    display: none;
}
.scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
}
</style>
