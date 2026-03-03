<template>
  <div class="bg-slate-50 dark:bg-[#0f172a] min-h-screen">
    <!-- Page Header -->
    <div class="page-header relative overflow-hidden">
      <div class="absolute inset-0 bg-grid-dots opacity-20 mix-blend-overlay"></div>
      <div class="absolute top-1/2 left-1/2 w-[800px] h-[800px] bg-green-500/10 rounded-full blur-[100px] -translate-y-1/2 -translate-x-1/2 pointer-events-none"></div>
      <div class="container relative z-10">
        <h1 class="text-4xl md:text-5xl lg:text-6xl font-black mb-4 tracking-tight text-white">Giới thiệu Nodi POS</h1>
        <p class="text-lg md:text-xl text-slate-300 font-light max-w-2xl mx-auto">Phần mềm quản lý bán hàng thế hệ mới — thiết kế riêng cho cửa hàng vật tư nông nghiệp Việt Nam.</p>
      </div>
    </div>

    <!-- Story Section -->
    <section class="py-16 md:py-24">
      <div class="container">
        <div class="flex flex-col lg:flex-row gap-12 lg:gap-20 items-center">
          <div class="flex-1 reveal">
            <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-green-500/10 text-green-600 dark:text-green-400 text-xs font-bold uppercase tracking-wider mb-6">
              <Leaf :size="14" />
              Câu chuyện sản phẩm
            </div>
            <h2 class="text-3xl md:text-4xl font-black mb-6 text-slate-900 dark:text-white tracking-tight">
              Phần mềm POS <span class="text-transparent bg-clip-text bg-gradient-to-r from-green-500 to-emerald-500">duy nhất</span> hiểu ngành vật tư nông nghiệp
            </h2>
            <div class="space-y-4 text-lg text-slate-600 dark:text-slate-300 leading-relaxed font-light">
              <p>
                Cửa hàng vật tư nông nghiệp không giống bất kỳ ngành nào khác. Bạn cần quản lý 
                <strong class="text-slate-900 dark:text-white font-medium">hàng ngàn loại thuốc BVTV có hạn sử dụng</strong>, 
                theo dõi <strong class="text-slate-900 dark:text-white font-medium">công nợ theo mùa vụ</strong> cho hàng trăm nông dân, 
                xuất hóa đơn với <strong class="text-slate-900 dark:text-white font-medium">nhiều mức thuế suất khác nhau</strong>, 
                và tất cả phải hoạt động khi <strong class="text-slate-900 dark:text-white font-medium">mạng internet chập chờn</strong>.
              </p>
              <p>
                Nodi POS được sinh ra từ thực tế ấy. Không phải phần mềm POS phổ thông rồi "gắn thêm" tính năng, 
                mà được <strong class="text-slate-900 dark:text-white font-medium">thiết kế từ gốc</strong> cho đại lý vật tư — 
                từ quản lý lô hàng FEFO, dual-mode thuế, đến trợ lý AI có thể chẩn đoán bệnh cây trồng ngay tại quầy.
              </p>
            </div>
          </div>
          <!-- Key Stats -->
          <div class="flex-shrink-0 w-full lg:w-auto reveal reveal-delay-2">
            <div class="grid grid-cols-2 gap-4">
              <div v-for="stat in stats" :key="stat.label" 
                   class="card-glass p-6 text-center hover:border-green-500/50 hover:-translate-y-1 transition-all duration-300">
                <div class="text-3xl md:text-4xl font-black text-transparent bg-clip-text bg-gradient-to-r from-green-500 to-emerald-500 mb-2">{{ stat.value }}</div>
                <div class="text-sm text-[var(--text-muted)] font-medium">{{ stat.label }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- Core Capabilities -->
    <section class="py-16 md:py-24 relative overflow-hidden">
      <div class="absolute top-0 right-0 w-[800px] h-[800px] bg-primary/5 rounded-full blur-[120px] -translate-y-1/2 translate-x-1/2 pointer-events-none"></div>
      <div class="container relative z-10">
        <div class="text-center mb-16 reveal">
          <h2 class="section-title">Tất cả trong <span class="text-gradient">một ứng dụng</span></h2>
          <p class="section-subtitle">16 nhóm tính năng chuyên sâu, bao phủ mọi nghiệp vụ cửa hàng VTNN</p>
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <div v-for="(cap, i) in capabilities" :key="cap.title"
               class="card-glass group cursor-default p-6 hover:border-green-500/50 hover:shadow-xl hover:shadow-green-500/5 hover:-translate-y-1 transition-all duration-300 reveal"
               :class="`reveal-delay-${(i % 4) + 1}`">
            <div class="w-12 h-12 mb-4 rounded-xl flex items-center justify-center transition-transform duration-300 group-hover:scale-110"
                 :class="cap.bgClass">
              <component :is="cap.iconComponent" :size="24" :class="cap.iconClass" />
            </div>
            <h3 class="text-lg font-bold mb-2 text-slate-900 dark:text-slate-100">{{ cap.title }}</h3>
            <p class="text-sm text-[var(--text-muted)] leading-relaxed">{{ cap.desc }}</p>
          </div>
        </div>
        <div class="text-center mt-12 reveal">
          <NuxtLink to="/tinh-nang" class="btn btn-outline border-slate-300 dark:border-slate-600 hover:border-primary">
            Xem chi tiết 16+ tính năng
            <ArrowRight :size="16" />
          </NuxtLink>
        </div>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- HĐĐT & Tax — Highlight Section -->
    <section class="py-16 md:py-24 bg-gradient-to-br from-slate-900 via-[#0a1128] to-slate-900 relative overflow-hidden">
      <div class="absolute inset-0 bg-grid-dots opacity-10 mix-blend-overlay"></div>
      <div class="absolute top-1/4 left-0 w-[500px] h-[500px] bg-green-500/10 rounded-full blur-[100px] -translate-x-1/2 pointer-events-none"></div>
      <div class="absolute bottom-0 right-0 w-[400px] h-[400px] bg-amber-500/10 rounded-full blur-[100px] translate-x-1/3 pointer-events-none"></div>

      <div class="container relative z-10">
        <div class="text-center mb-16 reveal">
          <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-amber-500/10 border border-amber-500/20 text-amber-400 text-sm font-bold mb-6">
            <FileText :size="16" />
            Hóa đơn điện tử & Thuế
          </div>
          <h2 class="text-3xl md:text-5xl font-black text-white tracking-tight mb-6">
            Yên tâm về thuế, <br class="hidden md:block" />
            <span class="text-transparent bg-clip-text bg-gradient-to-r from-amber-400 to-yellow-300">tập trung kinh doanh</span>
          </h2>
          <p class="text-lg md:text-xl text-slate-300 max-w-3xl mx-auto font-light leading-relaxed">
            Nodi POS là phần mềm POS nông nghiệp duy nhất có hệ thống thuế Dual-Mode — giúp bạn 
            tách riêng doanh thu có hóa đơn VAT và bán lẻ, tự động tính thuế, cảnh báo ngưỡng, 
            và xuất hóa đơn điện tử ngay tại quầy.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-12">
          <div v-for="(tax, i) in taxFeatures" :key="tax.title"
               class="bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl p-8 hover:bg-white/10 hover:border-green-500/30 transition-all duration-300 reveal"
               :class="`reveal-delay-${(i % 3) + 1}`">
            <div class="w-12 h-12 mb-5 rounded-xl flex items-center justify-center"
                 :class="tax.bgClass">
              <component :is="tax.iconComponent" :size="24" :class="tax.iconClass" />
            </div>
            <h3 class="text-xl font-bold text-white mb-3">{{ tax.title }}</h3>
            <p class="text-slate-300 text-sm leading-relaxed">{{ tax.desc }}</p>
          </div>
        </div>

        <!-- Tax Knowledge Highlight -->
        <div class="reveal">
          <div class="bg-gradient-to-r from-amber-500/10 to-green-500/10 border border-amber-500/20 rounded-2xl p-8 md:p-10 flex flex-col md:flex-row gap-8 items-center">
            <div class="w-20 h-20 md:w-24 md:h-24 rounded-2xl bg-gradient-to-br from-amber-500 to-yellow-500 flex items-center justify-center flex-shrink-0 shadow-xl shadow-amber-500/20">
              <GraduationCap :size="40" class="text-white" />
            </div>
            <div class="flex-1 text-center md:text-left">
              <h3 class="text-2xl font-black text-white mb-3">125+ Kiến thức thuế tích hợp sẵn</h3>
              <p class="text-slate-300 leading-relaxed">
                Hỏi AI về VAT thuốc BVTV, thuế suất phân bón, Nghị định 123, hóa đơn đầu vào, kê khai thuế khoán, 
                giấy phép kinh doanh BVTV — tất cả đều có sẵn trong app. Không cần google, không cần hỏi kế toán. 
                AI trả lời bằng <strong class="text-white">tiếng Việt dễ hiểu</strong>, kèm căn cứ pháp lý cụ thể.
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- AI Showcase -->
    <section class="py-16 md:py-24 relative overflow-hidden">
      <div class="absolute bottom-0 left-0 w-[600px] h-[600px] bg-violet-500/5 rounded-full blur-[100px] translate-y-1/2 -translate-x-1/2 pointer-events-none"></div>
      <div class="container relative z-10">
        <div class="text-center mb-16 reveal">
          <h2 class="section-title">Trợ lý AI <span class="text-gradient">thông minh nhất</span> ngành POS</h2>
          <p class="section-subtitle">100% offline, tiếng Việt tự nhiên, 46 lệnh, 315+ bệnh cây trồng</p>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <!-- AI Chat Demo -->
          <div class="reveal">
            <div class="bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 shadow-xl overflow-hidden">
              <div class="bg-gradient-to-r from-green-500 to-emerald-500 px-6 py-4 flex items-center gap-3">
                <Bot :size="20" class="text-white" />
                <span class="text-white font-bold">Nodi AI — Trợ lý bán hàng</span>
                <div class="ml-auto flex items-center gap-1.5">
                  <div class="w-2 h-2 rounded-full bg-green-300 animate-pulse"></div>
                  <span class="text-green-100 text-xs font-medium">Offline</span>
                </div>
              </div>
              <div class="p-6 space-y-4">
                <div v-for="msg in chatDemo" :key="msg.text"
                     class="flex" :class="msg.isUser ? 'justify-end' : 'justify-start'">
                  <div class="max-w-[80%] rounded-2xl px-5 py-3 text-sm"
                       :class="msg.isUser 
                         ? 'bg-green-500 text-white rounded-tr-sm' 
                         : 'bg-slate-100 dark:bg-slate-700 text-slate-800 dark:text-slate-200 rounded-tl-sm'">
                    {{ msg.text }}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- AI Capabilities Grid -->
          <div class="space-y-4 reveal reveal-delay-2">
            <div v-for="ai in aiCapabilities" :key="ai.title"
                 class="card-glass p-5 hover:border-green-500/30 hover:-translate-y-0.5 transition-all duration-300">
              <div class="flex items-start gap-4">
                <div class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0"
                     :class="ai.bgClass">
                  <component :is="ai.iconComponent" :size="20" :class="ai.iconClass" />
                </div>
                <div>
                  <h4 class="font-bold text-slate-900 dark:text-white mb-1">{{ ai.title }}</h4>
                  <p class="text-sm text-[var(--text-muted)]">{{ ai.desc }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- Tech Stack -->
    <section class="py-16 md:py-24 relative overflow-hidden">
      <div class="container relative z-10">
        <div class="text-center mb-16 reveal">
          <h2 class="section-title">Nền tảng công nghệ <span class="text-gradient">tiên tiến</span></h2>
          <p class="section-subtitle">Desktop native Tauri v2 — nhanh, ổn định, bảo mật</p>
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <div v-for="(tech, i) in techs" :key="tech.title"
               class="card-glass text-center p-8 hover:-translate-y-1 transition-all duration-300 reveal"
               :class="`reveal-delay-${(i % 4) + 1}`">
            <div class="w-14 h-14 mx-auto mb-5 rounded-2xl bg-gradient-to-br from-green-500/10 to-emerald-500/20 flex items-center justify-center text-primary">
              <component :is="tech.iconComponent" :size="28" />
            </div>
            <h3 class="text-lg font-bold mb-2 text-slate-900 dark:text-slate-100">{{ tech.title }}</h3>
            <p class="text-sm text-[var(--text-muted)] leading-relaxed">{{ tech.desc }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA -->
    <section class="py-12 md:py-20 mb-10">
      <div class="container reveal">
        <div class="relative overflow-hidden rounded-[2.5rem] bg-gradient-to-br from-[#0f172a] via-[#1a2e4c] to-[#0f172a] border border-slate-700 p-10 md:p-14 text-center text-white shadow-2xl">
          <div class="absolute inset-0 noise-overlay opacity-10"></div>
          <div class="absolute top-0 right-0 w-80 h-80 bg-green-500/20 rounded-full -translate-y-1/2 translate-x-1/3 blur-3xl mix-blend-screen pointer-events-none"></div>
          <div class="absolute bottom-0 left-0 w-64 h-64 bg-blue-500/20 rounded-full translate-y-1/2 -translate-x-1/3 blur-3xl mix-blend-screen pointer-events-none"></div>

          <h2 class="text-3xl md:text-5xl font-black mb-4 relative z-10 tracking-tight">Sẵn sàng nâng cấp cửa hàng?</h2>
          <p class="text-lg md:text-xl text-slate-300/90 max-w-xl mx-auto mb-10 relative z-10 font-light">
            Cài đặt trong 1 phút, dùng thử miễn phí 30 ngày. Không yêu cầu thẻ tín dụng.
          </p>
          <div class="flex flex-col sm:flex-row gap-4 justify-center relative z-10">
            <NuxtLink to="/tai-ung-dung"
                      class="btn btn-lg bg-gradient-to-r from-green-500 to-emerald-500 text-white hover:shadow-2xl hover:shadow-green-500/40 hover:-translate-y-1 hover:scale-105 active:scale-95 transition-all w-full sm:w-auto justify-center">
              <Download :size="20" />
              Tải miễn phí cho Windows
            </NuxtLink>
            <NuxtLink to="/tinh-nang"
                      class="btn btn-lg bg-white/5 text-white hover:bg-white/10 border border-white/10 backdrop-blur-sm transition-all hover:border-white/20 w-full sm:w-auto justify-center">
              Xem tính năng chi tiết
              <ArrowRight :size="18" />
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { 
  Leaf, WifiOff, Bot, FileText, QrCode, Layers, GraduationCap, 
  Fingerprint, Download, ArrowRight, ShoppingCart, Package, 
  Warehouse, DollarSign, BarChart3, Users, Cloud, Shield,
  Cpu, Database, Zap, Check, AlertTriangle, Receipt, Monitor,
  Calculator, ClipboardList, Settings, MessageSquare, Bug,
  BookOpen, Search, TrendingUp, Sprout
} from 'lucide-vue-next'

useHead({
  title: 'Giới thiệu — Nodi POS',
  meta: [
    { name: 'description', content: 'Nodi POS — phần mềm quản lý bán hàng thông minh dành riêng cho cửa hàng vật tư nông nghiệp. 100% offline, AI chẩn đoán bệnh cây, hóa đơn điện tử, dual-mode thuế.' },
    { name: 'keywords', content: 'phần mềm quản lý cửa hàng vật tư nông nghiệp, POS thuốc BVTV, phần mềm bán thuốc trừ sâu, Nodi POS, hóa đơn điện tử nông nghiệp' },
  ],
})

const stats = [
  { value: '5700+', label: 'Sản phẩm BVTV' },
  { value: '315+', label: 'Bệnh cây trồng' },
  { value: '46', label: 'Lệnh AI thông minh' },
  { value: '100%', label: 'Hoạt động Offline' },
]

const capabilities = [
  { iconComponent: ShoppingCart, title: 'Bán hàng POS', desc: 'Tạo hóa đơn trong 5 giây, quét mã, VietQR Napas', bgClass: 'bg-green-500/10', iconClass: 'text-green-500' },
  { iconComponent: Warehouse, title: 'Quản lý kho', desc: 'Lô hàng, FEFO, hạn sử dụng, catalog 5700+ SP', bgClass: 'bg-blue-500/10', iconClass: 'text-blue-500' },
  { iconComponent: Package, title: 'Nhập hàng & NCC', desc: 'Phiếu nhập, công nợ NCC, tự động cập nhật kho', bgClass: 'bg-violet-500/10', iconClass: 'text-violet-500' },
  { iconComponent: DollarSign, title: 'Công nợ KH', desc: 'Ghi nợ, thu nợ, phân khúc VIP, timeline', bgClass: 'bg-amber-500/10', iconClass: 'text-amber-500' },
  { iconComponent: FileText, title: 'Hóa đơn điện tử', desc: 'VNPT S-Invoice, Viettel, xuất XML eTax', bgClass: 'bg-rose-500/10', iconClass: 'text-rose-500' },
  { iconComponent: Calculator, title: 'Thuế & Dual-Mode', desc: 'Tách riêng có HĐ / không HĐ, VAT 0-5-10%', bgClass: 'bg-orange-500/10', iconClass: 'text-orange-500' },
  { iconComponent: BarChart3, title: 'Báo cáo', desc: 'Doanh thu, lợi nhuận, so sánh, biểu đồ', bgClass: 'bg-cyan-500/10', iconClass: 'text-cyan-500' },
  { iconComponent: Bot, title: 'Trợ lý AI', desc: '46 lệnh, 315+ bệnh, self-learning, 100% offline', bgClass: 'bg-purple-500/10', iconClass: 'text-purple-500' },
  { iconComponent: Users, title: 'Nhân viên', desc: 'PIN 4 số, 9 quyền, RBAC, đổi ca nhanh', bgClass: 'bg-teal-500/10', iconClass: 'text-teal-500' },
  { iconComponent: ClipboardList, title: 'Đơn hàng & Trả hàng', desc: 'Lịch sử, filter VAT, in lại, return', bgClass: 'bg-pink-500/10', iconClass: 'text-pink-500' },
  { iconComponent: Cloud, title: 'Cloud Sync', desc: 'Offline-first, backup tự động, khôi phục 1 click', bgClass: 'bg-sky-500/10', iconClass: 'text-sky-500' },
  { iconComponent: Settings, title: 'Cài đặt & OTA', desc: 'Dark mode, cập nhật tự động, log hệ thống', bgClass: 'bg-slate-500/10', iconClass: 'text-slate-500' },
]

const taxFeatures = [
  {
    iconComponent: Receipt, title: 'Dual-Mode hóa đơn',
    desc: 'Mỗi đơn hàng bạn chọn: xuất hóa đơn VAT hay bán lẻ không khai thuế. Hệ thống tự tách riêng báo cáo — kế toán rõ ràng, kiểm tra thuế yên tâm.',
    bgClass: 'bg-amber-500/20', iconClass: 'text-amber-400',
  },
  {
    iconComponent: BarChart3, title: 'Bảng VAT chi tiết',
    desc: 'Phân tách doanh thu theo thuế suất 0%, 5%, 10% cho từng tháng. Xuất bảng kê thuế đầu ra sẵn sàng nộp cho cơ quan thuế.',
    bgClass: 'bg-green-500/20', iconClass: 'text-green-400',
  },
  {
    iconComponent: AlertTriangle, title: 'Cảnh báo ngưỡng doanh thu',
    desc: 'Tự động nhắc khi doanh thu gần ngưỡng 500 triệu (thuế khoán) hoặc 1 tỷ (bắt buộc HĐĐT). Không bao giờ bị phạt vì chậm chuyển đổi.',
    bgClass: 'bg-red-500/20', iconClass: 'text-red-400',
  },
  {
    iconComponent: FileText, title: 'Hóa đơn điện tử tích hợp',
    desc: 'Kết nối trực tiếp VNPT S-Invoice, Viettel, CK-S. Phát hành HĐĐT ngay tại màn hình bán hàng — không cần mở thêm phần mềm nào.',
    bgClass: 'bg-blue-500/20', iconClass: 'text-blue-400',
  },
  {
    iconComponent: BookOpen, title: 'Xuất XML chuẩn eTax',
    desc: 'Xuất file XML hóa đơn theo chuẩn Tổng cục Thuế, import miễn phí vào eTax. Tiết kiệm chi phí cho cửa hàng nhỏ mới phát hành HĐĐT.',
    bgClass: 'bg-violet-500/20', iconClass: 'text-violet-400',
  },
  {
    iconComponent: Calculator, title: 'Gán thuế suất từng SP',
    desc: 'Mỗi sản phẩm gán thuế suất riêng: 0% cho giống cây, 5% cho thuốc BVTV, 10% cho vật dụng. Tính thuế chính xác từng đồng.',
    bgClass: 'bg-cyan-500/20', iconClass: 'text-cyan-400',
  },
]

const chatDemo = [
  { text: 'Lấy 2 chai Beam 75WP và 1 thùng phân NPK', isUser: true },
  { text: '✅ Đã thêm vào giỏ:\n• 2 chai Beam 75WP — 180.000đ\n• 1 thùng NPK 16-16-8 — 520.000đ\nTổng: 700.000đ. Chốt đơn không anh?', isUser: false },
  { text: 'Lúa bị vàng lá, cuốn tròn', isUser: true },
  { text: '🌾 Có thể là Sâu cuốn lá nhỏ. Anh kiểm tra lá có bị cuốn dọc không? Nếu đúng, dùng Prevathon 5SC hoặc Virtako. Trong kho mình còn 5 chai Virtako.', isUser: false },
]

const aiCapabilities = [
  { iconComponent: ShoppingCart, title: 'Bán hàng bằng chat', desc: '"Lấy 3 bao phân NPK" → AI thêm đúng SP vào giỏ, đúng đơn vị, đúng giá', bgClass: 'bg-green-500/10', iconClass: 'text-green-500' },
  { iconComponent: Bug, title: 'Chẩn đoán bệnh cây', desc: '315+ bệnh trên Lúa, Sầu riêng, Cà phê, Cam quýt, Rau — hỏi triệu chứng, AI gợi thuốc trong kho', bgClass: 'bg-rose-500/10', iconClass: 'text-rose-500' },
  { iconComponent: Search, title: 'Tra cứu mọi thứ', desc: '"Doanh thu tháng 2?", "Khách nợ nhiều nhất?", "Hàng sắp hết hạn?" — AI trả lời tức thì', bgClass: 'bg-blue-500/10', iconClass: 'text-blue-500' },
  { iconComponent: GraduationCap, title: 'Kiến thức thuế', desc: '125+ chủ đề: VAT, PIT, CIT, NĐ 123, giấy phép BVTV — trả lời bằng tiếng Việt kèm căn cứ pháp lý', bgClass: 'bg-amber-500/10', iconClass: 'text-amber-500' },
  { iconComponent: Sprout, title: 'Cảnh báo mùa vụ', desc: 'Tự động nhắc sâu bệnh theo mùa trong năm, gợi ý SP cross-sell phù hợp mùa vụ', bgClass: 'bg-teal-500/10', iconClass: 'text-teal-500' },
  { iconComponent: TrendingUp, title: 'Self-Learning', desc: 'Học từ feedback 👍/👎 của chính bạn, 7-Layer Fallback — AI luôn có câu trả lời thông minh', bgClass: 'bg-violet-500/10', iconClass: 'text-violet-500' },
]

const techs = [
  { iconComponent: Cpu, title: 'Tauri v2 (Rust)', desc: 'Backend Rust native — khởi động 2s, chiếm ít RAM, bảo mật cao.' },
  { iconComponent: Database, title: 'SQLite + Cloud', desc: 'Database local tốc độ cực nhanh, đồng bộ PostgreSQL cloud khi có mạng.' },
  { iconComponent: Zap, title: 'Vue 3 + TypeScript', desc: 'Giao diện hiện đại, responsive, Composition API hiệu năng cao.' },
  { iconComponent: Shield, title: 'Bảo mật đa lớp', desc: 'JWT + HWID + License key — xác thực bảo mật, chống crack.' },
]
</script>
