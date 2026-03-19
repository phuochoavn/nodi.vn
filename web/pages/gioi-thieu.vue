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
                từ quản lý hàng hóa theo lô và hạn sử dụng, tách riêng hóa đơn thuế và bán lẻ, đến trợ lý AI có thể tư vấn bệnh cây trồng ngay tại quầy.
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
          <p class="section-subtitle">Từ bán hàng, kho, thuế đến báo cáo — tất cả nghiệp vụ đều có sẵn</p>
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
            Xem chi tiết tất cả tính năng
            <ArrowRight :size="16" />
          </NuxtLink>
        </div>
      </div>
    </section>

    <div class="section-divider"></div>

    <!-- Offline & Cloud — Highlight Section -->
    <section class="py-16 md:py-24 bg-gradient-to-br from-slate-900 via-[#0a1128] to-slate-900 relative overflow-hidden">
      <div class="absolute inset-0 bg-grid-dots opacity-10 mix-blend-overlay"></div>
      <div class="absolute top-1/4 left-0 w-[500px] h-[500px] bg-green-500/10 rounded-full blur-[100px] -translate-x-1/2 pointer-events-none"></div>
      <div class="absolute bottom-0 right-0 w-[400px] h-[400px] bg-blue-500/10 rounded-full blur-[100px] translate-x-1/3 pointer-events-none"></div>

      <div class="container relative z-10">
        <div class="text-center mb-16 reveal">
          <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-blue-500/10 border border-blue-500/20 text-blue-400 text-sm font-bold mb-6">
            <WifiOff :size="16" />
            Offline-first
          </div>
          <h2 class="text-3xl md:text-5xl font-black text-white tracking-tight mb-6">
            Mạng chập chờn? <br class="hidden md:block" />
            <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-cyan-300">Không vấn đề gì</span>
          </h2>
          <p class="text-lg md:text-xl text-slate-300 max-w-3xl mx-auto font-light leading-relaxed">
            Nodi POS chạy hoàn toàn trên máy tính của bạn. Không cần internet để bán hàng, 
            không phụ thuộc server. Dữ liệu tự đồng sao lưu lên đám mây khi có mạng.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-12">
          <div v-for="(item, i) in offlineFeatures" :key="item.title"
               class="bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl p-8 hover:bg-white/10 hover:border-green-500/30 transition-all duration-300 reveal"
               :class="`reveal-delay-${(i % 3) + 1}`">
            <div class="w-12 h-12 mb-5 rounded-xl flex items-center justify-center"
                 :class="item.bgClass">
              <component :is="item.iconComponent" :size="24" :class="item.iconClass" />
            </div>
            <h3 class="text-xl font-bold text-white mb-3">{{ item.title }}</h3>
            <p class="text-slate-300 text-sm leading-relaxed">{{ item.desc }}</p>
          </div>
        </div>

        <!-- Highlighted stat -->
        <div class="reveal">
          <div class="bg-gradient-to-r from-blue-500/10 to-green-500/10 border border-blue-500/20 rounded-2xl p-8 md:p-10 flex flex-col md:flex-row gap-8 items-center">
            <div class="w-20 h-20 md:w-24 md:h-24 rounded-2xl bg-gradient-to-br from-blue-500 to-cyan-500 flex items-center justify-center flex-shrink-0 shadow-xl shadow-blue-500/20">
              <Cloud :size="40" class="text-white" />
            </div>
            <div class="flex-1 text-center md:text-left">
              <h3 class="text-2xl font-black text-white mb-3">Máy hư không mất dữ liệu</h3>
              <p class="text-slate-300 leading-relaxed">
                Dữ liệu được tự động sao lưu lên đám mây. Nếu máy tính gặp sự cố, bạn chỉ cần cài lại app và khôi phục — 
                toàn bộ khách hàng, hàng hóa, đơn hàng <strong class="text-white">trở lại như cũ</strong> chỉ với 1 click.
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
          <h2 class="section-title">Trợ lý AI <span class="text-gradient">hiểu ngành nông nghiệp</span></h2>
          <p class="section-subtitle">Nói tiếng Việt, hoạt động không cần mạng, hỗ trợ bán hàng và tư vấn cây trồng</p>
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
          <h2 class="section-title">Tại sao chọn <span class="text-gradient">Nodi POS?</span></h2>
          <p class="section-subtitle">Phần mềm chạy trực tiếp trên máy tính — nhanh, ổn định, không phụ thuộc internet</p>
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
  BookOpen, Search, TrendingUp, Sprout, RefreshCw
} from 'lucide-vue-next'

useHead({
  title: 'Giới thiệu — Nodi POS',
  meta: [
    { name: 'description', content: 'Nodi POS — phần mềm quản lý bán hàng thông minh dành riêng cho cửa hàng vật tư nông nghiệp. 100% offline, AI chẩn đoán bệnh cây, hóa đơn điện tử, dual-mode thuế.' },
    { name: 'keywords', content: 'phần mềm quản lý cửa hàng vật tư nông nghiệp, POS thuốc BVTV, phần mềm bán thuốc trừ sâu, Nodi POS, hóa đơn điện tử nông nghiệp' },
  ],
})

const stats = [
  { value: '46+', label: 'Lệnh AI thông minh' },
  { value: '< 5s', label: 'Tạo hóa đơn' },
  { value: '0đ', label: 'Phí dùng thử' },
  { value: '100%', label: 'Hoạt động Offline' },
]

const capabilities = [
  { iconComponent: ShoppingCart, title: 'Bán hàng nhanh', desc: 'Quét mã, tính tiền, thanh toán QR — dưới 5 giây', bgClass: 'bg-green-500/10', iconClass: 'text-green-500' },
  { iconComponent: Warehouse, title: 'Quản lý kho', desc: 'Theo dõi tồn kho, lô hàng, hạn sử dụng tự động', bgClass: 'bg-blue-500/10', iconClass: 'text-blue-500' },
  { iconComponent: Package, title: 'Nhập hàng', desc: 'Tạo phiếu nhập, quản lý nhà cung cấp, cập nhật kho', bgClass: 'bg-violet-500/10', iconClass: 'text-violet-500' },
  { iconComponent: DollarSign, title: 'Công nợ', desc: 'Ghi nợ, thu nợ, lịch sử chi tiết cho từng khách', bgClass: 'bg-amber-500/10', iconClass: 'text-amber-500' },
  { iconComponent: FileText, title: 'Hóa đơn & In ấn', desc: 'In hóa đơn bán hàng, phiếu nhập, phiếu công nợ', bgClass: 'bg-rose-500/10', iconClass: 'text-rose-500' },
  { iconComponent: BarChart3, title: 'Báo cáo', desc: 'Xem doanh thu, lợi nhuận, so sánh các khoảng thời gian', bgClass: 'bg-cyan-500/10', iconClass: 'text-cyan-500' },
  { iconComponent: Bot, title: 'Trợ lý AI', desc: 'Bán hàng bằng chat, tư vấn bệnh cây, tra cứu nhanh', bgClass: 'bg-purple-500/10', iconClass: 'text-purple-500' },
  { iconComponent: Users, title: 'Nhân viên', desc: 'Phân quyền rõ ràng, ai làm gì biết nấy', bgClass: 'bg-teal-500/10', iconClass: 'text-teal-500' },
  { iconComponent: ClipboardList, title: 'Đơn hàng', desc: 'Lịch sử mua bán, trả hàng, in lại hóa đơn', bgClass: 'bg-pink-500/10', iconClass: 'text-pink-500' },
  { iconComponent: Cloud, title: 'Sao lưu đám mây', desc: 'Tự động sao lưu, máy hư không mất dữ liệu', bgClass: 'bg-sky-500/10', iconClass: 'text-sky-500' },
  { iconComponent: DollarSign, title: 'Sổ thu chi', desc: 'Ghi nhận thu/chi ngoài bán hàng, kiểm soát dòng tiền', bgClass: 'bg-orange-500/10', iconClass: 'text-orange-500' },
  { iconComponent: Settings, title: 'Cập nhật tự động', desc: 'Luôn có phiên bản mới nhất, không cần cài lại', bgClass: 'bg-slate-500/10', iconClass: 'text-slate-500' },
]

const offlineFeatures = [
  {
    iconComponent: WifiOff, title: 'Không cần internet',
    desc: 'Toàn bộ dữ liệu nằm ngay trên máy tính của bạn. Mạng chậm, mất mạng — vẫn bán hàng bình thường, không gián đoạn.',
    bgClass: 'bg-blue-500/20', iconClass: 'text-blue-400',
  },
  {
    iconComponent: Cloud, title: 'Tự đồng sao lưu',
    desc: 'Khi có mạng, dữ liệu tự đồng đồng bộ lên đám mây. Lên lịch backup tự động, không cần nhớ.',
    bgClass: 'bg-green-500/20', iconClass: 'text-green-400',
  },
  {
    iconComponent: Zap, title: 'Nhanh như ứng dụng bàn',
    desc: 'Không phải web trình duyệt chậm chạp. Nodi là app cài trên máy, khởi động 2 giây, thao tác mượt mà.',
    bgClass: 'bg-violet-500/20', iconClass: 'text-violet-400',
  },
  {
    iconComponent: Shield, title: 'Dữ liệu an toàn',
    desc: 'Dữ liệu lưu trên máy của bạn, không ai truy cập được. Mã hóa bảo mật, xác thực riêng từng máy tính.',
    bgClass: 'bg-amber-500/20', iconClass: 'text-amber-400',
  },
  {
    iconComponent: RefreshCw, title: 'Khôi phục 1 click',
    desc: 'Đổi máy mới? Cài app, đăng nhập, khôi phục từ cloud — toàn bộ dữ liệu trở lại nguyên vẹn.',
    bgClass: 'bg-cyan-500/20', iconClass: 'text-cyan-400',
  },
  {
    iconComponent: Monitor, title: 'Cập nhật tự động',
    desc: 'Tính năng mới được cập nhật ngay trong app, không cần tải lại từ đầu. Luôn có phiên bản mới nhất.',
    bgClass: 'bg-rose-500/20', iconClass: 'text-rose-400',
  },
]

const chatDemo = [
  { text: 'Lấy 2 chai Beam 75WP và 1 thùng phân NPK', isUser: true },
  { text: '✅ Đã thêm vào giỏ:\n• 2 chai Beam 75WP — 180.000đ\n• 1 thùng NPK 16-16-8 — 520.000đ\nTổng: 700.000đ. Chốt đơn không anh?', isUser: false },
  { text: 'Lúa bị vàng lá, cuốn tròn', isUser: true },
  { text: '🌾 Có thể là Sâu cuốn lá nhỏ. Anh kiểm tra lá có bị cuốn dọc không? Nếu đúng, dùng Prevathon 5SC hoặc Virtako. Trong kho mình còn 5 chai Virtako.', isUser: false },
]

const aiCapabilities = [
  { iconComponent: ShoppingCart, title: 'Bán hàng bằng chat', desc: 'Nói "lấy 3 bao phân NPK" — AI tự thêm đúng sản phẩm, đúng giá vào giỏ hàng', bgClass: 'bg-green-500/10', iconClass: 'text-green-500' },
  { iconComponent: Bug, title: 'Tư vấn bệnh cây trồng', desc: 'Mô tả triệu chứng, AI gợi ý bệnh và thuốc phù hợp có sẵn trong kho', bgClass: 'bg-rose-500/10', iconClass: 'text-rose-500' },
  { iconComponent: Search, title: 'Tra cứu nhanh', desc: '"Doanh thu tháng này?", "Ai nợ nhiều nhất?", "Hàng sắp hết hạn?" — hỏi gì cũng trả lời', bgClass: 'bg-blue-500/10', iconClass: 'text-blue-500' },
  { iconComponent: GraduationCap, title: 'Kiến thức nông nghiệp', desc: 'Hỏi về cách dùng thuốc, liều lượng, thời điểm phun — AI trả lời tiếng Việt dễ hiểu', bgClass: 'bg-amber-500/10', iconClass: 'text-amber-500' },
  { iconComponent: Sprout, title: 'Nhắc nhở mùa vụ', desc: 'Tự nhắc sâu bệnh theo mùa, gợi ý sản phẩm phù hợp thời điểm', bgClass: 'bg-teal-500/10', iconClass: 'text-teal-500' },
  { iconComponent: TrendingUp, title: 'Càng dùng càng thông minh', desc: 'AI tự học từ cách bạn sử dụng, ngày qua ngày trả lời chính xác hơn', bgClass: 'bg-violet-500/10', iconClass: 'text-violet-500' },
]

const techs = [
  { iconComponent: Zap, title: 'Mở là chạy ngay', desc: 'Phần mềm chạy trực tiếp trên máy tính, khởi động chỉ 2 giây, không chờ đợi.' },
  { iconComponent: Database, title: 'Dữ liệu luôn an toàn', desc: 'Lưu trữ ngay trên máy để tốc độ nhanh nhất, tự đồng bộ lên đám mây khi có mạng.' },
  { iconComponent: Monitor, title: 'Giao diện dễ dùng', desc: 'Thiết kế hiện đại, hỗ trợ chế độ sáng/tối, dùng được ngay không cần đào tạo.' },
  { iconComponent: Shield, title: 'Bảo mật cao', desc: 'Dữ liệu được mã hóa, mỗi máy tính xác thực riêng, không ai truy cập trái phép.' },
]
</script>
