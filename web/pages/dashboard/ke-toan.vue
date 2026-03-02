<template>
  <div>
    <h2 class="page-title">📋 Kế toán & Thuế</h2>

    <!-- Tab Navigation -->
    <div class="tabs">
      <button v-for="tab in tabs" :key="tab.id" class="tab-btn"
              :class="{ active: activeTab === tab.id }"
              @click="activeTab = tab.id">
        <span class="tab-icon">{{ tab.icon }}</span>
        {{ tab.label }}
      </button>
    </div>

    <!-- Tab: Sổ kế toán -->
    <div v-if="activeTab === 'books'" class="tab-content">
      <!-- Overview Cards -->
      <div class="overview-grid" v-if="overview">
        <div class="stat-card blue">
          <div class="stat-label">Doanh thu</div>
          <div class="stat-value">{{ fmt(overview.revenue) }}</div>
        </div>
        <div class="stat-card green">
          <div class="stat-label">Lợi nhuận gộp</div>
          <div class="stat-value">{{ fmt(overview.gross_profit) }}</div>
        </div>
        <div class="stat-card orange">
          <div class="stat-label">Chi phí</div>
          <div class="stat-value">{{ fmt(overview.expenses) }}</div>
        </div>
        <div class="stat-card red">
          <div class="stat-label">Thuế phải nộp</div>
          <div class="stat-value">{{ fmt(overview.total_tax_payable) }}</div>
        </div>
      </div>

      <!-- Period Selector -->
      <div class="period-bar card">
        <div class="period-inputs">
          <label>Từ <input type="date" v-model="dateFrom" /></label>
          <label>Đến <input type="date" v-model="dateTo" /></label>
          <button class="btn-primary" @click="loadOverview">Xem</button>
        </div>
      </div>

      <!-- Books List -->
      <div class="books-grid">
        <div v-for="book in books" :key="book.id" class="book-card card"
             @click="openBook(book)">
          <div class="book-icon">📘</div>
          <div class="book-info">
            <h4>{{ book.name }}</h4>
            <div v-if="activeBook?.book_id === book.id" class="book-count">
              {{ activeBook.count }} bản ghi
            </div>
          </div>
          <div class="book-actions">
            <button class="btn-sm btn-outline" @click.stop="exportBook(book.id)" :disabled="exporting === book.id">
              {{ exporting === book.id ? '⏳' : '📥' }} Excel
            </button>
          </div>
        </div>
      </div>

      <!-- Active Book Detail -->
      <div v-if="activeBook" class="book-detail card">
        <div class="book-detail-header">
          <h3>{{ activeBook.book_name }}</h3>
          <button class="btn-sm btn-outline" @click="activeBook = null">✕ Đóng</button>
        </div>
        <div class="book-detail-period">
          Kỳ: {{ activeBook.period?.from }} — {{ activeBook.period?.to }}
          · {{ activeBook.count }} bản ghi
          <span v-if="activeBook.total"> · Tổng: <strong>{{ fmt(activeBook.total) }}</strong></span>
          <span v-if="activeBook.total_inventory_value"> · Giá trị tồn: <strong>{{ fmt(activeBook.total_inventory_value) }}</strong></span>
        </div>
        
        <!-- S1 Revenue Table -->
        <table v-if="activeBook.book_id === 's1-revenue'" class="data-table">
          <thead><tr><th>Ngày</th><th>Số HĐ</th><th>Khách hàng</th><th class="num">Tổng tiền</th><th class="num">Giảm giá</th><th class="num">Thực thu</th><th>Hình thức</th></tr></thead>
          <tbody>
            <tr v-for="(e, i) in activeBook.entries" :key="i">
              <td>{{ e.date }}</td><td>{{ e.invoice_number }}</td><td>{{ e.customer_name }}</td>
              <td class="num">{{ fmt(e.total_amount) }}</td><td class="num">{{ fmt(e.discount) }}</td>
              <td class="num">{{ fmt(e.final_amount) }}</td><td>{{ e.payment_method }}</td>
            </tr>
          </tbody>
        </table>

        <!-- S2 Inventory Table -->
        <table v-if="activeBook.book_id === 's2-inventory'" class="data-table">
          <thead><tr><th>Tên hàng</th><th>ĐVT</th><th class="num">Tồn kho</th><th class="num">Giá vốn</th><th class="num">Giá bán</th><th class="num">GT Tồn</th><th>Nhóm</th></tr></thead>
          <tbody>
            <tr v-for="(e, i) in activeBook.entries" :key="i">
              <td>{{ e.product_name }}</td><td>{{ e.unit }}</td>
              <td class="num">{{ e.stock_quantity }}</td><td class="num">{{ fmt(e.cost_price) }}</td>
              <td class="num">{{ fmt(e.sell_price) }}</td><td class="num">{{ fmt(e.inventory_value) }}</td>
              <td>{{ e.category }}</td>
            </tr>
          </tbody>
        </table>

        <!-- S3/S5 Expenses/Salary Table -->
        <table v-if="activeBook.book_id === 's3-expenses' || activeBook.book_id === 's5-salary'" class="data-table">
          <thead><tr><th>Ngày</th><th class="num">Số tiền</th><th>Loại</th><th>Ghi chú</th></tr></thead>
          <tbody>
            <tr v-for="(e, i) in activeBook.entries" :key="i">
              <td>{{ e.date }}</td><td class="num">{{ fmt(e.amount) }}</td>
              <td>{{ e.category }}</td><td>{{ e.note }}</td>
            </tr>
          </tbody>
        </table>

        <!-- S4 Tax Table -->
        <table v-if="activeBook.book_id === 's4-tax'" class="data-table">
          <thead><tr><th>Tháng</th><th class="num">Doanh thu</th><th>GTGT %</th><th class="num">Thuế GTGT</th><th>TNCN %</th><th class="num">Thuế TNCN</th><th class="num">Tổng thuế</th></tr></thead>
          <tbody>
            <tr v-for="(e, i) in activeBook.entries" :key="i">
              <td>{{ e.month }}</td><td class="num">{{ fmt(e.revenue) }}</td>
              <td>{{ e.vat_rate }}%</td><td class="num">{{ fmt(e.vat_amount) }}</td>
              <td>{{ e.pit_rate }}%</td><td class="num">{{ fmt(e.pit_amount) }}</td>
              <td class="num"><strong>{{ fmt(e.total_tax) }}</strong></td>
            </tr>
          </tbody>
        </table>

        <!-- S6/S7 Cash/Bank Table -->
        <table v-if="activeBook.book_id === 's6-cash' || activeBook.book_id === 's7-bank'" class="data-table">
          <thead><tr><th>Ngày</th><th class="num">Thu</th><th class="num">Chi</th><th>Ghi chú</th><th class="num">Số dư</th></tr></thead>
          <tbody>
            <tr v-for="(e, i) in activeBook.entries" :key="i">
              <td>{{ e.date }}</td>
              <td class="num receipt">{{ e.receipt ? fmt(e.receipt) : '' }}</td>
              <td class="num payment">{{ e.payment ? fmt(e.payment) : '' }}</td>
              <td>{{ e.note }}</td>
              <td class="num">{{ fmt(e.balance || e.amount) }}</td>
            </tr>
          </tbody>
        </table>

        <div v-if="activeBook.entries?.length === 0" class="empty">Chưa có dữ liệu trong kỳ này</div>
      </div>
    </div>

    <!-- Tab: Kê khai thuế -->
    <div v-if="activeTab === 'tax'" class="tab-content">
      <div class="card tax-card">
        <h3>📄 Tờ khai thuế — Mẫu số 01/CNKD</h3>
        <p class="regulation">Theo Thông tư 40/2021/TT-BTC</p>
        
        <div class="period-bar">
          <div class="period-inputs">
            <label>Kỳ tính thuế
              <select v-model="taxPeriod">
                <option value="2026-Q1">Quý 1/2026</option>
                <option value="2026-Q2">Quý 2/2026</option>
                <option value="2026-Q3">Quý 3/2026</option>
                <option value="2026-Q4">Quý 4/2026</option>
                <option value="2025-Q1">Quý 1/2025</option>
                <option value="2025-Q2">Quý 2/2025</option>
                <option value="2025-Q3">Quý 3/2025</option>
                <option value="2025-Q4">Quý 4/2025</option>
              </select>
            </label>
            <button class="btn-primary" @click="loadTaxDeclaration">Tính thuế</button>
            <button class="btn-outline" @click="exportTaxDeclaration" :disabled="exportingTax">
              {{ exportingTax ? '⏳ Đang xuất...' : '📥 Xuất Excel' }}
            </button>
          </div>
        </div>

        <div v-if="taxDecl" class="tax-detail">
          <!-- Taxpayer Info -->
          <div class="tax-section">
            <h4>I. Thông tin người nộp thuế</h4>
            <div class="tax-info-grid">
              <div><span class="tax-label">Tên HKD:</span> {{ taxDecl.taxpayer.name || '—' }}</div>
              <div><span class="tax-label">MST:</span> {{ taxDecl.taxpayer.tax_code || '—' }}</div>
              <div><span class="tax-label">Địa chỉ:</span> {{ taxDecl.taxpayer.address || '—' }}</div>
              <div><span class="tax-label">SĐT:</span> {{ taxDecl.taxpayer.phone || '—' }}</div>
            </div>
          </div>

          <!-- Tax Calculation -->
          <div class="tax-section">
            <h4>II. Tính thuế</h4>
            <table class="tax-table">
              <thead><tr><th>Chỉ tiêu</th><th class="num">Giá trị</th></tr></thead>
              <tbody>
                <tr><td>Ngành nghề kinh doanh</td><td>{{ taxDecl.business_type }}</td></tr>
                <tr><td>Tổng doanh thu trong kỳ</td><td class="num"><strong>{{ fmt(taxDecl.revenue) }}</strong></td></tr>
                <tr><td>Số đơn hàng</td><td class="num">{{ taxDecl.order_count }}</td></tr>
                <tr class="separator"><td colspan="2"></td></tr>
                <tr><td>Thuế suất GTGT</td><td class="num">{{ taxDecl.vat_rate }}%</td></tr>
                <tr><td>Thuế GTGT phải nộp</td><td class="num tax-amount">{{ fmt(taxDecl.vat_amount) }}</td></tr>
                <tr class="separator"><td colspan="2"></td></tr>
                <tr><td>Thuế suất TNCN</td><td class="num">{{ taxDecl.pit_rate }}%</td></tr>
                <tr><td>Thuế TNCN phải nộp</td><td class="num tax-amount">{{ fmt(taxDecl.pit_amount) }}</td></tr>
                <tr class="separator"><td colspan="2"></td></tr>
                <tr class="total-row"><td><strong>TỔNG SỐ THUẾ PHẢI NỘP</strong></td><td class="num"><strong class="total-tax">{{ fmt(taxDecl.total_tax) }}</strong></td></tr>
              </tbody>
            </table>
          </div>
        </div>
        <div v-else class="empty">Chọn kỳ tính thuế và nhấn "Tính thuế" để xem tờ khai</div>
      </div>
    </div>

    <!-- Tab: Hóa đơn điện tử -->
    <div v-if="activeTab === 'einvoice'" class="tab-content">
      <div class="card einvoice-card">
        <h3>🧾 Cấu hình Hóa đơn Điện tử</h3>
        
        <div v-if="einvoiceConfig" class="einvoice-form">
          <div class="status-badge" :class="einvoiceConfig.status">
            {{ einvoiceConfig.status === 'active' ? '✅ Đã kích hoạt' : einvoiceConfig.status === 'inactive' ? '⏸️ Chưa kích hoạt' : '⚠️ Chưa cấu hình' }}
          </div>

          <div class="form-grid">
            <div class="form-group">
              <label>Nhà cung cấp HĐĐT</label>
              <select v-model="einvoiceForm.provider">
                <option value="none">— Chưa chọn —</option>
                <option value="vnpt">VNPT Invoice</option>
                <option value="viettel">Viettel S-Invoice</option>
                <option value="misa">MISA meInvoice</option>
              </select>
            </div>
            <div class="form-group">
              <label>Mã số thuế (MST)</label>
              <input type="text" v-model="einvoiceForm.tax_code" placeholder="VD: 0123456789" />
            </div>
            <div class="form-group">
              <label>API Key</label>
              <input type="text" v-model="einvoiceForm.api_key" placeholder="API Key từ nhà cung cấp" />
            </div>
            <div class="form-group">
              <label>API Secret</label>
              <input type="password" v-model="einvoiceForm.api_secret" placeholder="API Secret" />
            </div>
            <div class="form-group">
              <label>Ký hiệu hóa đơn</label>
              <input type="text" v-model="einvoiceForm.series_symbol" placeholder="VD: 1K26TYY" />
            </div>
            <div class="form-group">
              <label>Kích hoạt</label>
              <label class="toggle">
                <input type="checkbox" v-model="einvoiceForm.is_active" />
                <span class="toggle-slider"></span>
              </label>
            </div>
          </div>

          <h4 style="margin-top: 24px;">⚙️ Cấu hình thuế</h4>
          <div class="form-grid">
            <div class="form-group">
              <label>Loại hình kinh doanh</label>
              <select v-model="einvoiceForm.business_type">
                <option value="retail">Bán lẻ</option>
                <option value="wholesale">Bán buôn</option>
                <option value="service">Dịch vụ</option>
              </select>
            </div>
            <div class="form-group">
              <label>Thuế suất GTGT (%)</label>
              <input type="number" v-model.number="einvoiceForm.vat_rate" step="0.1" min="0" max="100" />
            </div>
            <div class="form-group">
              <label>Thuế suất TNCN (%)</label>
              <input type="number" v-model.number="einvoiceForm.pit_rate" step="0.1" min="0" max="100" />
            </div>
            <div class="form-group">
              <label>Kỳ kê khai</label>
              <select v-model="einvoiceForm.tax_period">
                <option value="quarterly">Theo quý</option>
                <option value="monthly">Theo tháng</option>
              </select>
            </div>
          </div>

          <div class="form-actions">
            <button class="btn-primary" @click="saveConfig" :disabled="saving">
              {{ saving ? 'Đang lưu...' : '💾 Lưu cấu hình' }}
            </button>
            <span v-if="saveMsg" class="save-msg" :class="{ error: saveError }">{{ saveMsg }}</span>
          </div>
        </div>
        <div v-else class="loading">Đang tải cấu hình...</div>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Kế toán & Thuế — Dashboard' })

const { fetchApi } = useAuth()

// === State ===
const activeTab = ref('books')
const tabs = [
  { id: 'books', label: 'Sổ kế toán', icon: '📘' },
  { id: 'tax', label: 'Kê khai thuế', icon: '📄' },
  { id: 'einvoice', label: 'Hóa đơn điện tử', icon: '🧾' },
]

// Overview
const overview = ref(null)
const today = new Date()
const dateFrom = ref(new Date(today.getFullYear(), today.getMonth(), 1).toISOString().slice(0, 10))
const dateTo = ref(today.toISOString().slice(0, 10))
const books = ref([])
const activeBook = ref(null)
const exporting = ref(null)

// Tax
const taxPeriod = ref(`${today.getFullYear()}-Q${Math.ceil((today.getMonth() + 1) / 3)}`)
const taxDecl = ref(null)
const exportingTax = ref(false)

// E-Invoice
const einvoiceConfig = ref(null)
const einvoiceForm = ref({
  provider: 'none', tax_code: '', api_key: '', api_secret: '',
  series_symbol: '', is_active: false,
  business_type: 'retail', vat_rate: 1.0, pit_rate: 0.5, tax_period: 'quarterly',
})
const saving = ref(false)
const saveMsg = ref('')
const saveError = ref(false)

// === Lifecycle ===
onMounted(async () => {
  await loadOverview()
  loadEinvoiceConfig()
})

// === Methods ===
async function loadOverview() {
  try {
    const r = await fetchApi(`/api/dashboard/accounting/overview?from=${dateFrom.value}&to=${dateTo.value}`)
    overview.value = r
    books.value = r.books || []
  } catch (e) { console.error('loadOverview', e) }
}

async function openBook(book) {
  try {
    const r = await fetchApi(`/api/dashboard/accounting/books/${book.id}?from=${dateFrom.value}&to=${dateTo.value}`)
    activeBook.value = r
  } catch (e) { console.error('openBook', e) }
}

async function exportBook(bookId) {
  exporting.value = bookId
  try {
    const blob = await $fetch(`/api/dashboard/accounting/books/${bookId}/export?from=${dateFrom.value}&to=${dateTo.value}`, {
      headers: { Authorization: `Bearer ${useAuth().token.value}` },
      responseType: 'blob',
    })
    downloadBlob(blob, `${bookId}_${dateFrom.value}_to_${dateTo.value}.xlsx`)
  } catch (e) { console.error('exportBook', e) }
  exporting.value = null
}

async function loadTaxDeclaration() {
  try {
    const r = await fetchApi(`/api/dashboard/accounting/tax-declaration?period=${taxPeriod.value}`)
    taxDecl.value = r.declaration
  } catch (e) { console.error('loadTaxDeclaration', e) }
}

async function exportTaxDeclaration() {
  exportingTax.value = true
  try {
    const blob = await $fetch(`/api/dashboard/accounting/tax-declaration/export?period=${taxPeriod.value}`, {
      headers: { Authorization: `Bearer ${useAuth().token.value}` },
      responseType: 'blob',
    })
    downloadBlob(blob, `to_khai_thue_${taxPeriod.value}.xlsx`)
  } catch (e) { console.error('exportTaxDeclaration', e) }
  exportingTax.value = false
}

async function loadEinvoiceConfig() {
  try {
    const r = await fetchApi('/api/dashboard/einvoice/config')
    einvoiceConfig.value = r
    if (r.einvoice) {
      einvoiceForm.value.provider = r.einvoice.provider || 'none'
      einvoiceForm.value.tax_code = r.einvoice.tax_code || ''
      einvoiceForm.value.series_symbol = r.einvoice.series_symbol || ''
      einvoiceForm.value.is_active = r.einvoice.is_active || false
    }
    if (r.tax) {
      einvoiceForm.value.business_type = r.tax.business_type || 'retail'
      einvoiceForm.value.vat_rate = r.tax.vat_rate ?? 1.0
      einvoiceForm.value.pit_rate = r.tax.pit_rate ?? 0.5
      einvoiceForm.value.tax_period = r.tax.tax_period || 'quarterly'
    }
  } catch (e) { console.error('loadEinvoiceConfig', e) }
}

async function saveConfig() {
  saving.value = true; saveMsg.value = ''; saveError.value = false
  try {
    const r = await fetchApi('/api/dashboard/einvoice/config', {
      method: 'PUT',
      body: einvoiceForm.value,
    })
    if (r.success) {
      saveMsg.value = '✅ Đã lưu!'
      await loadEinvoiceConfig()
    } else {
      saveMsg.value = r.message || 'Lỗi'
      saveError.value = true
    }
  } catch (e) {
    saveMsg.value = 'Lỗi kết nối'; saveError.value = true
  }
  saving.value = false
  setTimeout(() => { saveMsg.value = '' }, 3000)
}

function downloadBlob(blob, filename) {
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url; a.download = filename; a.click()
  URL.revokeObjectURL(url)
}

function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }
</script>

<style scoped>
/* Page-specific styles only — shared styles from dashboard-global.css */
.tab-icon { font-size: 1.1rem; }

/* Overview stat cards with colored borders */
.overview-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 16px; margin-bottom: 20px; }
.stat-card { background: white; border-radius: 12px; padding: 20px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); border-left: 4px solid; }
.stat-card.blue { border-color: #2563EB; }
.stat-card.green { border-color: #16A34A; }
.stat-card.orange { border-color: #EA580C; }
.stat-card.red { border-color: #DC2626; }
.stat-label { font-size: 0.85rem; color: #64748B; margin-bottom: 4px; }
.stat-value { font-size: 1.3rem; font-weight: 800; color: #1E293B; }

/* Period inputs */
.period-bar { margin-bottom: 20px; }
.period-inputs { display: flex; align-items: end; gap: 12px; flex-wrap: wrap; }
.period-inputs label { font-size: 0.85rem; color: #64748B; font-weight: 500; }
.period-inputs input, .period-inputs select { display: block; margin-top: 4px; padding: 8px 12px; border: 1px solid #E2E8F0; border-radius: 8px; font-size: 0.9rem; background: white; }
.period-inputs input:focus, .period-inputs select:focus { outline: none; border-color: #2563EB; }

/* Books grid */
.books-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 12px; margin-bottom: 20px; }
.book-card { display: flex; align-items: center; gap: 12px; cursor: pointer; transition: all 0.2s; border: 2px solid transparent; }
.book-card:hover { border-color: #2563EB; transform: translateY(-1px); }
.book-icon { font-size: 1.8rem; flex-shrink: 0; }
.book-info { flex: 1; }
.book-info h4 { font-size: 0.9rem; font-weight: 600; color: #1E293B; margin: 0; }
.book-count { font-size: 0.8rem; color: #64748B; margin-top: 2px; }
.book-actions { flex-shrink: 0; }

/* Book detail */
.book-detail { margin-top: 20px; }
.book-detail-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.book-detail-header h3 { font-size: 1.1rem; font-weight: 700; margin: 0; }
.book-detail-period { font-size: 0.85rem; color: #64748B; margin-bottom: 16px; }

/* .num alignment for tables */
.num { text-align: right; font-variant-numeric: tabular-nums; }
.receipt { color: #16A34A; }
.payment { color: #DC2626; }

/* Tax styles */
.tax-card h3 { font-size: 1.2rem; font-weight: 700; margin-bottom: 4px; }
.regulation { font-size: 0.85rem; color: #64748B; margin-bottom: 20px; }
.tax-section { margin-top: 24px; }
.tax-section h4 { font-size: 1rem; font-weight: 700; color: #2563EB; margin-bottom: 12px; padding-bottom: 8px; border-bottom: 2px solid #EFF6FF; }
.tax-info-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 8px 24px; }
.tax-label { font-weight: 600; color: #475569; }
.tax-table { width: 100%; border-collapse: collapse; }
.tax-table th { padding: 10px 16px; background: #2563EB; color: white; text-align: left; font-size: 0.9rem; }
.tax-table td { padding: 10px 16px; border-bottom: 1px solid #E2E8F0; font-size: 0.9rem; }
.tax-table .tax-amount { color: #DC2626; font-weight: 600; }
.tax-table .total-row { background: #FEF2F2; }
.tax-table .total-tax { font-size: 1.1rem; color: #DC2626; }
.tax-table .separator td { padding: 4px; border: none; }

/* E-Invoice badges */
.status-badge { display: inline-block; padding: 6px 14px; border-radius: 20px; font-size: 0.85rem; font-weight: 600; margin-bottom: 20px; }
.status-badge.active { background: #DCFCE7; color: #16A34A; }
.status-badge.inactive { background: #FEF3C7; color: #D97706; }
.status-badge.not_configured { background: #FEE2E2; color: #DC2626; }

.form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }

/* Toggle switch */
.toggle { position: relative; display: inline-block; width: 44px; height: 24px; cursor: pointer; }
.toggle input { opacity: 0; width: 0; height: 0; }
.toggle-slider { position: absolute; cursor: pointer; inset: 0; background: #CBD5E1; border-radius: 24px; transition: 0.3s; }
.toggle-slider::before { content: ''; position: absolute; height: 18px; width: 18px; left: 3px; bottom: 3px; background: white; border-radius: 50%; transition: 0.3s; }
.toggle input:checked + .toggle-slider { background: #2563EB; }
.toggle input:checked + .toggle-slider::before { transform: translateX(20px); }

.form-actions { margin-top: 24px; display: flex; align-items: center; gap: 12px; }
.save-msg { font-size: 0.85rem; font-weight: 500; color: #16A34A; }
.save-msg.error { color: #DC2626; }
.loading { text-align: center; color: #94A3B8; padding: 32px; }

/* Dark mode — page-specific */
:root.dark .stat-card, :root.dark .book-card { background: #1E293B; }
:root.dark .stat-value, :root.dark .book-info h4 { color: #E2E8F0; }
:root.dark .period-inputs input, :root.dark .period-inputs select { background: #0F172A; color: #E2E8F0; border-color: #334155; }
:root.dark .tax-table td { border-color: #334155; color: #E2E8F0; }
:root.dark .toggle-slider { background: #475569; }

@media (max-width: 768px) {
  .overview-grid { grid-template-columns: 1fr 1fr; }
  .books-grid { grid-template-columns: 1fr; }
  .form-grid { grid-template-columns: 1fr; }
  .tax-info-grid { grid-template-columns: 1fr; }
}
</style>
