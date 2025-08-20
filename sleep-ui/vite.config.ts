import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
const target = process.env.PROXY_TARGET ?? 'http://localhost:8080';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  server: {
    port: 5173,
    proxy: {
      '/api': { target, changeOrigin: true },
      '/auth': {
        target,
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/auth/, '')
      },
      '/sleep': { target, changeOrigin: true },
      '/exercise': { target, changeOrigin: true },
      '/note': { target, changeOrigin: true },
      '/health': { target, changeOrigin: true }
    }
  }
});
