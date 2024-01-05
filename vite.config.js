import { defineConfig } from 'vite'
import { svelte, vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import path from 'path'
import autoprefixer from 'autoprefixer'

export default defineConfig({
  clearScreen: false,
  server: {
    port: 5448,
    strictPort: true,
  },
  build: {
    sourcemap: true,
    target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'],
  },
  resolve: {
    alias: {
      $lib: path.resolve(__dirname, './src/lib'),
      $routes: path.resolve(__dirname, './src/routes'),
    },
  },
  plugins: [
    svelte({
      preprocess: vitePreprocess({
        postcss: {
          plugins: [autoprefixer],
        },
      }),
    }),
  ],
})
