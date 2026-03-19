// Global toast notification system
interface Toast {
    id: number
    message: string
    type: string
    visible: boolean
}

const toasts = ref<Toast[]>([])
let toastId = 0

export const useToast = () => {
    function addToast(message: string, type = 'info', duration = 4000) {
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

    function removeToast(id: number) {
        const idx = toasts.value.findIndex(t => t.id === id)
        if (idx !== -1) {
            toasts.value[idx].visible = false
            setTimeout(() => {
                toasts.value = toasts.value.filter(t => t.id !== id)
            }, 300)
        }
    }

    function success(message: string, duration?: number) { return addToast(message, 'success', duration) }
    function error(message: string, duration?: number) { return addToast(message, 'error', duration || 6000) }
    function warning(message: string, duration?: number) { return addToast(message, 'warning', duration) }
    function info(message: string, duration?: number) { return addToast(message, 'info', duration) }

    return { toasts, addToast, removeToast, success, error, warning, info }
}
