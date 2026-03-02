// Global toast notification system
const toasts = ref([])
let toastId = 0

export const useToast = () => {
    function addToast(message, type = 'info', duration = 4000) {
        const id = ++toastId
        toasts.value.push({ id, message, type, visible: true })
        if (duration > 0) {
            setTimeout(() => removeToast(id), duration)
        }
        // Max 5 toasts at a time
        if (toasts.value.length > 5) {
            toasts.value.shift()
        }
        return id
    }

    function removeToast(id) {
        const idx = toasts.value.findIndex(t => t.id === id)
        if (idx !== -1) {
            toasts.value[idx].visible = false
            setTimeout(() => {
                toasts.value = toasts.value.filter(t => t.id !== id)
            }, 300)
        }
    }

    function success(message, duration) { return addToast(message, 'success', duration) }
    function error(message, duration) { return addToast(message, 'error', duration || 6000) }
    function warning(message, duration) { return addToast(message, 'warning', duration) }
    function info(message, duration) { return addToast(message, 'info', duration) }

    return { toasts, addToast, removeToast, success, error, warning, info }
}
