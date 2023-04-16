// https://nuxt.com/docs/api/configuration/nuxt-config
import wasm from "vite-plugin-wasm";
export default defineNuxtConfig({
  nitro: {
    experimental: {
      wasm: true,
    },
  },
  vite: {
    plugins: [wasm()],
  },
});
