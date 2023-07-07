// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    '@nuxtjs/tailwindcss',
    '@nuxtjs/google-fonts',
    '@pinia/nuxt',
  ],
  tailwindcss: {
    cssPath: '~/assets/css/main.css',
  },
  googleFonts: {
      preconnect: true,
      families: {
        'Inter': {wght: [100, 200, 300, 400, 500, 600, 700, 800]},
      }
    },
})
