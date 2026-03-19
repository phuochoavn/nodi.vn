interface LoginResponse {
    success: boolean
    token: string
    user: any
    stores: any[]
    store_id: string
    store_name: string
}

interface SwitchStoreResponse {
    success: boolean
    token: string
    store_id: string
    store_name: string
}

interface StoresResponse {
    stores: any[]
}

interface CreateStoreResponse {
    success: boolean
}

interface FetchOptions {
    method?: 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE' | 'HEAD' | 'OPTIONS'
    body?: any
    headers?: Record<string, string>
}

export const useAuth = () => {
    const token = useCookie('nodi_token', { maxAge: 86400 })
    const user = useState<any>('user', () => null)
    const stores = useState<any[]>('stores', () => [])
    const activeStoreId = useCookie('nodi_active_store', { maxAge: 86400 })
    const activeStoreName = useState('activeStoreName', () => '')

    const isAuthenticated = computed(() => !!token.value)

    const login = async (username: string, password: string) => {
        const res = await $fetch<LoginResponse>('/api/login', {
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

    const switchStore = async (storeId: string) => {
        const res = await fetchApi<SwitchStoreResponse>('/api/stores/switch', {
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
                const active = parsed.find((s: any) => s.store_id === activeStoreId.value)
                if (active) activeStoreName.value = active.store_name || ''
            } catch (e) {
                stores.value = []
            }
        }
    }

    const refreshStores = async () => {
        try {
            const res = await fetchApi<StoresResponse>('/api/stores')
            stores.value = res.stores || []
            const storesCookie = useCookie('nodi_stores', { maxAge: 86400 })
            storesCookie.value = JSON.stringify(res.stores || [])
        } catch (e) {
            console.error('Failed to refresh stores:', e)
        }
    }

    const createStore = async (storeName: string) => {
        const res = await fetchApi<CreateStoreResponse>('/api/stores/create', {
            method: 'POST',
            body: { store_name: storeName },
        })
        if (res.success) {
            await refreshStores()
        }
        return res
    }

    const fetchApi = async <T = any>(url: string, opts: FetchOptions = {}): Promise<T> => {
        const { error: showError } = useToast()
        try {
            return await ($fetch as any)(url, {
                ...opts,
                headers: {
                    ...opts.headers,
                    Authorization: `Bearer ${token.value}`,
                },
            }) as T
        } catch (e: any) {
            // Auto-redirect to login on 401 (token expired)
            if (e?.response?.status === 401 || e?.status === 401 || e?.statusCode === 401) {
                token.value = null
                user.value = null
                const storesCookie = useCookie('nodi_stores', { maxAge: 86400 })
                storesCookie.value = null
                showError('Phiên đăng nhập hết hạn. Vui lòng đăng nhập lại.')
                navigateTo('/login')
            } else {
                const msg = e?.data?.message || e?.message || 'Lỗi kết nối server'
                showError(msg)
            }
            throw e
        }
    }

    return { token, user, stores, activeStoreId, activeStoreName, isAuthenticated, login, logout, switchStore, loadStores, refreshStores, createStore, fetchApi }
}
