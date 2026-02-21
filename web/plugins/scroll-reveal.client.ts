export default defineNuxtPlugin((nuxtApp) => {
    let observer: IntersectionObserver | null = null

    const observeElements = () => {
        if (!observer) return
        const revealEls = document.querySelectorAll('.reveal:not(.visible)')
        revealEls.forEach(el => observer!.observe(el))
    }

    nuxtApp.hook('app:mounted', () => {
        observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('visible')
                    observer!.unobserve(entry.target)
                }
            })
        }, { threshold: 0.1, rootMargin: '0px 0px -60px 0px' })

        // Observe initially
        setTimeout(observeElements, 100)
    })

    // Also observe when navigating to new pages via vue-router
    nuxtApp.hook('page:finish', () => {
        // We add a slight delay to allow the page transition to finish and DOM to update
        setTimeout(observeElements, 300)
    })
})
