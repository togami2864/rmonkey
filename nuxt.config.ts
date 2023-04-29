// https://nuxt.com/docs/api/configuration/nuxt-config
import { defineNuxtConfig } from 'nuxt/config';
import wasm from 'vite-plugin-wasm';
export default defineNuxtConfig({
  app: {
    baseURL: '/rmonkey',
  },
  srcDir: 'src',
  buildDir: './dist',
  ssr: false,
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
