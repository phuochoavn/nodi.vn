import { defineEventHandler } from 'h3'

export default defineEventHandler(() => {
    const baseUrl = 'https://nodi.vn'
    const now = new Date().toISOString().split('T')[0]

    const staticPages = [
        { url: '/', priority: '1.0', changefreq: 'weekly' },
        { url: '/tinh-nang', priority: '0.9', changefreq: 'monthly' },
        { url: '/bang-gia', priority: '0.9', changefreq: 'monthly' },
        { url: '/tai-ung-dung', priority: '0.9', changefreq: 'monthly' },
        { url: '/lien-he', priority: '0.7', changefreq: 'monthly' },
        { url: '/blog', priority: '0.8', changefreq: 'weekly' },
        { url: '/huong-dan', priority: '0.8', changefreq: 'monthly' },
        { url: '/chinh-sach-bao-mat', priority: '0.3', changefreq: 'yearly' },
        { url: '/dieu-khoan-su-dung', priority: '0.3', changefreq: 'yearly' },
    ]

    const blogSlugs = [
        'phan-mem-quan-ly-cua-hang-vat-tu-nong-nghiep',
        'hoa-don-dien-tu-dai-ly-phan-bon',
        'quan-ly-cong-no-khach-hang-nong-dan',
        'quan-ly-ton-kho-thuoc-bvtv',
        'chuyen-doi-so-dai-ly-nong-nghiep',
    ]

    const guideSlugs = ['cai-dat', 'ban-hang', 'nhap-hang', 'cong-no', 'backup', 'hoa-don-dien-tu']

    const blogPages = blogSlugs.map(s => ({ url: `/blog/${s}`, priority: '0.7', changefreq: 'monthly' }))
    const guidePages = guideSlugs.map(s => ({ url: `/huong-dan/${s}`, priority: '0.7', changefreq: 'monthly' }))

    const allPages = [...staticPages, ...blogPages, ...guidePages]

    const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${allPages.map(p => `  <url>
    <loc>${baseUrl}${p.url}</loc>
    <lastmod>${now}</lastmod>
    <changefreq>${p.changefreq}</changefreq>
    <priority>${p.priority}</priority>
  </url>`).join('\n')}
</urlset>`

    return new Response(xml, {
        headers: { 'Content-Type': 'application/xml; charset=utf-8' }
    })
})
