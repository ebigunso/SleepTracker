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
      '/auth': {
        target,
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/auth/, '')
      },
      '/sleep': {
        target,
        changeOrigin: true,
        bypass(req) {
          const accept = req.headers['accept'] || '';
          if (req.method === 'GET' && accept.includes('text/html')) {
            // Let SvelteKit handle UI route navigations like /sleep/new
            return false;
          }
        }
      },
      '/exercise': {
        target,
        changeOrigin: true,
        bypass(req) {
          const accept = req.headers['accept'] || '';
          if (req.method === 'GET' && accept.includes('text/html')) {
            return false;
          }
        }
      },
      '/note': {
        target,
        changeOrigin: true,
        bypass(req) {
          const accept = req.headers['accept'] || '';
          if (req.method === 'GET' && accept.includes('text/html')) {
            return false;
          }
        }
      },
      '/health': { target, changeOrigin: true }
    }
  }
});
