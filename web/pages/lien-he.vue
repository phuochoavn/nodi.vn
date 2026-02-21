<template>
  <div>
    <div class="page-header">
      <div class="container">
        <h1>Liên hệ chúng tôi</h1>
        <p>Sẵn sàng hỗ trợ bạn mọi lúc</p>
      </div>
    </div>

    <section class="section">
      <div class="container">
        <div class="grid-2 items-start">
          <!-- Contact Info -->
          <div class="reveal">
            <h2 class="text-2xl font-extrabold mb-6 text-slate-900 dark:text-white">Thông tin liên hệ</h2>
            <div class="space-y-5">
              <div v-for="info in contactInfo" :key="info.label"
                   class="flex items-start gap-4 p-4 rounded-xl bg-[var(--surface)] dark:bg-slate-800/50 border border-[var(--border)]">
                <div class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0"
                     :class="info.bgClass">
                  <component :is="info.iconComponent" :size="18" :class="info.iconClass" />
                </div>
                <div>
                  <h3 class="font-semibold text-slate-800 dark:text-slate-100 mb-0.5">{{ info.label }}</h3>
                  <a v-if="info.href" :href="info.href" :target="info.external ? '_blank' : undefined"
                     class="text-[var(--text-muted)] hover:text-primary transition-colors">
                    {{ info.value }}
                  </a>
                  <p v-else class="text-[var(--text-muted)]">{{ info.value }}</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Contact Form -->
          <div class="card p-8 reveal reveal-delay-2">
            <h2 class="text-2xl font-extrabold mb-6 text-slate-900 dark:text-white">Gửi tin nhắn</h2>
            <form @submit.prevent="submitForm" class="space-y-5">
              <div>
                <label for="name" class="form-label">Họ tên</label>
                <input id="name" v-model="form.name" type="text" placeholder="Nguyễn Văn A" required class="form-input" />
              </div>
              <div>
                <label for="phone" class="form-label">Số điện thoại</label>
                <input id="phone" v-model="form.phone" type="tel" placeholder="0901234567" required class="form-input" />
              </div>
              <div>
                <label for="email" class="form-label">Email</label>
                <input id="email" v-model="form.email" type="email" placeholder="email@example.com" class="form-input" />
              </div>
              <div>
                <label for="message" class="form-label">Nội dung</label>
                <textarea id="message" v-model="form.message" rows="4" placeholder="Bạn cần hỗ trợ gì?" required class="form-input resize-none"></textarea>
              </div>
              <button type="submit" class="btn btn-primary w-full justify-center">
                <Send :size="16" />
                {{ submitted ? 'Đã gửi thành công!' : 'Gửi tin nhắn' }}
              </button>
            </form>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { MessageCircle, Phone, Mail, Clock, Send } from 'lucide-vue-next'

useHead({
  title: 'Liên hệ — Nodi POS',
  meta: [{ name: 'description', content: 'Liên hệ với Nodi POS qua Zalo, điện thoại hoặc email. Hỗ trợ kỹ thuật từ 8:00 đến 17:00.' }],
})

const form = reactive({ name: '', phone: '', email: '', message: '' })
const submitted = ref(false)

const submitForm = () => {
  console.log('Contact form submitted:', form)
  submitted.value = true
  setTimeout(() => { submitted.value = false }, 3000)
}

const contactInfo = [
  {
    iconComponent: MessageCircle, label: 'Zalo (chính)', value: '0374.222.326',
    href: 'https://zalo.me/0374222326', external: true,
    bgClass: 'bg-blue-50 dark:bg-blue-900/30', iconClass: 'text-blue-500',
  },
  {
    iconComponent: Phone, label: 'Số điện thoại', value: '0374.222.326',
    href: 'tel:0374222326',
    bgClass: 'bg-primary/10 dark:bg-primary/20', iconClass: 'text-primary',
  },
  {
    iconComponent: Mail, label: 'Email', value: 'hoavn12345@gmail.com',
    href: 'mailto:hoavn12345@gmail.com',
    bgClass: 'bg-accent/10 dark:bg-accent/20', iconClass: 'text-accent',
  },
  {
    iconComponent: Clock, label: 'Giờ hỗ trợ', value: '8:00 – 17:00 (Thứ 2 – Thứ 7)',
    bgClass: 'bg-slate-100 dark:bg-slate-800', iconClass: 'text-slate-500',
  },
]
</script>
