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
            <nav class="flex flex-col gap-1 pr-6 border-r border-slate-200 dark:border-slate-800 max-h-[calc(100vh-10rem)] overflow-y-auto scrollbar-hide">
              <a v-for="section in sections" :key="section.id"
                 :href="`#${section.id}`"
                 class="group flex items-center gap-3 px-4 py-2.5 rounded-l-xl text-sm transition-all duration-300 relative"
                 :class="activeSection === section.id 
                   ? 'font-bold text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-500/10 before:absolute before:right-[-1px] before:top-0 before:bottom-0 before:w-1 before:bg-green-500 before:rounded-l' 
                   : 'font-medium text-slate-500 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-slate-200'">
                <div class="w-7 h-7 rounded-lg flex items-center justify-center transition-colors flex-shrink-0"
                     :class="activeSection === section.id ? 'bg-green-500/20 text-green-600 dark:text-green-400' : 'bg-slate-100 dark:bg-slate-800 text-slate-400'">
                  <component :is="section.iconComponent" :size="14" />
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
import { 
  ShoppingCart, Package, Wallet, FileText, BarChart3, Bot, RefreshCw, 
  ClipboardList, FileSpreadsheet, Check, Download, Warehouse, DollarSign,
  Calculator, Users, Cloud, UserCheck, Shield, Printer, Settings, Smartphone,
  QrCode, Lock, Store
} from 'lucide-vue-next'
import { ref, onMounted, onUnmounted } from 'vue'

useHead({
  title: 'Tính năng — Nodi POS',
  meta: [
    { name: 'description', content: 'Tính năng chi tiết của Nodi POS: Bán hàng POS, quản lý kho, nhập hàng NCC, công nợ, hóa đơn điện tử, báo cáo, AI chatbot, quản lý nhân viên, thu chi, đồng bộ cloud' },
  ],
})

const activeSection = ref('pos')
let observer = null

const sections = [
  {
    id: 'pos', iconComponent: ShoppingCart, title: 'Bán hàng POS',
    desc: 'Bán hàng thao tác dưới 5 giây. Hỗ trợ quét mã vạch, thanh toán đa kênh, VietQR và ghi nợ ngay tại quầy.',
    items: [
      'Quét mã vạch + Remote Scanner (quét qua điện thoại)',
      'Tìm kiếm nhanh theo tên, barcode, hoạt chất',
      'Đa đơn vị tính linh hoạt (chai, thùng, lít, kg) — quy đổi tự động',
      'Thanh toán: tiền mặt, chuyển khoản, kết hợp',
      'Thanh toán VietQR (Napas) — tạo mã QR tự động, hiển thị màn hình phụ',
      'Ghi nợ khách hàng ngay tại quầy checkout',
      'Toggle xuất hóa đơn VAT / bán lẻ (Dual-Mode)',
      'In hóa đơn qua máy in nhiệt (58mm/80mm)',
      'Gán khách hàng vào đơn để theo dõi lịch sử mua',
    ],
  },
  {
    id: 'kho-hang', iconComponent: Warehouse, title: 'Quản lý kho hàng',
    desc: 'Quản lý tồn kho, lô hàng, hạn sử dụng với hệ thống FEFO tự động.',
    items: [
      'Danh sách sản phẩm — tồn kho, giá bán, giá vốn real-time',
      'Quản lý lô hàng (Batch) — theo dõi ngày sản xuất, ngày hết hạn',
      'FEFO/FIFO — tự động bán lô hết hạn trước',
      'Cảnh báo hết hạn (≤30 ngày, ≤7 ngày) và hết hàng (tồn kho ≤5)',
      'Phát hiện hàng bán chậm — sản phẩm lâu không bán',
      'Kiểm kho (Stocktake) — nhập thực tế, tính chênh lệch tự động',
      'Xuất Excel tồn kho (.xlsx)',
    ],
  },
  {
    id: 'nhap-hang', iconComponent: Package, title: 'Nhập hàng & NCC',
    desc: 'Quản lý toàn diện quy trình nhập hàng, kiểm soát nhà cung cấp và công nợ chặt chẽ.',
    items: [
      'Tạo phiếu nhập hàng — chọn NCC, thêm sản phẩm, số lượng, giá nhập',
      'Ghi nợ nhà cung cấp — tự động tính công nợ khi nhập hàng',
      'Lịch sử nhập hàng — lọc theo ngày, NCC, trạng thái',
      'Chi tiết NCC — lịch sử nhập, công nợ, tổng giao dịch',
      'Thanh toán nợ NCC — trả từng phần hoặc toàn bộ',
      'Snapshot nợ NCC — lưu snapshot theo ngày để đối soát',
      'Tự động cập nhật tồn kho sau khi nhập',
    ],
  },
  {
    id: 'khach-hang', iconComponent: UserCheck, title: 'Khách hàng & Công nợ',
    desc: 'Quản lý khách hàng, công nợ, lịch sử giao dịch với phân khúc thông minh.',
    items: [
      'Danh sách khách hàng — tên, SĐT, địa chỉ, ghi chú',
      'Quản lý công nợ — ghi nợ, thu nợ, lịch sử chi tiết',
      'Thu nợ từng phần hoặc toàn bộ, theo dõi timeline giao dịch',
      'Phân khúc khách hàng — VIP, thường xuyên, mới, lâu không mua (churn)',
      'Chi tiết khách — tổng mua, tổng nợ, lịch sử mua hàng đầy đủ',
    ],
  },
  {
    id: 'thu-chi', iconComponent: DollarSign, title: 'Thu chi (Cashflow)',
    desc: 'Sổ thu chi giúp ghi nhận mọi khoản thu/chi ngoài bán hàng, kiểm soát dòng tiền cửa hàng.',
    items: [
      'Sổ thu chi — ghi nhận thu/chi ngoài bán hàng (tiền thuê, điện, lương...)',
      'Phân loại giao dịch — danh mục thu/chi tùy chỉnh',
      'Tổng hợp — thu - chi = tồn quỹ theo ngày/tháng',
      'Lọc theo thời gian — xem thu chi theo khoảng ngày bất kỳ',
    ],
  },
  {
    id: 'hddt', iconComponent: FileText, title: 'Hóa đơn điện tử',
    desc: 'Giải pháp khởi tạo hóa đơn trực tiếp từ máy tính tiền, tuân thủ pháp luật thuế.',
    items: [
      'Tích hợp VNPT S-Invoice, Viettel, CK-S',
      'Phát hành HĐĐT ngay tại màn hình POS',
      'Tra cứu, gửi lại HĐĐT trực tiếp',
      'Xuất XML hóa đơn theo chuẩn — import vào eTax miễn phí',
      'Tuân thủ Nghị định 123/2020/NĐ-CP',
    ],
  },
  {
    id: 'thue', iconComponent: Calculator, title: 'Thuế & Kế toán',
    desc: 'Dual-Mode báo cáo thuế, bảng VAT chi tiết, cảnh báo ngưỡng doanh thu — giúp chủ cửa hàng yên tâm tuân thủ thuế.',
    items: [
      'Dual-Mode — tách riêng doanh thu có hóa đơn (khai thuế) vs bán lẻ không hóa đơn',
      'Bảng VAT chi tiết — phân tách thuế suất 0%, 5%, 10% từng tháng',
      'Cảnh báo ngưỡng thuế — nhắc khi doanh thu gần 500M (thuế khoán) hoặc 1 tỷ (HĐĐT)',
      'Thuế suất sản phẩm — gán riêng cho từng SP (0%/5%/10%)',
      'Kiến thức thuế 125+ topics — VAT, CIT, PIT, NĐ 123, giấy phép BVTV...',
    ],
  },
  {
    id: 'bao-cao', iconComponent: BarChart3, title: 'Báo cáo & Thống kê',
    desc: 'Biểu đồ trực quan giúp nắm bắt sức khỏe kinh doanh tức thời.',
    items: [
      'Doanh thu theo ngày, tuần, tháng, quý, năm, tùy chọn khoảng thời gian',
      'Lợi nhuận tính tự động từ giá bán - giá vốn',
      'Sản phẩm bán chạy — top products theo số lượng hoặc doanh thu',
      'So sánh doanh thu — so sánh 2 khoảng thời gian (tháng này vs tháng trước)',
      'Biểu đồ cột, đường, tròn trực quan',
      'Xuất báo cáo Excel (.xlsx)',
    ],
  },
  {
    id: 'ai', iconComponent: Bot, title: 'Trợ lý AI Nodi',
    desc: 'AI thông minh 100% offline, hiểu tiếng Việt tự nhiên, hỗ trợ bán hàng bằng chat, chẩn đoán 315+ bệnh cây trồng.',
    items: [
      '46 lệnh AI — bán hàng, tra cứu kho, báo cáo bằng chat tự nhiên',
      '"Lấy 2 chai Beam 75WP" → thêm đúng SP vào giỏ hàng',
      'Chuyên gia bệnh cây trồng — 315+ bệnh trên Lúa, Sầu riêng, Cà phê...',
      'Guided Diagnostic — hỏi từng bước khi triệu chứng mơ hồ',
      'Tra cứu doanh thu, tồn kho, công nợ bằng câu hỏi tự nhiên',
      'Self-Learning — học từ feedback 👍/👎, tự cải thiện',
      '7-Layer Fallback — luôn có câu trả lời, graceful degradation',
      'Cảnh báo mùa vụ — nhắc sâu bệnh theo mùa trong năm',
      'Kiến thức thuế 125+ topics — VAT, CIT, PIT, NĐ 123, giấy phép BVTV',
    ],
  },
  {
    id: 'nhan-vien', iconComponent: Users, title: 'Quản lý nhân viên',
    desc: 'Phân quyền chi tiết 9 quyền, đăng nhập PIN 4 số, đổi ca nhanh — kiểm soát truy cập an toàn.',
    items: [
      'Đăng nhập bằng mã PIN 4 số — mỗi NV có PIN riêng',
      'Phân quyền chi tiết (9 quyền): bán hàng, kho, doanh thu, báo cáo, KH, thu chi...',
      'RBAC — nhân viên chỉ thấy menu/trang được phân quyền',
      'Màn hình khóa — khóa máy POS, yêu cầu PIN để mở lại',
      'Đổi ca — chuyển nhanh giữa các nhân viên bằng PIN',
    ],
  },
  {
    id: 'don-hang', iconComponent: ClipboardList, title: 'Đơn hàng & Trả hàng',
    desc: 'Lịch sử đơn hàng đầy đủ, lọc theo ngày, trạng thái. Xử lý trả hàng chuyên nghiệp.',
    items: [
      'Danh sách đơn hàng — lọc theo ngày, trạng thái, phương thức thanh toán',
      'Filter hóa đơn VAT — lọc riêng đơn có xuất HĐ vs bán lẻ',
      'In lại hóa đơn bất kỳ đơn hàng nào',
      'Trả hàng (Return) — hoàn tiền/tồn kho, phân loại lý do trả',
      'Chốt sổ cuối ngày — tổng kết doanh thu, số đơn, tồn quỹ',
    ],
  },
  {
    id: 'cloud', iconComponent: Cloud, title: 'Đồng bộ Cloud & Sao lưu',
    desc: 'Hoạt động 100% offline, đồng bộ tự động khi có mạng. Cloud backup an toàn, khôi phục 1 click.',
    items: [
      'Offline-first — hoạt động 100% offline, sync khi có mạng',
      'Sync tự động — đồng bộ dữ liệu lên server mỗi 60 giây',
      'Exponential backoff — server lỗi tự giãn retry (60s → 120s → 10 phút)',
      'Cloud backup — sao lưu database lên cloud, lên lịch tự động',
      'Khôi phục từ cloud — restore database từ bản backup, chỉ 1 click',
      'Sync indicator — hiển thị trạng thái đồng bộ trên giao diện',
    ],
  },
  {
    id: 'bao-mat', iconComponent: Shield, title: 'Bảo mật & Bản quyền',
    desc: 'Kích hoạt bằng license key, xác thực HWID, đăng nhập/đăng ký bảo mật JWT.',
    items: [
      'Kích hoạt bằng license key — nhập key để kích hoạt Pro',
      'Chế độ Free/Trial — dùng thử đầy đủ tính năng trong 30 ngày',
      'Xác thực HWID — mỗi máy tính có định danh duy nhất',
      'Đăng nhập/đăng ký tài khoản — quản lý từ xa qua web',
      'JWT authentication — xác thực bảo mật cho API cloud',
    ],
  },
  {
    id: 'in-an', iconComponent: Printer, title: 'In ấn',
    desc: 'In hóa đơn bán hàng, phiếu nhập, công nợ qua máy in nhiệt hoặc A4.',
    items: [
      'In hóa đơn bán hàng — máy in nhiệt 58mm/80mm, A4 Laser',
      'In phiếu nhập hàng, phiếu công nợ khách hàng',
      'Cấu hình máy in — chọn máy in, kích thước giấy, logo',
    ],
  },
  {
    id: 'he-thong', iconComponent: Settings, title: 'Cài đặt & Tiện ích',
    desc: 'Cấu hình cửa hàng, giao diện, AI, cập nhật tự động OTA, hỗ trợ kỹ thuật.',
    items: [
      'Thông tin cửa hàng — tên, SĐT, địa chỉ, logo (hiển thị trên hóa đơn)',
      'Dark Mode / Light Mode — giao diện tối giảm mỏi mắt',
      'Tour hướng dẫn — hướng dẫn sử dụng cho người dùng mới',
      'Chat trực tiếp KTV — chat real-time với đội ngũ kỹ thuật',
      'Cập nhật OTA — kiểm tra phiên bản mới, tải về và cài đặt trong app',
      'Quản lý database — xem dung lượng, xóa dữ liệu cũ, reset',
    ],
  },
  {
    id: 'import', iconComponent: FileSpreadsheet, title: 'Import Excel',
    desc: 'Bắt đầu dùng thử chỉ sau 5 phút với dữ liệu hàng hóa đẩy lên tự động.',
    items: [
      'Tự động map cột hệ thống với cột Excel tiếng Việt',
      'Nhập nhanh kho danh mục sản phẩm từ file Excel',
      'Preview trực quan trước khi xác nhận lưu dữ liệu',
    ],
  },
  {
    id: 'da-cua-hang', iconComponent: Store, title: 'Quản lý đa cửa hàng',
    desc: '1 tài khoản quản lý nhiều cửa hàng/chi nhánh. Dữ liệu tách biệt, chuyển đổi nhanh.',
    items: [
      'Tạo và quản lý nhiều cửa hàng/chi nhánh trên 1 tài khoản',
      'Mỗi cửa hàng sử dụng 1 license key riêng',
      'Mỗi cửa hàng hỗ trợ tối đa 10 thiết bị (PC + Mobile)',
      'Chuyển đổi giữa các cửa hàng chỉ 1 click',
      'Dữ liệu hoàn toàn tách biệt giữa các cửa hàng',
      'Xem thống kê từng cửa hàng trên web dashboard',
    ],
  },
]

onMounted(() => {
  // Setup intersection observer for scroll spy
  observer = new IntersectionObserver((entries) => {
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
/* Hidden scrollbar for mobile tabs & sidebar */
.scrollbar-hide::-webkit-scrollbar {
    display: none;
}
.scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
}
</style>
