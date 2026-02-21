<template>
  <div class="support-layout">
    <!-- Ticket List (Left) -->
    <div class="ticket-list" :class="{ collapsed: activeTicket }">
      <div class="list-header">
        <h2>💬 Hỗ trợ Kỹ thuật</h2>
        <div class="filter-bar">
          <button v-for="f in filters" :key="f.value" class="filter-btn" :class="{ active: filter === f.value }" @click="filter = f.value">
            {{ f.label }} <span v-if="f.count" class="filter-count">{{ f.count }}</span>
          </button>
        </div>
      </div>
      <div class="ticket-items">
        <div v-for="t in filteredTickets" :key="t.id" class="ticket-item" :class="{ active: activeTicket?.id === t.id, unread: t.unread_count > 0 }" @click="selectTicket(t)">
          <div class="ticket-top">
            <span class="ticket-store">{{ t.store_name || 'Khách hàng' }}</span>
            <span class="ticket-time">{{ timeAgo(t.updated_at) }}</span>
          </div>
          <div class="ticket-subject">{{ t.subject }}</div>
          <div class="ticket-preview">{{ t.last_message?.slice(0, 60) || '...' }}</div>
          <div class="ticket-meta">
            <span class="status-dot" :class="t.status"></span>
            <span class="ticket-status">{{ statusLabel(t.status) }}</span>
            <span v-if="t.unread_count > 0" class="unread-badge">{{ t.unread_count }}</span>
          </div>
        </div>
        <div v-if="filteredTickets.length === 0" class="empty-list">
          <div class="empty-icon">📭</div>
          <p>Chưa có ticket nào</p>
        </div>
      </div>
    </div>

    <!-- Chat Thread (Right) -->
    <div class="chat-area" v-if="activeTicket">
      <!-- Back button mobile -->
      <button class="back-btn" @click="activeTicket=null">← Danh sách</button>

      <!-- Store Info Card -->
      <div class="store-info-card">
        <div class="store-info-main">
          <h3>{{ activeTicket.store_name || 'Khách hàng' }}</h3>
          <div class="store-info-details">
            <span>📞 {{ activeTicket.phone || '—' }}</span>
            <span>🔑 {{ activeTicket.license_key }}</span>
            <span v-if="ticketDetail?.ticket?.license_type">📋 {{ typeLabel(ticketDetail.ticket.license_type) }}</span>
            <span v-if="wsConnected" class="ws-status connected">🟢 Live</span>
            <span v-else class="ws-status disconnected">⚪ Polling</span>
          </div>
        </div>
        <div class="store-info-status">
          <select v-model="statusSelect" @change="updateStatus" class="status-select" :class="statusSelect">
            <option value="open">🟢 Open</option>
            <option value="in_progress">🔵 Đang xử lý</option>
            <option value="resolved">✅ Đã giải quyết</option>
            <option value="closed">⬛ Đóng</option>
          </select>
        </div>
      </div>

      <!-- Typing indicator -->
      <div v-if="typingUser" class="typing-indicator">
        <span class="typing-dots"><span></span><span></span><span></span></span>
        {{ typingUser }} đang nhập...
      </div>

      <!-- Messages -->
      <div class="messages-container" ref="messagesContainer">
        <div class="messages-list">
          <div v-for="m in messages" :key="m.id" class="msg-row" :class="m.sender_type">
            <div class="msg-bubble" :class="m.sender_type">
              <div class="msg-text">{{ m.message || m.text }}</div>
              <div class="msg-meta">
                <template v-if="m.sender_type === 'system'">🔔 Hệ thống</template>
                <template v-else-if="m.sender_type === 'admin'">🛡️ Admin</template>
                <template v-else>👤 {{ m.sender_name || 'Khách' }}</template>
                — {{ formatTime(m.created_at) }}
              </div>
            </div>
          </div>
          <div v-if="messages.length === 0" class="empty-chat">
            <p>Chưa có tin nhắn</p>
          </div>
        </div>
      </div>

      <!-- Reply Input -->
      <div class="reply-bar">
        <input v-model="replyText" @keydown.enter="sendReply" @input="onTyping" placeholder="Nhập tin nhắn..." class="reply-input" :disabled="sending">
        <button class="send-btn" @click="sendReply" :disabled="!replyText.trim() || sending">📤</button>
      </div>
    </div>

    <!-- Empty state when no ticket selected -->
    <div class="chat-area empty-state" v-else>
      <div class="empty-icon">💬</div>
      <h3>Chọn ticket để bắt đầu chat</h3>
      <p>Chọn một ticket từ danh sách bên trái để xem và trả lời</p>
    </div>
  </div>
</template>

<script setup>
definePageMeta({ layout: 'admin', middleware: 'admin' })
useHead({ title: 'Hỗ trợ Kỹ thuật — Admin' })
const { fetchApi, token: authToken } = useAuth()

const tickets = ref([])
const activeTicket = ref(null)
const ticketDetail = ref(null)
const messages = ref([])
const replyText = ref('')
const sending = ref(false)
const filter = ref('all')
const statusSelect = ref('open')
const messagesContainer = ref(null)
const lastMsgCount = ref(0)

// WebSocket state
const wsConnected = ref(false)
const typingUser = ref(null)
let ws = null
let typingTimer = null
let typingClearTimer = null
let pollInterval = null

// Sound
let notifSound = null
onMounted(() => {
  try { notifSound = new Audio('data:audio/wav;base64,UklGRl9vT19teleXBm') } catch(e) {}
  loadTickets()
  // Ticket list polling (always active, lightweight)
  pollInterval = setInterval(() => loadTickets(), 15000)
  onUnmounted(() => {
    clearInterval(pollInterval)
    closeWs()
  })
})

const filters = computed(() => {
  const open = tickets.value.filter(t => t.status === 'open').length
  const inP = tickets.value.filter(t => t.status === 'in_progress').length
  return [
    { label: 'Tất cả', value: 'all', count: tickets.value.length },
    { label: 'Mới', value: 'open', count: open },
    { label: 'Đang xử lý', value: 'in_progress', count: inP },
    { label: 'Đã xong', value: 'done', count: 0 },
  ]
})

const filteredTickets = computed(() => {
  if (filter.value === 'all') return tickets.value
  if (filter.value === 'done') return tickets.value.filter(t => t.status === 'resolved' || t.status === 'closed')
  return tickets.value.filter(t => t.status === filter.value)
})

async function loadTickets() {
  try { const r = await fetchApi('/api/admin/support/tickets'); tickets.value = r } catch(e) { console.error(e) }
}

async function selectTicket(t) {
  activeTicket.value = t
  statusSelect.value = t.status
  await loadMessages(t.id)
  await loadTicketDetail(t.id)
  connectWs(t.id)
}

async function loadTicketDetail(id) {
  try { ticketDetail.value = await fetchApi(`/api/admin/support/tickets/${id}`) } catch(e) { console.error(e) }
}

async function loadMessages(id) {
  try {
    const r = await fetchApi(`/api/admin/support/tickets/${id}`)
    const newMsgs = r.messages || []
    if (newMsgs.length > lastMsgCount.value && lastMsgCount.value > 0) {
      try { notifSound?.play() } catch(e) {}
    }
    lastMsgCount.value = newMsgs.length
    messages.value = newMsgs
    ticketDetail.value = r
    nextTick(() => scrollToBottom())
  } catch(e) { console.error(e) }
}

// ========================
// WebSocket
// ========================

function getWsUrl(ticketId) {
  const tkn = authToken.value || ''
  const proto = location.protocol === 'https:' ? 'wss:' : 'ws:'
  return `${proto}//${location.host}/api/support/ws?ticket_id=${ticketId}&token=${tkn}`
}

function connectWs(ticketId) {
  closeWs()
  try {
    ws = new WebSocket(getWsUrl(ticketId))

    ws.onopen = () => {
      wsConnected.value = true
      console.log('[WS] Connected to ticket', ticketId)
    }

    ws.onmessage = (event) => {
      try {
        const evt = JSON.parse(event.data)
        handleWsEvent(evt)
      } catch(e) { console.error('[WS] Parse error', e) }
    }

    ws.onclose = () => {
      wsConnected.value = false
      console.log('[WS] Disconnected')
      // Auto-reconnect after 3s if still viewing same ticket
      setTimeout(() => {
        if (activeTicket.value?.id === ticketId) {
          console.log('[WS] Reconnecting...')
          connectWs(ticketId)
        }
      }, 3000)
    }

    ws.onerror = (err) => {
      console.error('[WS] Error', err)
      wsConnected.value = false
    }
  } catch(e) {
    console.error('[WS] Failed to connect', e)
    wsConnected.value = false
  }
}

function closeWs() {
  if (ws) {
    ws.onclose = null // Prevent auto-reconnect
    ws.close()
    ws = null
  }
  wsConnected.value = false
  typingUser.value = null
}

function handleWsEvent(evt) {
  switch(evt.type) {
    case 'message':
      // Add to messages list
      messages.value.push({
        id: evt.id,
        sender_type: evt.sender_type,
        sender_name: evt.sender_name,
        message: evt.text,
        created_at: evt.created_at
      })
      lastMsgCount.value = messages.value.length
      nextTick(() => scrollToBottom())
      // Play sound for customer messages
      if (evt.sender_type === 'customer') {
        try { notifSound?.play() } catch(e) {}
      }
      // Refresh ticket list to update preview/unread
      loadTickets()
      break

    case 'typing':
      if (evt.sender_type !== 'admin') {
        typingUser.value = evt.sender_name || 'Khách hàng'
        clearTimeout(typingClearTimer)
        typingClearTimer = setTimeout(() => { typingUser.value = null }, 4000)
      }
      break

    case 'stop_typing':
      if (evt.sender_type !== 'admin') {
        typingUser.value = null
      }
      break

    case 'ticket_closed':
      statusSelect.value = 'closed'
      if (activeTicket.value) activeTicket.value.status = 'closed'
      loadTickets()
      break

    case 'online':
      // Could show online indicator for customer
      break

    case 'read':
      // Could update message read status visually
      break

    case 'error':
      console.error('[WS] Server error:', evt.message)
      break
  }
}

function wsSend(data) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(data))
    return true
  }
  return false
}

// ========================
// Send & Typing
// ========================

async function sendReply() {
  if (!replyText.value.trim() || sending.value) return
  const text = replyText.value.trim()
  sending.value = true

  // Try WebSocket first
  if (wsSend({ type: 'message', text })) {
    replyText.value = ''
    sending.value = false
    return
  }

  // Fallback to HTTP
  try {
    await fetchApi(`/api/admin/support/tickets/${activeTicket.value.id}/reply`, { method: 'POST', body: { message: text } })
    replyText.value = ''
    await loadMessages(activeTicket.value.id)
    loadTickets()
  } catch(e) { console.error(e) }
  finally { sending.value = false }
}

function onTyping() {
  if (!typingTimer) {
    wsSend({ type: 'typing' })
  }
  clearTimeout(typingTimer)
  typingTimer = setTimeout(() => {
    wsSend({ type: 'stop_typing' })
    typingTimer = null
  }, 2000)
}

async function updateStatus() {
  try {
    await fetchApi(`/api/admin/support/tickets/${activeTicket.value.id}/status`, { method: 'PUT', body: { status: statusSelect.value } })
    activeTicket.value.status = statusSelect.value
    loadTickets()
    // Reload messages to get system message
    if (!wsConnected.value) await loadMessages(activeTicket.value.id)
  } catch(e) { console.error(e) }
}

function scrollToBottom() {
  if (messagesContainer.value) messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
}

function statusLabel(s) { return { open: 'Mới', in_progress: 'Đang xử lý', resolved: 'Đã giải quyết', closed: 'Đóng' }[s] || s }
function typeLabel(t) { return { MONTHLY:'Tháng', YEARLY:'Năm', TRIAL:'Trial', lifetime:'Lifetime' }[t] || t }
function formatTime(dt) { if (!dt) return ''; return dt.slice(5,16).replace('T',' ') }
function timeAgo(dt) {
  if (!dt) return ''
  const ms = Date.now() - new Date(dt).getTime()
  if (ms < 60000) return 'Vừa xong'
  if (ms < 3600000) return Math.floor(ms/60000) + ' phút trước'
  if (ms < 86400000) return Math.floor(ms/3600000) + ' giờ trước'
  return Math.floor(ms/86400000) + ' ngày trước'
}
</script>

<style scoped>
.support-layout { display: flex; height: calc(100vh - 100px); gap: 0; background: #F1F5F9; border-radius: 12px; overflow: hidden; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.ticket-list { width: 360px; min-width: 360px; background: white; border-right: 1px solid #E2E8F0; display: flex; flex-direction: column; }
.list-header { padding: 20px; border-bottom: 1px solid #F1F5F9; }
.list-header h2 { font-size: 1.2rem; font-weight: 800; margin-bottom: 12px; }
.filter-bar { display: flex; gap: 4px; flex-wrap: wrap; }
.filter-btn { padding: 4px 12px; border: 1px solid #E2E8F0; border-radius: 999px; background: white; font-size: 0.8rem; cursor: pointer; font-family: inherit; transition: all 0.15s; color: #64748B; }
.filter-btn.active { background: #7C3AED; color: white; border-color: #7C3AED; }
.filter-count { background: rgba(0,0,0,0.1); padding: 0 6px; border-radius: 999px; font-size: 0.7rem; margin-left: 2px; }
.filter-btn.active .filter-count { background: rgba(255,255,255,0.3); }

.ticket-items { flex: 1; overflow-y: auto; }
.ticket-item { padding: 16px 20px; border-bottom: 1px solid #F8FAFC; cursor: pointer; transition: background 0.15s; }
.ticket-item:hover { background: #F8FAFC; }
.ticket-item.active { background: #EDE9FE; border-left: 3px solid #7C3AED; }
.ticket-item.unread { background: #FFF7ED; }
.ticket-top { display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px; }
.ticket-store { font-weight: 700; font-size: 0.9rem; color: #1E293B; }
.ticket-time { font-size: 0.75rem; color: #94A3B8; }
.ticket-subject { font-size: 0.85rem; font-weight: 600; color: #475569; margin-bottom: 4px; }
.ticket-preview { font-size: 0.8rem; color: #94A3B8; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.ticket-meta { display: flex; align-items: center; gap: 6px; margin-top: 6px; }
.status-dot { width: 8px; height: 8px; border-radius: 50%; }
.status-dot.open { background: #10B981; }
.status-dot.in_progress { background: #3B82F6; }
.status-dot.resolved { background: #8B5CF6; }
.status-dot.closed { background: #94A3B8; }
.ticket-status { font-size: 0.75rem; color: #64748B; }
.unread-badge { background: #EF4444; color: white; font-size: 0.7rem; padding: 1px 7px; border-radius: 999px; font-weight: 700; margin-left: auto; }

.empty-list { text-align: center; padding: 48px 20px; color: #94A3B8; }
.empty-icon { font-size: 3rem; margin-bottom: 12px; }

/* WS status */
.ws-status { font-size: 0.75rem; font-weight: 600; padding: 2px 8px; border-radius: 999px; }
.ws-status.connected { color: #10B981; background: #ECFDF5; }
.ws-status.disconnected { color: #94A3B8; background: #F8FAFC; }

/* Typing indicator */
.typing-indicator { padding: 6px 24px; font-size: 0.8rem; color: #7C3AED; background: #FAFAFE; border-bottom: 1px solid #F1F5F9; display: flex; align-items: center; gap: 8px; }
.typing-dots { display: inline-flex; gap: 3px; }
.typing-dots span { width: 5px; height: 5px; border-radius: 50%; background: #7C3AED; animation: typingBounce 1.4s infinite ease-in-out; }
.typing-dots span:nth-child(1) { animation-delay: 0s; }
.typing-dots span:nth-child(2) { animation-delay: 0.2s; }
.typing-dots span:nth-child(3) { animation-delay: 0.4s; }
@keyframes typingBounce { 0%, 80%, 100% { transform: scale(0); } 40% { transform: scale(1); } }

/* Chat Area */
.chat-area { flex: 1; display: flex; flex-direction: column; background: white; }
.chat-area.empty-state { align-items: center; justify-content: center; color: #94A3B8; }
.chat-area.empty-state h3 { color: #475569; margin-top: 12px; }
.back-btn { display: none; padding: 8px 16px; border: none; background: #F1F5F9; cursor: pointer; font-family: inherit; font-weight: 600; color: #475569; }

.store-info-card { display: flex; justify-content: space-between; align-items: center; padding: 16px 24px; border-bottom: 1px solid #F1F5F9; background: #FAFAFA; }
.store-info-main h3 { font-size: 1.1rem; font-weight: 800; margin-bottom: 4px; }
.store-info-details { display: flex; gap: 16px; font-size: 0.8rem; color: #64748B; flex-wrap: wrap; align-items: center; }
.status-select { padding: 6px 12px; border: 1px solid #E2E8F0; border-radius: 8px; font-family: inherit; font-size: 0.85rem; cursor: pointer; }
.status-select.open { border-color: #10B981; color: #10B981; }
.status-select.in_progress { border-color: #3B82F6; color: #3B82F6; }
.status-select.resolved { border-color: #8B5CF6; color: #8B5CF6; }
.status-select.closed { border-color: #94A3B8; color: #94A3B8; }

.messages-container { flex: 1; overflow-y: auto; padding: 24px; }
.msg-row { display: flex; margin-bottom: 16px; }
.msg-row.customer { justify-content: flex-start; }
.msg-row.admin { justify-content: flex-end; }
.msg-row.system { justify-content: center; }
.msg-bubble { max-width: 70%; padding: 12px 16px; border-radius: 16px; }
.msg-bubble.customer { background: #EFF6FF; border-bottom-left-radius: 4px; }
.msg-bubble.admin { background: #EDE9FE; border-bottom-right-radius: 4px; }
.msg-bubble.system { background: #FEF3C7; border-radius: 12px; font-size: 0.85rem; color: #92400E; text-align: center; max-width: 90%; }
.msg-text { font-size: 0.9rem; line-height: 1.5; white-space: pre-wrap; word-break: break-word; }
.msg-meta { font-size: 0.7rem; color: #94A3B8; margin-top: 6px; }
.empty-chat { text-align: center; color: #94A3B8; padding: 48px; }

.reply-bar { display: flex; gap: 8px; padding: 16px 24px; border-top: 1px solid #F1F5F9; background: #FAFAFA; }
.reply-input { flex: 1; padding: 12px 16px; border: 1px solid #E2E8F0; border-radius: 12px; font-family: inherit; font-size: 0.9rem; outline: none; transition: border-color 0.15s; }
.reply-input:focus { border-color: #7C3AED; }
.send-btn { width: 48px; height: 48px; border: none; background: #7C3AED; color: white; border-radius: 12px; font-size: 1.2rem; cursor: pointer; transition: background 0.15s; display: flex; align-items: center; justify-content: center; }
.send-btn:hover:not(:disabled) { background: #6D28D9; }
.send-btn:disabled { opacity: 0.5; cursor: not-allowed; }

@media (max-width: 768px) {
  .ticket-list { width: 100%; min-width: 100%; }
  .ticket-list.collapsed { display: none; }
  .back-btn { display: block; }
  .store-info-details { flex-direction: column; gap: 4px; }
}
</style>
