import { defineConfig } from 'vite'
export default defineConfig({
  root: 'frontend', publicDir: 'public', build: { outDir: '../dist', emptyOutDir: true, target: 'es2022' },
  test: { environment: 'node', include: ['src/**/*.test.ts'] }
})
