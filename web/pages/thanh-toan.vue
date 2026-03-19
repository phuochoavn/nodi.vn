<template>
  <div>
    <div class="page-header">
      <div class="container">
        <h1>Thanh toán</h1>
        <p>Nâng cấp lên Nodi POS Pro — Miễn phí 1 tháng đầu</p>
      </div>
    </div>

    <section class="section">
      <div class="container" style="max-width: 760px; margin: 0 auto;">

        <!-- Step Indicator -->
        <div class="steps-bar">
          <div v-for="(s, i) in steps" :key="i" class="step-item" :class="{ active: step === i, done: step > i }">
            <div class="step-num">{{ step > i ? '✓' : i + 1 }}</div>
            <span>{{ s }}</span>
            <div v-if="i < steps.length - 1" class="step-line" :class="{ filled: step > i }"></div>
          </div>
        </div>

        <!-- ==================== STEP 1: Choose Plan ==================== -->
        <div v-if="step === 0" class="fade-in">
          <h2 class="step-title">Chọn gói Pro</h2>

          <!-- Free vs Pro Comparison Banner -->
          <div class="comparison-banner">
            <div class="compare-icon">💡</div>
            <div>
              <strong>Free chỉ 20 đơn/ngày</strong> — Nâng cấp Pro để <strong>bán không giới hạn</strong>, tất cả tính năng giữ nguyên.
            </div>
          </div>

          <div class="plans-grid">
            <div v-for="p in plans" :key="p.id"
                 class="plan-card" :class="{ selected: selectedPlan === p.id, popular: p.popular }"
                 @click="selectedPlan = p.id">
              <div v-if="p.popular" class="popular-badge">⭐ Phổ biến nhất</div>
              <div v-if="p.save" class="save-badge">{{ p.save }}</div>
              <h3>{{ p.name }}</h3>
              <div class="price-row">
                <span class="price">{{ p.price }}</span>
                <span class="period">{{ p.period }}</span>
              </div>
              <p class="plan-desc">{{ p.desc }}</p>

              <!-- Highlight -->
              <div class="plan-highlight" v-if="p.highlight">
                <span class="highlight-icon">🎁</span>
                {{ p.highlight }}
              </div>

              <div class="divider"></div>

              <ul class="plan-features">
                <li v-for="f in p.features" :key="f" :class="{ bold: f.bold }">
                  <span class="check-icon">✅</span>
                  <span>{{ f.text }}</span>
                </li>
              </ul>

              <div class="select-indicator">
                <div class="radio" :class="{ checked: selectedPlan === p.id }">
                  <div class="radio-inner"></div>
                </div>
                <span>{{ selectedPlan === p.id ? 'Đã chọn' : 'Chọn gói này' }}</span>
              </div>
            </div>
          </div>

          <!-- Why Upgrade Section -->
          <div class="why-upgrade">
            <h3>🚀 Tại sao nâng cấp Pro?</h3>
            <div class="benefits-grid">
              <div class="benefit-item" v-for="b in benefits" :key="b.title">
                <span class="benefit-icon">{{ b.icon }}</span>
                <div>
                  <strong>{{ b.title }}</strong>
                  <p>{{ b.desc }}</p>
                </div>
              </div>
            </div>
          </div>

          <button class="btn btn-primary w-full cta-btn" :disabled="!selectedPlan" @click="step = 1">
            Tiếp tục đến bước thanh toán →
          </button>
        </div>

        <!-- ==================== STEP 2: Customer Info ==================== -->
        <div v-if="step === 1" class="fade-in">
          <h2 class="step-title">Thông tin liên hệ</h2>

          <!-- Selected Plan Summary -->
          <div class="selected-plan-summary">
            <div class="sps-left">
              <span class="sps-label">Gói đã chọn:</span>
              <strong>{{ selectedPlan === 'YEARLY' ? 'Pro Theo năm' : 'Pro Theo tháng' }}</strong>
            </div>
            <div class="sps-right">
              <strong class="sps-price">{{ selectedPlan === 'YEARLY' ? '1.990.000đ' : '299.000đ' }}</strong>
              <span class="sps-trial">+ 1 tháng miễn phí</span>
            </div>
          </div>

          <form @submit.prevent="createOrder" class="info-form">
            <div>
              <label class="form-label">Tên cửa hàng / Họ tên</label>
              <input v-model="form.name" type="text" placeholder="VD: Cửa hàng Nông nghiệp ABC" class="form-input" />
            </div>
            <div>
              <label class="form-label">Số điện thoại <span class="required">*</span></label>
              <input v-model="form.phone" type="tel" placeholder="0901234567" required class="form-input" />
              <p class="field-hint">Dùng để liên hệ khi cần hỗ trợ</p>
            </div>
            <div>
              <label class="form-label">Email (nhận license key)</label>
              <input v-model="form.email" type="email" placeholder="email@example.com" class="form-input" />
              <p class="field-hint">License key sẽ được gửi qua email hoặc Zalo</p>
            </div>
            <div class="form-actions">
              <button type="button" class="btn btn-outline" @click="step = 0">← Quay lại</button>
              <button type="submit" class="btn btn-primary" :disabled="!form.phone || loading">
                {{ loading ? 'Đang xử lý...' : 'Tạo đơn thanh toán' }}
              </button>
            </div>
          </form>
        </div>

        <!-- ==================== STEP 3: QR Payment ==================== -->
        <div v-if="step === 2" class="fade-in">
          <h2 class="step-title">Chuyển khoản ngân hàng</h2>

          <!-- PAID Success -->
          <div v-if="orderStatus === 'PAID'" class="status-card success-card">
            <div class="status-icon">🎉</div>
            <h3>Thanh toán thành công!</h3>
            <p>License key của bạn:</p>
            <div class="license-key">{{ orderData?.license_key }}</div>
            <p class="hint">Vui lòng lưu lại key này và nhập vào ứng dụng Nodi POS</p>
            <div class="success-actions">
              <NuxtLink to="/tai-ung-dung" class="btn btn-primary">Tải ứng dụng</NuxtLink>
              <NuxtLink to="/huong-dan" class="btn btn-outline">Hướng dẫn sử dụng</NuxtLink>
            </div>
          </div>

          <!-- EXPIRED -->
          <div v-else-if="orderStatus === 'EXPIRED'" class="status-card expired-card">
            <div class="status-icon">⏰</div>
            <h3>Đơn hàng đã hết hạn</h3>
            <p>Đơn hàng vượt quá 30 phút. Vui lòng tạo đơn mới.</p>
            <button class="btn btn-primary" @click="resetOrder" style="margin-top: 1rem;">Tạo đơn mới</button>
          </div>

          <!-- PENDING — Show QR -->
          <div v-else class="qr-section">
            <!-- Order Summary -->
            <div class="order-summary">
              <div class="summary-row">
                <span>Gói</span>
                <strong>{{ orderData?.plan === 'YEARLY' ? 'Pro Năm (13 tháng)' : 'Pro Tháng (2 tháng)' }}</strong>
              </div>
              <div class="summary-row">
                <span>Số tiền</span>
                <strong class="amount">{{ orderData?.amount_formatted }}</strong>
              </div>
              <div class="summary-row">
                <span>Mã đơn</span>
                <strong class="order-code">{{ orderData?.order_code }}</strong>
              </div>
              <div class="summary-row trial-row">
                <span>🎁 Khuyến mãi</span>
                <strong class="trial-text">+ 1 tháng miễn phí</strong>
              </div>
            </div>

            <!-- QR Code -->
            <div class="qr-wrapper">
              <img :src="orderData?.qr_url" alt="QR chuyển khoản" class="qr-image" />
              <p class="qr-hint">Mở app ngân hàng → Quét mã QR</p>
            </div>

            <!-- Bank Info -->
            <div class="bank-info">
              <h4>Hoặc chuyển khoản thủ công</h4>
              <div class="bank-row">
                <span>Ngân hàng</span>
                <strong>Vietcombank (VCB)</strong>
              </div>
              <div class="bank-row">
                <span>Số tài khoản</span>
                <strong class="copyable" @click="copyText(orderData?.bank?.account_no)">
                  {{ orderData?.bank?.account_no }}
                  <span class="copy-icon">📋</span>
                </strong>
              </div>
              <div class="bank-row">
                <span>Chủ tài khoản</span>
                <strong>{{ orderData?.bank?.account_name }}</strong>
              </div>
              <div class="bank-row highlight">
                <span>Nội dung CK</span>
                <strong class="copyable" @click="copyText(orderData?.bank?.transfer_content)">
                  {{ orderData?.bank?.transfer_content }}
                  <span class="copy-icon">📋</span>
                </strong>
              </div>
            </div>

            <!-- Countdown -->
            <div class="countdown" v-if="countdown > 0">
              <Clock :size="16" />
              Đơn hàng hết hạn sau: <strong>{{ formatCountdown }}</strong>
            </div>

            <!-- Polling status -->
            <div class="checking-status">
              <div class="spinner"></div>
              Đang kiểm tra thanh toán tự động...
            </div>

            <div class="note">
              <strong>⚠️ Lưu ý quan trọng:</strong>
              <ul>
                <li>Ghi <strong>đúng nội dung chuyển khoản</strong> để hệ thống nhận diện tự động</li>
                <li>Nếu sau 10 phút chưa xác nhận, liên hệ Zalo <a href="https://zalo.me/0374222326" target="_blank">0374.222.326</a></li>
                <li>License key được gửi ngay trên trang này khi xác nhận xong</li>
              </ul>
            </div>
          </div>
        </div>

        <!-- Error -->
        <div v-if="error" class="error-msg">{{ error }}</div>

      </div>
    </section>
  </div>
</template>

<script setup>
import { Clock } from 'lucide-vue-next'

useHead({
  title: 'Thanh toán — Nodi POS',
  meta: [{ name: 'description', content: 'Thanh toán nâng cấp Nodi POS Pro. Chuyển khoản ngân hàng qua QR an toàn, nhanh chóng.' }],
})

const steps = ['Chọn gói', 'Thông tin', 'Thanh toán']
const step = ref(0)
const selectedPlan = ref('YEARLY')
const form = reactive({ name: '', phone: '', email: '' })
const loading = ref(false)
const error = ref('')
const orderData = ref(null)
const orderStatus = ref('PENDING')
const countdown = ref(0)
let pollTimer = null
let countdownTimer = null

const plans = [
  {
    id: 'MONTHLY', name: 'Theo tháng', price: '299.000đ', period: '/tháng',
    desc: 'Dùng thử PRO 1 tháng miễn phí • Hủy bất cứ lúc nào',
    highlight: 'Miễn phí 1 tháng đầu khi mua',
    features: [
      { text: 'Bán hàng không giới hạn đơn', bold: true },
      { text: 'Quản lý sản phẩm, khách hàng, NCC' },
      { text: 'Nhập hàng, tồn kho, lô hàng' },
      { text: 'Báo cáo chi tiết (doanh thu, lãi/lỗ, top SP)' },
      { text: 'Xuất Excel' },
      { text: 'AI Chatbot thông minh' },
      { text: 'Hóa đơn điện tử (VNPT)' },
      { text: 'Cloud backup tự động' },
      { text: 'Khôi phục data khi đổi máy' },
      { text: 'Phân quyền nhân viên (9 quyền)' },
      { text: 'Tối đa 10 thiết bị (PC + Mobile)' },
      { text: 'Cập nhật phiên bản mới' },
      { text: 'Hỗ trợ kỹ thuật qua Zalo' },
    ],
  },
  {
    id: 'YEARLY', name: 'Theo năm', price: '1.990.000đ', period: '/năm', popular: true,
    desc: '≈ 166k/tháng — Tiết kiệm 44% so với gói tháng',
    highlight: 'Miễn phí 1 tháng đầu khi mua',
    save: 'Tiết kiệm 1.598.000đ',
    features: [
      { text: 'Bán hàng không giới hạn đơn', bold: true },
      { text: 'Tất cả tính năng gói tháng' },
      { text: 'Tiết kiệm 44% so với gói tháng' },
      { text: 'Ưu tiên hỗ trợ kỹ thuật' },
      { text: 'Remote support (điều khiển từ xa)' },
      { text: 'Quản lý sản phẩm, khách hàng, NCC' },
      { text: 'Nhập hàng, tồn kho, lô hàng' },
      { text: 'Báo cáo chi tiết (doanh thu, lãi/lỗ, top SP)' },
      { text: 'Xuất Excel' },
      { text: 'AI Chatbot thông minh' },
      { text: 'Hóa đơn điện tử (VNPT)' },
      { text: 'Cloud backup tự động' },
      { text: 'Phân quyền nhân viên (9 quyền)' },
    ],
  },
]

const benefits = [
  { icon: '♾️', title: 'Không giới hạn đơn hàng', desc: 'Bán bao nhiêu cũng được, không sợ đạt limit 20 đơn/ngày.' },
  { icon: '☁️', title: 'Cloud backup tự động', desc: 'Dữ liệu lưu trên cloud, đổi máy khôi phục 1 click.' },
  { icon: '📊', title: 'Báo cáo chuyên sâu', desc: 'Theo dõi doanh thu, lãi/lỗ, top sản phẩm theo thời gian thực.' },
  { icon: '🤖', title: 'AI Chatbot hỗ trợ', desc: 'Trợ lý AI giúp tra cứu sản phẩm, kiểm tra tồn kho nhanh.' },
]

const formatCountdown = computed(() => {
  const m = Math.floor(countdown.value / 60)
  const s = countdown.value % 60
  return `${m}:${s.toString().padStart(2, '0')}`
})

async function createOrder() {
  loading.value = true
  error.value = ''
  try {
    const res = await $fetch('/api/payment/create-order', {
      method: 'POST',
      body: {
        plan: selectedPlan.value,
        customer_name: form.name || undefined,
        customer_phone: form.phone,
        customer_email: form.email || undefined,
      },
    })
    if (res.success) {
      orderData.value = res
      orderStatus.value = 'PENDING'
      step.value = 2
      countdown.value = res.expires_in_minutes * 60
      startPolling()
      startCountdown()
    } else {
      error.value = res.message || 'Có lỗi xảy ra'
    }
  } catch (e) {
    error.value = e.data?.message || e.message || 'Không thể tạo đơn hàng'
  }
  loading.value = false
}

function startPolling() {
  stopPolling()
  pollTimer = setInterval(async () => {
    if (!orderData.value?.order_code) return
    try {
      const res = await $fetch(`/api/payment/check/${orderData.value.order_code}`)
      if (res.status === 'PAID') {
        orderStatus.value = 'PAID'
        orderData.value.license_key = res.license_key
        stopPolling()
        stopCountdown()
      } else if (res.status === 'EXPIRED') {
        orderStatus.value = 'EXPIRED'
        stopPolling()
        stopCountdown()
      }
    } catch {}
  }, 5000)
}

function stopPolling() { if (pollTimer) { clearInterval(pollTimer); pollTimer = null } }

function startCountdown() {
  stopCountdown()
  countdownTimer = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      orderStatus.value = 'EXPIRED'
      stopCountdown()
      stopPolling()
    }
  }, 1000)
}

function stopCountdown() { if (countdownTimer) { clearInterval(countdownTimer); countdownTimer = null } }

function resetOrder() {
  step.value = 0
  orderData.value = null
  orderStatus.value = 'PENDING'
  countdown.value = 0
  error.value = ''
  stopPolling()
  stopCountdown()
}

function copyText(text) {
  if (!text) return
  navigator.clipboard.writeText(text).catch(() => {})
}

onUnmounted(() => { stopPolling(); stopCountdown() })
</script>

<style scoped>
/* Steps Bar */
.steps-bar {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 0;
  margin-bottom: 2.5rem;
}
.step-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--text-muted);
  font-size: 0.9rem;
}
.step-item.active { color: var(--primary); font-weight: 600; }
.step-item.done { color: var(--accent, #22c55e); }
.step-num {
  width: 30px; height: 30px;
  border-radius: 50%;
  display: flex; align-items: center; justify-content: center;
  font-size: 0.8rem; font-weight: 700;
  background: var(--surface);
  border: 2px solid var(--border);
  transition: all 0.3s;
}
.step-item.active .step-num { background: var(--primary); color: white; border-color: var(--primary); }
.step-item.done .step-num { background: var(--accent, #22c55e); color: white; border-color: var(--accent, #22c55e); }
.step-line {
  width: 40px; height: 2px;
  background: var(--border);
  margin: 0 0.5rem;
  transition: background 0.3s;
}
.step-line.filled { background: var(--accent, #22c55e); }

.step-title {
  font-size: 1.5rem;
  font-weight: 800;
  text-align: center;
  margin-bottom: 1.5rem;
  color: var(--text);
}

/* Comparison Banner */
.comparison-banner {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.85rem 1.25rem;
  border-radius: 0.75rem;
  background: linear-gradient(135deg, rgba(var(--primary-rgb, 34, 197, 94), 0.08), rgba(59, 130, 246, 0.06));
  border: 1px solid rgba(var(--primary-rgb, 34, 197, 94), 0.2);
  margin-bottom: 1.5rem;
  font-size: 0.88rem;
  color: var(--text);
}
.compare-icon { font-size: 1.3rem; flex-shrink: 0; }

/* Plans Grid */
.plans-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.25rem;
}
.plan-card {
  position: relative;
  padding: 1.75rem 1.5rem;
  border-radius: 1rem;
  border: 2px solid var(--border);
  background: var(--surface);
  cursor: pointer;
  transition: all 0.25s;
}
.plan-card:hover { border-color: var(--primary); transform: translateY(-3px); box-shadow: 0 8px 30px rgba(0,0,0,0.08); }
.plan-card.selected { border-color: var(--primary); box-shadow: 0 0 0 3px rgba(var(--primary-rgb, 34, 197, 94), 0.15), 0 8px 30px rgba(0,0,0,0.08); }
.plan-card.popular { border-color: var(--primary); }

.popular-badge {
  position: absolute; top: -12px; left: 50%; transform: translateX(-50%);
  background: var(--primary); color: white;
  font-size: 0.72rem; font-weight: 700;
  padding: 3px 14px; border-radius: 20px;
  white-space: nowrap;
}
.save-badge {
  display: inline-block;
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
  font-size: 0.72rem; font-weight: 700;
  padding: 2px 10px; border-radius: 20px;
  margin-bottom: 0.5rem;
}

.plan-card h3 { font-size: 1.15rem; font-weight: 700; margin-bottom: 0.25rem; color: var(--text); }
.price-row { display: flex; align-items: baseline; gap: 0.15rem; margin-bottom: 0.25rem; }
.price { font-size: 1.6rem; font-weight: 800; color: var(--primary); }
.period { font-size: 0.85rem; font-weight: 400; color: var(--text-muted); }
.plan-desc { font-size: 0.8rem; color: var(--text-muted); margin-bottom: 0.5rem; line-height: 1.4; }

.plan-highlight {
  display: flex; align-items: center; gap: 0.4rem;
  background: rgba(251, 191, 36, 0.1);
  color: #b45309;
  font-size: 0.78rem; font-weight: 600;
  padding: 0.4rem 0.75rem;
  border-radius: 0.5rem;
  margin-bottom: 0.75rem;
}
.highlight-icon { font-size: 1rem; }

.divider { height: 1px; background: var(--border); margin: 0.75rem 0; }

.plan-features { list-style: none; padding: 0; margin: 0; }
.plan-features li {
  display: flex; align-items: flex-start; gap: 0.4rem;
  padding: 0.25rem 0;
  font-size: 0.82rem;
  color: var(--text-muted);
  line-height: 1.4;
}
.plan-features li.bold { font-weight: 600; color: var(--text); }
.check-icon { font-size: 0.7rem; flex-shrink: 0; margin-top: 2px; }

.select-indicator {
  display: flex; align-items: center; gap: 0.5rem;
  margin-top: 1rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--border);
  font-size: 0.82rem;
  color: var(--text-muted);
}
.radio {
  width: 18px; height: 18px;
  border-radius: 50%;
  border: 2px solid var(--border);
  display: flex; align-items: center; justify-content: center;
  transition: all 0.2s;
}
.radio.checked { border-color: var(--primary); }
.radio-inner {
  width: 10px; height: 10px;
  border-radius: 50%;
  background: transparent;
  transition: all 0.2s;
}
.radio.checked .radio-inner { background: var(--primary); }

/* Why Upgrade */
.why-upgrade {
  margin-top: 2rem;
  padding: 1.5rem;
  border-radius: 1rem;
  background: var(--surface);
  border: 1px solid var(--border);
}
.why-upgrade h3 { font-size: 1.1rem; font-weight: 700; margin-bottom: 1rem; color: var(--text); }
.benefits-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
.benefit-item {
  display: flex; gap: 0.6rem; align-items: flex-start;
}
.benefit-icon { font-size: 1.4rem; flex-shrink: 0; margin-top: 2px; }
.benefit-item strong { display: block; font-size: 0.85rem; color: var(--text); margin-bottom: 2px; }
.benefit-item p { font-size: 0.78rem; color: var(--text-muted); margin: 0; line-height: 1.4; }

.cta-btn {
  margin-top: 1.5rem;
  padding: 0.85rem;
  font-size: 1rem;
  justify-content: center;
}

/* Step 2 */
.selected-plan-summary {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
  border-radius: 0.75rem;
  background: var(--surface);
  border: 1px solid var(--border);
  margin-bottom: 1.5rem;
}
.sps-label { font-size: 0.8rem; color: var(--text-muted); display: block; margin-bottom: 2px; }
.sps-right { text-align: right; }
.sps-price { color: var(--primary); font-size: 1.1rem; display: block; }
.sps-trial { font-size: 0.75rem; color: #b45309; font-weight: 600; }

.info-form { display: flex; flex-direction: column; gap: 1rem; }
.required { color: #ef4444; }
.field-hint { font-size: 0.75rem; color: var(--text-muted); margin-top: 0.25rem; }
.form-actions { display: flex; gap: 1rem; margin-top: 0.5rem; }
.form-actions .btn { flex: 1; justify-content: center; }
.btn-outline {
  background: transparent;
  border: 2px solid var(--border);
  color: var(--text);
  padding: 0.6rem 1.25rem;
  border-radius: 0.75rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}
.btn-outline:hover { border-color: var(--primary); color: var(--primary); }

/* Step 3: QR */
.qr-section { text-align: center; }
.order-summary {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1rem 1.5rem;
  margin-bottom: 1.5rem;
  text-align: left;
}
.summary-row {
  display: flex; justify-content: space-between; align-items: center;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--border);
  font-size: 0.9rem;
}
.summary-row:last-child { border-bottom: none; }
.summary-row span { color: var(--text-muted); }
.amount { color: var(--primary); font-size: 1.15rem; }
.order-code { font-family: 'Courier New', monospace; letter-spacing: 1px; }
.trial-row { background: rgba(251, 191, 36, 0.06); margin: 0 -1.5rem; padding: 0.6rem 1.5rem; }
.trial-text { color: #b45309; }

.qr-wrapper { margin-bottom: 1.5rem; }
.qr-image {
  width: 280px; height: auto;
  border-radius: 1rem;
  box-shadow: 0 4px 24px rgba(0,0,0,0.1);
}
.qr-hint { font-size: 0.8rem; color: var(--text-muted); margin-top: 0.5rem; }

/* Bank Info */
.bank-info {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 1rem;
  padding: 1rem 1.5rem;
  text-align: left;
  margin-bottom: 1.5rem;
}
.bank-info h4 { font-size: 0.95rem; font-weight: 700; margin-bottom: 0.75rem; color: var(--text); }
.bank-row {
  display: flex; justify-content: space-between; align-items: center;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--border);
  font-size: 0.88rem;
}
.bank-row:last-child { border-bottom: none; }
.bank-row span { color: var(--text-muted); }
.bank-row.highlight { background: rgba(var(--primary-rgb, 34, 197, 94), 0.06); margin: 0 -1.5rem; padding: 0.65rem 1.5rem; border-radius: 0.5rem; }
.copyable { cursor: pointer; display: flex; align-items: center; gap: 0.3rem; }
.copyable:hover { color: var(--primary); }
.copy-icon { font-size: 0.8rem; opacity: 0.5; }

/* Countdown */
.countdown {
  display: flex; align-items: center; justify-content: center; gap: 0.4rem;
  color: var(--text-muted); font-size: 0.85rem;
  margin-bottom: 1rem;
}

/* Checking */
.checking-status {
  display: flex; align-items: center; justify-content: center; gap: 0.5rem;
  color: var(--text-muted); font-size: 0.85rem;
  margin-bottom: 1rem;
}
.spinner {
  width: 16px; height: 16px;
  border: 2px solid var(--border);
  border-top-color: var(--primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}
@keyframes spin { to { transform: rotate(360deg); } }

.note {
  font-size: 0.82rem;
  color: var(--text-muted);
  padding: 1rem 1.25rem;
  background: rgba(251, 191, 36, 0.08);
  border-radius: 0.75rem;
  border: 1px solid rgba(251, 191, 36, 0.2);
  text-align: left;
}
.note ul { margin: 0.5rem 0 0 1.2rem; padding: 0; }
.note li { margin-bottom: 0.3rem; }
.note a { color: var(--primary); text-decoration: underline; }

/* Status Cards */
.status-card {
  text-align: center;
  padding: 2.5rem 2rem;
  border-radius: 1rem;
  border: 2px solid;
}
.success-card { border-color: #22c55e; background: rgba(34, 197, 94, 0.05); }
.expired-card { border-color: #ef4444; background: rgba(239, 68, 68, 0.05); }
.status-icon { font-size: 3.5rem; margin-bottom: 0.75rem; }
.status-card h3 { font-size: 1.4rem; margin-bottom: 0.5rem; }
.license-key {
  display: inline-block;
  font-family: 'Courier New', monospace;
  font-size: 1.3rem; font-weight: 800;
  letter-spacing: 2px;
  padding: 0.75rem 1.5rem;
  background: var(--surface);
  border: 2px dashed var(--primary);
  border-radius: 0.75rem;
  margin: 0.75rem 0;
  color: var(--primary);
}
.hint { font-size: 0.82rem; color: var(--text-muted); }
.success-actions {
  display: flex; gap: 0.75rem; justify-content: center; margin-top: 1.25rem;
}

.error-msg {
  text-align: center;
  color: #ef4444;
  background: rgba(239, 68, 68, 0.08);
  padding: 0.75rem;
  border-radius: 0.75rem;
  margin-top: 1rem;
  font-size: 0.9rem;
}

.fade-in { animation: fadeIn 0.3s ease; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }

@media (max-width: 640px) {
  .plans-grid { grid-template-columns: 1fr; }
  .benefits-grid { grid-template-columns: 1fr; }
  .steps-bar { gap: 0; }
  .step-item span { font-size: 0.75rem; }
  .step-line { width: 24px; }
  .qr-image { width: 220px; }
  .selected-plan-summary { flex-direction: column; gap: 0.5rem; text-align: center; }
  .sps-right { text-align: center; }
}
</style>
