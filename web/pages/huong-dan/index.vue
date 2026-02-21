<template>
  <div>
    <section class="guide-hero">
      <div class="container">
        <h1>📖 Hướng Dẫn Sử Dụng Nodi POS</h1>
        <p>Tài liệu hướng dẫn chi tiết giúp bạn sử dụng Nodi POS hiệu quả nhất.</p>
      </div>
    </section>
    <section class="guides-listing">
      <div class="container">
        <div v-for="group in groups" :key="group.title" class="guide-group">
          <h2>{{ group.icon }} {{ group.title }}</h2>
          <div class="guide-grid">
            <NuxtLink v-for="g in group.items" :key="g.slug" :to="'/huong-dan/'+g.slug" class="guide-card">
              <span class="guide-icon">{{ g.icon }}</span>
              <div><h3>{{ g.title }}</h3><p>{{ g.desc }}</p></div>
              <span class="arrow">→</span>
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'default' })
useHead({
  title: 'Hướng dẫn sử dụng — Nodi POS',
  meta: [
    { name: 'description', content: 'Hướng dẫn cài đặt, bán hàng, nhập hàng, công nợ, sao lưu và hóa đơn điện tử trên Nodi POS.' },
    { property: 'og:title', content: 'Hướng dẫn sử dụng — Nodi POS' },
  ]
})

const groups = [
  { title: 'Bắt đầu', icon: '🚀', items: [
    { slug: 'cai-dat', icon: '💻', title: 'Cài đặt Nodi POS', desc: 'Download, cài đặt và cấu hình ban đầu' },
  ]},
  { title: 'Nghiệp vụ', icon: '📋', items: [
    { slug: 'ban-hang', icon: '🛒', title: 'Hướng dẫn bán hàng', desc: 'Tìm sản phẩm, thêm giỏ, thanh toán, in hóa đơn' },
    { slug: 'nhap-hang', icon: '📥', title: 'Hướng dẫn nhập hàng', desc: 'Tạo phiếu nhập, chọn NCC, nhập số lượng' },
    { slug: 'cong-no', icon: '💰', title: 'Quản lý công nợ', desc: 'Xem nợ KH/NCC, thu tiền, lịch sử thanh toán' },
  ]},
  { title: 'Nâng cao', icon: '⚙️', items: [
    { slug: 'backup', icon: '💾', title: 'Sao lưu & khôi phục', desc: 'Backup tự động, thủ công, khôi phục dữ liệu' },
    { slug: 'hoa-don-dien-tu', icon: '🧾', title: 'Hóa đơn điện tử', desc: 'Cấu hình VNPT, xuất HĐĐT, tra cứu hóa đơn' },
  ]},
]
</script>

<style scoped>
.guide-hero { background: linear-gradient(135deg, #065F46, #10B981); color: white; padding: 80px 0 60px; text-align: center; }
.guide-hero h1 { font-size: 2.2rem; font-weight: 800; margin-bottom: 12px; }
.guide-hero p { font-size: 1.1rem; color: #A7F3D0; }
.container { max-width: 900px; margin: 0 auto; padding: 0 24px; }
.guides-listing { padding: 60px 0; background: #F8FAFC; }
.guide-group { margin-bottom: 40px; }
.guide-group h2 { font-size: 1.2rem; font-weight: 700; color: #1E293B; margin-bottom: 16px; }
.guide-grid { display: flex; flex-direction: column; gap: 12px; }
.guide-card {
  background: white; border-radius: 12px; padding: 20px 24px; display: flex; align-items: center; gap: 16px;
  box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); transition: transform 0.2s, box-shadow 0.2s; color: inherit;
}
.guide-card:hover { transform: translateX(4px); box-shadow: 0 4px 16px rgb(0 0 0 / 0.1); }
.guide-icon { font-size: 2rem; }
.guide-card h3 { font-weight: 700; font-size: 1rem; margin-bottom: 4px; }
.guide-card p { font-size: 0.85rem; color: #64748B; }
.arrow { margin-left: auto; font-size: 1.2rem; color: #10B981; }
@media (max-width: 768px) { .guide-hero { padding: 60px 0 40px; } .guide-hero h1 { font-size: 1.5rem; } }
</style>
