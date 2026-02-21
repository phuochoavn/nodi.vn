<template>
  <div>
    <section class="blog-hero">
      <div class="container">
        <h1>📰 Blog Nodi POS</h1>
        <p>Chia sẻ kinh nghiệm quản lý cửa hàng vật tư nông nghiệp, mẹo kinh doanh và xu hướng chuyển đổi số.</p>
      </div>
    </section>
    <section class="blog-listing">
      <div class="container">
        <div class="blog-grid">
          <NuxtLink v-for="post in posts" :key="post.slug" :to="'/blog/' + post.slug" class="blog-card">
            <div class="blog-thumb">{{ post.icon }}</div>
            <div class="blog-body">
              <span class="blog-date">📅 {{ post.date }}</span>
              <h2>{{ post.title }}</h2>
              <p>{{ post.description }}</p>
              <span class="read-more">Đọc tiếp →</span>
            </div>
          </NuxtLink>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'default' })
useHead({
  title: 'Blog — Nodi POS | Kinh nghiệm quản lý cửa hàng nông nghiệp',
  meta: [
    { name: 'description', content: 'Blog chia sẻ kinh nghiệm quản lý cửa hàng vật tư nông nghiệp, mẹo bán hàng, công nợ, tồn kho và chuyển đổi số.' },
    { property: 'og:title', content: 'Blog — Nodi POS' },
    { property: 'og:description', content: 'Kinh nghiệm quản lý cửa hàng vật tư nông nghiệp' },
  ]
})

const posts = [
  { slug: 'phan-mem-quan-ly-cua-hang-vat-tu-nong-nghiep', title: 'Phần mềm quản lý cửa hàng vật tư nông nghiệp 2026', description: 'Tổng hợp tiêu chí chọn phần mềm quản lý phù hợp cho đại lý vật tư nông nghiệp, từ bán hàng, tồn kho đến công nợ.', date: '15/02/2026', icon: '🌾' },
  { slug: 'hoa-don-dien-tu-dai-ly-phan-bon', title: 'Hướng dẫn xuất hóa đơn điện tử cho đại lý phân bón', description: 'Quy trình xuất HĐĐT đúng quy định cho đại lý phân bón, thuốc BVTV — từ đăng ký đến phát hành.', date: '12/02/2026', icon: '🧾' },
  { slug: 'quan-ly-cong-no-khach-hang-nong-dan', title: 'Cách quản lý công nợ khách hàng nông dân hiệu quả', description: 'Phương pháp theo dõi công nợ, nhắc thanh toán, và giảm thiểu rủi ro nợ xấu trong lĩnh vực nông nghiệp.', date: '08/02/2026', icon: '💰' },
  { slug: 'quan-ly-ton-kho-thuoc-bvtv', title: 'Quản lý tồn kho thuốc BVTV — tránh hết hạn, thất thoát', description: 'Chiến lược quản lý tồn kho thuốc bảo vệ thực vật hiệu quả, giảm hàng tồn hết hạn và thất thoát.', date: '03/02/2026', icon: '📦' },
  { slug: 'chuyen-doi-so-dai-ly-nong-nghiep', title: 'Chuyển đổi số cho đại lý nông nghiệp — bắt đầu từ đâu?', description: 'Hướng dẫn từng bước chuyển đổi số cho cửa hàng vật tư nông nghiệp, từ sổ sách sang phần mềm.', date: '28/01/2026', icon: '🚀' },
]
</script>

<style scoped>
.blog-hero { background: linear-gradient(135deg, #0F172A 0%, #1E3A5F 100%); color: white; padding: 80px 0 60px; text-align: center; }
.blog-hero h1 { font-size: 2.2rem; font-weight: 800; margin-bottom: 12px; }
.blog-hero p { font-size: 1.1rem; color: #94A3B8; max-width: 600px; margin: 0 auto; }
.container { max-width: 1100px; margin: 0 auto; padding: 0 24px; }
.blog-listing { padding: 60px 0; background: #F8FAFC; }
.blog-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(340px, 1fr)); gap: 24px; }
.blog-card { background: white; border-radius: 16px; overflow: hidden; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); transition: transform 0.3s, box-shadow 0.3s; display: block; color: inherit; }
.blog-card:hover { transform: translateY(-4px); box-shadow: 0 12px 30px rgb(0 0 0 / 0.1); }
.blog-thumb { font-size: 3rem; text-align: center; padding: 32px; background: linear-gradient(135deg, #EFF6FF, #F0FDF4); }
.blog-body { padding: 20px 24px 24px; }
.blog-date { font-size: 0.8rem; color: #94A3B8; }
.blog-body h2 { font-size: 1.15rem; font-weight: 700; margin: 8px 0 10px; line-height: 1.4; color: #1E293B; }
.blog-body p { font-size: 0.9rem; color: #64748B; line-height: 1.6; margin-bottom: 12px; }
.read-more { color: #2563EB; font-weight: 600; font-size: 0.9rem; }
@media (max-width: 768px) { .blog-grid { grid-template-columns: 1fr; } .blog-hero { padding: 60px 0 40px; } .blog-hero h1 { font-size: 1.6rem; } }
</style>
