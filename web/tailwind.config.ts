import type { Config } from 'tailwindcss'

export default {
    darkMode: 'class',
    content: [
        './components/**/*.{vue,js,ts}',
        './layouts/**/*.vue',
        './pages/**/*.vue',
        './composables/**/*.{js,ts}',
        './plugins/**/*.{js,ts}',
        './app.vue',
    ],
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: '#16a34a',
                    50: '#f0fdf4',
                    100: '#dcfce7',
                    200: '#bbf7d0',
                    300: '#86efac',
                    400: '#4ade80',
                    500: '#22c55e',
                    600: '#16a34a',
                    700: '#15803d',
                    800: '#166534',
                    900: '#14532d',
                },
                secondary: {
                    DEFAULT: '#2563eb',
                    50: '#eff6ff',
                    100: '#dbeafe',
                    200: '#bfdbfe',
                    300: '#93c5fd',
                    400: '#60a5fa',
                    500: '#3b82f6',
                    600: '#2563eb',
                    700: '#1d4ed8',
                    800: '#1e40af',
                    900: '#1e3a5f',
                },
                accent: {
                    DEFAULT: '#f59e0b',
                    50: '#fffbeb',
                    100: '#fef3c7',
                    200: '#fde68a',
                    300: '#fcd34d',
                    400: '#fbbf24',
                    500: '#f59e0b',
                    600: '#d97706',
                    700: '#b45309',
                    800: '#92400e',
                    900: '#78350f',
                },
            },
            fontFamily: {
                sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
            },
            borderRadius: {
                '2xl': '16px',
                '3xl': '24px',
            },
            boxShadow: {
                'glow-green': '0 0 20px rgba(22, 163, 74, 0.15)',
                'glow-blue': '0 0 20px rgba(37, 99, 235, 0.15)',
                'glow-amber': '0 0 20px rgba(245, 158, 11, 0.15)',
                'card': '0 1px 3px rgba(0,0,0,0.04), 0 4px 12px rgba(0,0,0,0.04)',
                'card-hover': '0 4px 12px rgba(0,0,0,0.08), 0 12px 28px rgba(0,0,0,0.08)',
            },
            animation: {
                'gradient': 'gradient 8s ease infinite',
                'float': 'float 6s ease-in-out infinite',
                'fade-in': 'fadeIn 0.5s ease-out forwards',
                'fade-up': 'fadeUp 0.6s ease-out forwards',
                'slide-in-right': 'slideInRight 0.5s ease-out forwards',
                'pulse-soft': 'pulseSoft 2s ease-in-out infinite',
                'marquee': 'marquee 25s linear infinite',
                'blob': 'blob 7s infinite',
            },
            keyframes: {
                gradient: {
                    '0%, 100%': { 'background-position': '0% 50%' },
                    '50%': { 'background-position': '100% 50%' },
                },
                float: {
                    '0%, 100%': { transform: 'translateY(0px)' },
                    '50%': { transform: 'translateY(-12px)' },
                },
                fadeIn: {
                    from: { opacity: '0' },
                    to: { opacity: '1' },
                },
                fadeUp: {
                    from: { opacity: '0', transform: 'translateY(24px)' },
                    to: { opacity: '1', transform: 'translateY(0)' },
                },
                slideInRight: {
                    from: { opacity: '0', transform: 'translateX(24px)' },
                    to: { opacity: '1', transform: 'translateX(0)' },
                },
                pulseSoft: {
                    '0%, 100%': { transform: 'scale(1)' },
                    '50%': { transform: 'scale(1.02)' },
                },
                marquee: {
                    '0%': { transform: 'translateX(0%)' },
                    '100%': { transform: 'translateX(-100%)' },
                },
                blob: {
                    '0%': { transform: 'translate(0px, 0px) scale(1)' },
                    '33%': { transform: 'translate(30px, -50px) scale(1.1)' },
                    '66%': { transform: 'translate(-20px, 20px) scale(0.9)' },
                    '100%': { transform: 'translate(0px, 0px) scale(1)' },
                },
            },
        },
    },
    plugins: [],
} satisfies Config
