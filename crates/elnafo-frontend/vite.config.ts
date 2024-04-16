import { fileURLToPath, URL } from "node:url"
import { defineConfig } from "vite"
import vue from "@vitejs/plugin-vue"
import vueJsx from "@vitejs/plugin-vue-jsx"

export default defineConfig({
    plugins: [
        vue(),
        vueJsx(),
    ],
    build: {
        rollupOptions: {
            output: {
                entryFileNames: "resources/assets/[name].js",
                assetFileNames: "resources/assets/[name][extname]",
                chunkFileNames: "resources/assets/[name].js"
            }
        }
    },
    resolve: {
        alias: {
            "@": fileURLToPath(new URL("./src", import.meta.url))
        }
    }
})
