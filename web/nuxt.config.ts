// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2025-01-01',
    devtools: { enabled: false },

    modules: [
        '@nuxtjs/tailwindcss',
        '@nuxtjs/color-mode',
    ],

    colorMode: {
        classSuffix: '',
        preference: 'light',
        fallback: 'light',
        storageKey: 'nodi-color-mode',
    },

    app: {
        head: {
            htmlAttrs: { lang: 'vi' },
            charset: 'utf-8',
            viewport: 'width=device-width, initial-scale=1',
            title: 'Nodi POS — Phần mềm quản lý cửa hàng vật tư nông nghiệp',
            meta: [
                { name: 'description', content: 'Phần mềm bán hàng, công nợ, hóa đơn điện tử cho đại lý vật tư nông nghiệp. Miễn phí dùng thử. Tải ngay!' },
                { property: 'og:title', content: 'Nodi POS — Quản lý cửa hàng vật tư nông nghiệp' },
                { property: 'og:description', content: 'Bán hàng, công nợ, tồn kho, HĐĐT — tất cả trong 1 phần mềm' },
                { property: 'og:image', content: 'https://nodi.vn/og-image.png' },
                { property: 'og:url', content: 'https://nodi.vn' },
                { property: 'og:type', content: 'website' },
                { name: 'robots', content: 'index, follow' },
            ],
            link: [
                { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' },
                { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
                { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
                { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
                { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap' },
            ]
        },
        pageTransition: { name: 'page', mode: 'out-in' },
    },

    css: ['~/assets/css/main.css', '~/assets/css/dashboard-global.css'],

    routeRules: {
        '/': { prerender: true },
        '/tinh-nang': { prerender: true },
        '/bang-gia': { prerender: true },
        '/tai-ung-dung': { prerender: true },
        '/lien-he': { prerender: true },
        '/chinh-sach-bao-mat': { prerender: true },
        '/dieu-khoan-su-dung': { prerender: true },
        '/blog': { prerender: true },
        '/blog/**': { prerender: true },
        '/huong-dan': { prerender: true },
        '/huong-dan/**': { prerender: true },
    },
})
