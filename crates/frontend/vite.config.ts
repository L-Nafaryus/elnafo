import { fileURLToPath, URL } from 'node:url'

import { resolve } from 'path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'

// https://vitejs.dev/config/
export default defineConfig({
    //base: '/',
    plugins: [
        vue(),
        vueJsx(),
    ],
    /*root: './src',
    build: {
        outDir: "dist",
        rollupOptions: {
            input: {
                app: './src/index.html'
            }
        }
    },*/
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        }
    }
})
