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
.total { margin-top: 16px; font-size: 1.1rem; text-align: right; color: #1E293B; }
:root.dark .total { color: #E2E8F0; }
</style>
