// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: false },
  ssr: false,

  modules: ['@nuxt/content'],

  css: ['~/assets/scss/main.scss'],

  // Static generation for GitHub Pages
  nitro: {
    preset: 'static',
  },

  runtimeConfig: {
    public: {
      siteUrl: process.env.NUXT_PUBLIC_SITE_URL || '',
      repoUrl: process.env.NUXT_PUBLIC_REPO_URL || '',
    },
  },

  // Global <head> — fonts + meta
  app: {
    // baseURL: process.env.NUXT_APP_BASE_URL || '/',  ← uncomment for project pages
    head: {
      htmlAttrs: { lang: 'en' },
      charset: 'utf-8',
      viewport: 'width=device-width, initial-scale=1',
      titleTemplate: '%s | The Agent Dispatch',
      meta: [
        { name: 'theme-color', content: '#FFFFFF' },
        { name: 'color-scheme', content: 'light' },
        { name: 'format-detection', content: 'telephone=no' },
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        {
          rel: 'preload',
          as: 'style',
          href: 'https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&display=swap',
          onload: "this.onload=null;this.rel='stylesheet'",
        },
      ],
    },
  },

  // Route rules — prerender all content routes
  routeRules: {
    '/': { prerender: true },
    '/prompts': { prerender: true },
    '/prompts/**': { prerender: true },
  },

  experimental: {
    payloadExtraction: true,
  },
})
