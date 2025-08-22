import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
const target = process.env.PROXY_TARGET ?? 'http://localhost:8080';
const isTest = process.env.VITEST;

export default defineConfig({
  plugins: isTest ? [] : [tailwindcss(), sveltekit()],
  server: {
    port: 5173,
    proxy: {
      '/api': { target, changeOrigin: true },
    }
  }
});
