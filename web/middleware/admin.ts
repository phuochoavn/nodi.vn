export default defineNuxtRouteMiddleware(() => {
    const { isAuthenticated } = useAuth()
    if (!isAuthenticated.value) return navigateTo('/login')

    // Decode JWT payload to check role (cookie persists across navigations, useState does not)
    const token = useCookie('nodi_token')
    if (!token.value) return navigateTo('/login')

    try {
        const payload = JSON.parse(atob(token.value.split('.')[1]))
        if (payload.role !== 'admin') return navigateTo('/dashboard')
    } catch {
        return navigateTo('/login')
    }
})
