<template>
  <div>
    <h2 class="page-title">📋 Đơn hàng</h2>
    <div class="table-wrap card">
      <table class="data-table">
        <thead>
          <tr>
            <th>Mã HĐ</th><th>Khách hàng</th><th>Tổng tiền</th>
            <th>Thanh toán</th><th>Ngày</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="o in orders" :key="o.id" @click="showDetail(o.id)" class="clickable">
            <td>{{ o.invoice_number || '#' + o.id }}</td>
            <td>{{ o.customer_name }}</td>
            <td class="num">{{ fmt(o.total_amount) }}</td>
            <td><span class="badge" :class="o.payment_method">{{ o.payment_method }}</span></td>
            <td>{{ o.created_at?.slice(0,16) }}</td>
          </tr>
          <tr v-if="orders.length === 0"><td colspan="5" class="empty">Chưa có đơn hàng</td></tr>
        </tbody>
      </table>
      <div class="pagination">
        <button :disabled="page <= 1" @click="page--; load()">← Trước</button>
        <span>Trang {{ page }} / {{ Math.ceil(total / limit) || 1 }}</span>
        <button :disabled="page * limit >= total" @click="page++; load()">Sau →</button>
      </div>
    </div>

    <!-- Detail Modal -->
    <div v-if="detail" class="modal-overlay" @click.self="detail = null">
      <div class="modal card">
        <h3>{{ detail.order.invoice_number }}</h3>
        <p>Khách: {{ detail.order.customer_name }} - {{ detail.order.customer_phone }}</p>
        <table class="data-table">
          <thead><tr><th>Sản phẩm</th><th>SL</th><th>Đơn giá</th><th>Thành tiền</th></tr></thead>
          <tbody>
            <tr v-for="(it, i) in detail.order.items" :key="i">
              <td>{{ it.product_name }}</td><td class="num">{{ it.quantity }}</td>
              <td class="num">{{ fmt(it.unit_price) }}</td><td class="num">{{ fmt(it.subtotal) }}</td>
            </tr>
          </tbody>
        </table>
        <p class="total">Tổng: <strong>{{ fmt(detail.order.total_amount) }}</strong></p>
        <button class="btn btn-secondary" @click="detail = null" style="margin-top:16px;">Đóng</button>
      </div>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'dashboard', middleware: 'auth' })
useHead({ title: 'Đơn hàng — Dashboard' })

const { fetchApi } = useAuth()
const orders = ref([])
const total = ref(0)
const page = ref(1)
const limit = 20
const detail = ref(null)

async function load() {
  try {
    const r = await fetchApi(`/api/dashboard/orders?page=${page.value}&limit=${limit}`)
    orders.value = r.orders; total.value = r.total
  } catch (e) { console.error(e) }
}

async function showDetail(id) {
  try { detail.value = await fetchApi(`/api/dashboard/orders/${id}`) } catch (e) { console.error(e) }
}

onMounted(load)
function fmt(v) { return new Intl.NumberFormat('vi-VN').format(v || 0) + 'đ' }
</script>

<style scoped>
.page-title { font-size: 1.5rem; font-weight: 800; margin-bottom: 24px; }
.table-wrap { overflow-x: auto; }
.data-table { width: 100%; border-collapse: collapse; }
.data-table th { text-align: left; padding: 12px; font-weight: 600; color: #64748B; font-size: 0.85rem; border-bottom: 2px solid #E2E8F0; }
.data-table td { padding: 12px; border-bottom: 1px solid #F1F5F9; }
.num { text-align: right; font-variant-numeric: tabular-nums; }
.clickable { cursor: pointer; transition: background 0.2s; }
.clickable:hover { background: #F8FAFC; }
.empty { text-align: center; color: #94A3B8; padding: 32px !important; }
.badge { padding: 2px 10px; border-radius: 999px; font-size: 0.75rem; font-weight: 600; }
.badge.CASH { background: #DCFCE7; color: #166534; }
.badge.TRANSFER { background: #DBEAFE; color: #1E40AF; }
.badge.DEBT { background: #FEF3C7; color: #92400E; }
.pagination { display: flex; justify-content: center; align-items: center; gap: 16px; padding: 16px; }
.pagination button { padding: 8px 16px; border: 1px solid #E2E8F0; border-radius: 8px; background: white; cursor: pointer; }
.pagination button:disabled { opacity: 0.4; cursor: default; }
.total { margin-top: 16px; font-size: 1.1rem; text-align: right; }
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 200; display: flex; align-items: center; justify-content: center; }
.modal { max-width: 600px; width: 90%; max-height: 80vh; overflow-y: auto; padding: 32px; }
.card { background: white; border-radius: 12px; padding: 24px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
</style>
