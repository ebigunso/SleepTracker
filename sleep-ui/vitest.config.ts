import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  resolve: {
    alias: {
      '$app/environment': resolve('tests/mocks/app-environment.ts'),
      '$lib': resolve('src/lib'),
    },
  },
  test: {
    environment: 'jsdom',
    globals: true,
    include: ['tests/unit/**/*.spec.ts'],
    exclude: ['node_modules', 'dist'],
  },
});
