// https://nuxt.com/docs/api/configuration/nuxt-config
import wasm from 'vite-plugin-wasm';
export default defineNuxtConfig({
  modules: ['nuxt-monaco-editor'],
  css: ['~/assets/reset.css'],
  nitro: {
    experimental: {
      wasm: true,
    },
  },
  vite: {
    plugins: [wasm()],
    build: {
      target: 'esnext',
    },
  },
});
