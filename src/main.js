import { createApp } from "vue"
import ElementPlus from "element-plus"
import locale from "element-plus/es/locale/lang/zh-cn"
import "element-plus/dist/index.css"
import * as ElementPlusIconsVue from "@element-plus/icons-vue"

import App from "./App.vue"
import router from "./router"
import store from "./store"
import "./styles.scss"
import { checkForAppUpdateWithPrompt } from "./updater"
import { shouldCheckUpdatesOnLaunch } from "./utils/preferences"

const app = createApp(App)

app.use(router)
app.use(store)
app.use(ElementPlus, { locale, size: "default" })

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

app.mount("#app")

router.isReady().then(() => {
  if (shouldCheckUpdatesOnLaunch()) {
    checkForAppUpdateWithPrompt()
  }
})
