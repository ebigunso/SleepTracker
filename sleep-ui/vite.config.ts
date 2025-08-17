import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  server: {
    port: 5173,
    proxy: {
      '/api': { target: 'http://localhost:8080', changeOrigin: true },
      '/login.json': { target: 'http://localhost:8080', changeOrigin: true },
      '/logout': { target: 'http://localhost:8080', changeOrigin: true },
      '/sleep': { target: 'http://localhost:8080', changeOrigin: true },
      '/exercise': { target: 'http://localhost:8080', changeOrigin: true },
      '/note': { target: 'http://localhost:8080', changeOrigin: true },
      '/health': { target: 'http://localhost:8080', changeOrigin: true }
    }
  }
});
