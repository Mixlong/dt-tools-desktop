import { defineConfig } from "vite"
import vue from "@vitejs/plugin-vue"
import { quasar, transformAssetUrls } from "@quasar/vite-plugin"
import AutoImport from "unplugin-auto-import/vite"
import path from "path"

export default defineConfig({
  base: "./",
  plugins: [
    vue({
      template: { transformAssetUrls },
    }),
    quasar(),
    AutoImport({
      imports: ["vue", "vue-router", "pinia"],
      dts: false,
    }),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
  server: {
    host: "0.0.0.0",
    port: 3011,
    strictPort: true,
  },
})
