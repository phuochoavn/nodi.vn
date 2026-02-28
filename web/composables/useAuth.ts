export const useAuth = () => {
    const token = useCookie('nodi_token', { maxAge: 86400 })
    const user = useState('user', () => null)
    const stores = useState('stores', () => [])
    const activeStoreId = useCookie('nodi_active_store', { maxAge: 86400 })
    const activeStoreName = useState('activeStoreName', () => '')

    const isAuthenticated = computed(() => !!token.value)

    const login = async (username, password) => {
        const res = await $fetch('/api/login', {
            method: 'POST',
            body: { username, password, hwid: '' },
        })
        if (res.success) {
            token.value = res.token
            user.value = res.user
            stores.value = res.stores || []
            activeStoreId.value = res.store_id
            activeStoreName.value = res.store_name || ''

            // Persist stores in cookie
            const storesCookie = useCookie('nodi_stores', { maxAge: 86400 })
            storesCookie.value = JSON.stringify(res.stores || [])
        }
        return res
    }

    const logout = () => {
        token.value = null
        user.value = null
        stores.value = []
        activeStoreId.value = null
        activeStoreName.value = ''
        const storesCookie = useCookie('nodi_stores', { maxAge: 86400 })
        storesCookie.value = null
        navigateTo('/login')
    }

    const switchStore = async (storeId) => {
        const res = await fetchApi('/api/stores/switch', {
            method: 'POST',
            body: { store_id: storeId },
        })
        if (res.success) {
            token.value = res.token
            activeStoreId.value = res.store_id
            activeStoreName.value = res.store_name || ''
            // Force page reload to refresh all data
            window.location.reload()
        }
        return res
    }

    const loadStores = () => {
        // Load stores from cookie on mount
        const storesCookie = useCookie('nodi_stores', { maxAge: 86400 })
        if (storesCookie.value) {
            try {
                const parsed = typeof storesCookie.value === 'string'
                    ? JSON.parse(storesCookie.value)
                    : storesCookie.value
                stores.value = parsed || []
                // Set active store name
                const active = parsed.find(s => s.store_id === activeStoreId.value)
                if (active) activeStoreName.value = active.store_name || ''
            } catch (e) {
                stores.value = []
            }
        }
    }

    const refreshStores = async () => {
        try {
            const res = await fetchApi('/api/stores')
            stores.value = res.stores || []
            const storesCookie = useCookie('nodi_stores', { maxAge: 86400 })
            storesCookie.value = JSON.stringify(res.stores || [])
        } catch (e) {
            console.error('Failed to refresh stores:', e)
        }
    }

    const createStore = async (storeName) => {
        const res = await fetchApi('/api/stores/create', {
            method: 'POST',
            body: { store_name: storeName },
        })
        if (res.success) {
            await refreshStores()
        }
        return res
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

    return { token, user, stores, activeStoreId, activeStoreName, isAuthenticated, login, logout, switchStore, loadStores, refreshStores, createStore, fetchApi }
}
