<template>
  <div>
    <div class="page-header">
      <div class="container">
        <h1>Tải Nodi POS — Miễn phí</h1>
        <p>Cài đặt trên PC, Android hoặc iOS — Quản lý cửa hàng mọi lúc mọi nơi</p>
      </div>
    </div>

    <!-- Platform Tabs -->
    <section class="section">
      <div class="container">
        <div class="flex justify-center gap-3 mb-10">
          <button v-for="tab in platforms" :key="tab.id"
                  @click="activePlatform = tab.id"
                  class="flex items-center gap-2 px-6 py-3 rounded-xl font-bold text-sm transition-all duration-300"
                  :class="activePlatform === tab.id
                    ? 'bg-primary text-white shadow-lg shadow-primary/25 scale-105'
                    : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700'">
            <component :is="tab.icon" :size="20" />
            {{ tab.label }}
          </button>
        </div>

        <!-- PC Download -->
        <div v-show="activePlatform === 'pc'" class="max-w-xl mx-auto">
          <div class="card text-center p-10 reveal">
            <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-blue-500/10 flex items-center justify-center">
              <Monitor :size="40" class="text-blue-500" />
            </div>
            <span class="badge badge-green mb-4">Phiên bản mới nhất</span>
            <h2 class="text-2xl font-extrabold mb-4 text-slate-900 dark:text-white">
              Nodi POS {{ info.latest_version ? `v${info.latest_version}` : '' }}
            </h2>
            <div class="space-y-2 mb-6 text-[var(--text-muted)]">
              <p><span class="font-semibold text-[var(--text)]">Nền tảng:</span> Windows 10/11 (64-bit)</p>
              <p v-if="info.file_size">
                <span class="font-semibold text-[var(--text)]">Kích thước:</span> {{ info.file_size }}
              </p>
            </div>

            <a v-if="info.download_url"
               :href="info.download_url"
               class="btn btn-primary btn-lg w-full justify-center mb-3">
              <Download :size="20" />
              Tải xuống cho Windows
            </a>
            <a v-else
               href="https://zalo.me/0374222326" target="_blank" rel="noopener"
               class="btn btn-primary btn-lg w-full justify-center mb-3">
              <MessageCircle :size="20" />
              Liên hệ qua Zalo để nhận link tải
            </a>

            <div v-if="info.release_notes" class="mt-4 text-left bg-slate-50 dark:bg-slate-800 rounded-xl p-4">
              <p class="text-xs font-bold text-slate-500 dark:text-slate-400 mb-2">Ghi chú phiên bản:</p>
              <p class="text-sm text-slate-600 dark:text-slate-300 whitespace-pre-line">{{ info.release_notes }}</p>
            </div>
          </div>
        </div>

        <!-- Android Download -->
        <div v-show="activePlatform === 'android'" class="max-w-xl mx-auto">
          <div class="card text-center p-10 reveal">
            <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-green-500/10 flex items-center justify-center">
              <Smartphone :size="40" class="text-green-500" />
            </div>
            <span v-if="android.latest_version" class="badge badge-green mb-4">Phiên bản {{ android.latest_version }}</span>
            <span v-else class="badge badge-green mb-4">Android</span>
            <h2 class="text-2xl font-extrabold mb-4 text-slate-900 dark:text-white">
              Nodi POS Mobile
            </h2>
            <div class="space-y-2 mb-6 text-[var(--text-muted)]">
              <p><span class="font-semibold text-[var(--text)]">Nền tảng:</span> Android 8.0+</p>
              <p v-if="android.file_size">
                <span class="font-semibold text-[var(--text)]">Kích thước:</span> {{ android.file_size }}
              </p>
            </div>

            <a v-if="android.download_url"
               :href="android.download_url"
               class="btn btn-lg w-full justify-center mb-3 bg-green-500 text-white hover:bg-green-600 transition-colors">
              <Download :size="20" />
              Tải file APK
            </a>
            <div v-else class="bg-amber-50 dark:bg-amber-900/20 rounded-xl p-6 mb-3">
              <Clock :size="32" class="mx-auto mb-3 text-amber-500" />
              <p class="text-sm font-bold text-amber-700 dark:text-amber-400 mb-1">Sắp có bản tải</p>
              <p class="text-xs text-amber-600 dark:text-amber-500">
                Liên hệ qua Zalo để nhận file APK trực tiếp
              </p>
              <a href="https://zalo.me/0374222326" target="_blank" rel="noopener"
                 class="inline-flex items-center gap-2 mt-3 px-4 py-2 rounded-lg bg-amber-500 text-white text-sm font-bold hover:bg-amber-600 transition-colors">
                <MessageCircle :size="16" />
                Zalo: 037.422.2326
              </a>
            </div>

            <div v-if="android.release_notes" class="mt-4 text-left bg-slate-50 dark:bg-slate-800 rounded-xl p-4">
              <p class="text-xs font-bold text-slate-500 dark:text-slate-400 mb-2">Ghi chú phiên bản:</p>
              <p class="text-sm text-slate-600 dark:text-slate-300 whitespace-pre-line">{{ android.release_notes }}</p>
            </div>
          </div>

          <!-- Android Install Guide -->
          <div class="mt-6 card p-6 reveal">
            <h3 class="font-bold text-slate-800 dark:text-white mb-4">📱 Hướng dẫn cài đặt APK</h3>
            <ol class="space-y-3 text-sm text-[var(--text-muted)]">
              <li class="flex gap-3">
                <span class="w-6 h-6 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 flex items-center justify-center font-bold text-xs flex-shrink-0">1</span>
                <span>Tải file APK về điện thoại</span>
              </li>
              <li class="flex gap-3">
                <span class="w-6 h-6 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 flex items-center justify-center font-bold text-xs flex-shrink-0">2</span>
                <span>Vào <strong>Cài đặt → Bảo mật</strong> → Bật "Cài đặt ứng dụng từ nguồn không xác định"</span>
              </li>
              <li class="flex gap-3">
                <span class="w-6 h-6 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 flex items-center justify-center font-bold text-xs flex-shrink-0">3</span>
                <span>Mở file APK → nhấn <strong>Cài đặt</strong></span>
              </li>
              <li class="flex gap-3">
                <span class="w-6 h-6 rounded-full bg-green-100 dark:bg-green-900/30 text-green-600 flex items-center justify-center font-bold text-xs flex-shrink-0">4</span>
                <span>Quét mã QR do chủ cửa hàng tạo để liên kết thiết bị</span>
              </li>
            </ol>
          </div>
        </div>

        <!-- iOS -->
        <div v-show="activePlatform === 'ios'" class="max-w-xl mx-auto">
          <div class="card text-center p-10 reveal">
            <div class="w-20 h-20 mx-auto mb-6 rounded-2xl bg-slate-500/10 flex items-center justify-center">
              <Tablet :size="40" class="text-slate-400" />
            </div>
            <h2 class="text-2xl font-extrabold mb-4 text-slate-900 dark:text-white">
              Nodi POS cho iOS
            </h2>
            <div class="bg-slate-100 dark:bg-slate-800 rounded-xl p-8 mb-4">
              <Clock :size="48" class="mx-auto mb-4 text-slate-400" />
              <p class="text-lg font-bold text-slate-700 dark:text-slate-300 mb-2">Đang phát triển</p>
              <p class="text-sm text-slate-500 dark:text-slate-400">
                Phiên bản iOS đang được phát triển và sẽ sớm có mặt trên App Store.
              </p>
              <p class="text-sm text-slate-500 dark:text-slate-400 mt-2">
                Theo dõi trang này để nhận thông báo khi có bản phát hành.
              </p>
            </div>
            <a href="https://zalo.me/0374222326" target="_blank" rel="noopener"
               class="btn btn-lg w-full justify-center bg-slate-800 dark:bg-white text-white dark:text-slate-800 hover:bg-slate-700 dark:hover:bg-slate-200 transition-colors">
              <MessageCircle :size="20" />
              Liên hệ để được thông báo
            </a>
          </div>
        </div>
      </div>
    </section>

    <!-- System Requirements -->
    <section class="section section-alt">
      <div class="container">
        <h2 class="section-title reveal">Yêu cầu hệ thống</h2>
        <div class="grid-3 mt-10">
          <div v-for="req in currentReqs" :key="req.label"
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
          <div class="absolute left-6 top-6 bottom-6 w-px bg-slate-200 dark:bg-slate-700 hidden md:block"></div>
          <div v-for="(step, i) in currentSteps" :key="i"
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
import { Monitor, Smartphone, Tablet, Download, MessageCircle, Clock, Cpu, HardDrive, Wifi } from 'lucide-vue-next'

useHead({
  title: 'Tải ứng dụng — Nodi POS',
  meta: [{ name: 'description', content: 'Tải Nodi POS miễn phí cho Windows, Android, iOS. Cài đặt trong 2 phút, quản lý cửa hàng vật tư nông nghiệp mọi lúc mọi nơi.' }],
})

const activePlatform = ref('pc')
const info = ref({})
const android = ref({})
const ios = ref({})

const platforms = [
  { id: 'pc', label: 'PC Windows', icon: Monitor },
  { id: 'android', label: 'Android', icon: Smartphone },
  { id: 'ios', label: 'iOS', icon: Tablet },
]

onMounted(async () => {
  try {
    const res = await fetch('/api/downloads/info')
    const data = await res.json()
    info.value = data
    android.value = data.android || {}
    ios.value = data.ios || {}
  } catch (e) {
    console.error('Failed to fetch download info:', e)
  }
})

const pcReqs = [
  { iconComponent: Monitor, label: 'Hệ điều hành', value: 'Windows 10/11 (64-bit)' },
  { iconComponent: Cpu, label: 'RAM', value: '4 GB trở lên' },
  { iconComponent: HardDrive, label: 'Ổ cứng', value: '500 MB trống' },
]

const androidReqs = [
  { iconComponent: Smartphone, label: 'Hệ điều hành', value: 'Android 8.0 trở lên' },
  { iconComponent: Cpu, label: 'RAM', value: '2 GB trở lên' },
  { iconComponent: Wifi, label: 'Kết nối', value: 'Wi-Fi hoặc 4G/5G' },
]

const iosReqs = [
  { iconComponent: Tablet, label: 'Hệ điều hành', value: 'iOS 15.0 trở lên' },
  { iconComponent: Cpu, label: 'Thiết bị', value: 'iPhone 8 trở lên' },
  { iconComponent: Wifi, label: 'Kết nối', value: 'Wi-Fi hoặc 4G/5G' },
]

const currentReqs = computed(() => {
  if (activePlatform.value === 'android') return androidReqs
  if (activePlatform.value === 'ios') return iosReqs
  return pcReqs
})

const pcSteps = [
  { title: 'Tải file cài đặt', desc: 'Nhấn nút "Tải xuống cho Windows" ở trên để tải file .exe' },
  { title: 'Chạy file cài đặt', desc: 'Mở file đã tải → nhấn Next → Install. Quá trình cài đặt chỉ mất 1 phút.' },
  { title: 'Nhập license key', desc: 'Mở ứng dụng → nhập license key (nếu có). Chưa có key? Dùng thử miễn phí 30 ngày!' },
  { title: 'Bắt đầu bán hàng!', desc: 'Thêm sản phẩm, thiết lập cửa hàng, và bắt đầu bán hàng ngay lập tức.' },
]

const androidSteps = [
  { title: 'Tải file APK', desc: 'Nhấn nút "Tải file APK" hoặc nhận file qua Zalo.' },
  { title: 'Cho phép cài đặt', desc: 'Vào Cài đặt → Bảo mật → Bật "Cài đặt từ nguồn không xác định".' },
  { title: 'Cài đặt ứng dụng', desc: 'Mở file APK đã tải → nhấn Cài đặt → Đợi hoàn tất.' },
  { title: 'Liên kết cửa hàng', desc: 'Quét mã QR từ ứng dụng PC để liên kết thiết bị vào cửa hàng.' },
]

const iosSteps = [
  { title: 'Tải từ App Store', desc: 'Tìm "Nodi POS" trên App Store và nhấn Tải về (sắp có).' },
  { title: 'Mở ứng dụng', desc: 'Mở Nodi POS từ màn hình chính.' },
  { title: 'Liên kết cửa hàng', desc: 'Quét mã QR từ ứng dụng PC để liên kết thiết bị.' },
  { title: 'Sẵn sàng bán hàng!', desc: 'Bắt đầu sử dụng ngay trên iPhone hoặc iPad.' },
]

const currentSteps = computed(() => {
  if (activePlatform.value === 'android') return androidSteps
  if (activePlatform.value === 'ios') return iosSteps
  return pcSteps
})
</script>
