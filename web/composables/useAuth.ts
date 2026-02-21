export const useAuth = () => {
    const token = useCookie('nodi_token', { maxAge: 86400 })
    const user = useState('user', () => null)

    const isAuthenticated = computed(() => !!token.value)

    const login = async (phone, password) => {
        const res = await $fetch('/api/login-with-license', {
            method: 'POST',
            body: { username: phone, password, license_key: '', hwid: '' },
        })
        if (res.success) {
            token.value = res.token
            user.value = res.user
        }
        return res
    }

    const logout = () => {
        token.value = null
        user.value = null
        navigateTo('/login')
    }

    const fetchApi = (url, opts = {}) => {
        return $fetch(url, {
            ...opts,
            headers: {
                ...opts.headers,
                Authorization: `Bearer ${token.value}`,
            },
        })
    }

    return { token, user, isAuthenticated, login, logout, fetchApi }
}
