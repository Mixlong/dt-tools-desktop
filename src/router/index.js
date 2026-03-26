import { createRouter, createWebHashHistory } from "vue-router"

const Layout = () => import("@/layout/AppLayout.vue")

const routes = [
  {
    path: "/",
    component: Layout,
    redirect: "/home",
    children: [
      { path: "home", component: () => import("@/views/HomeView.vue"), meta: { title: "首页" } },
      { path: "tools/serial", component: () => import("@/views/tools/SerialToolView.vue"), meta: { title: "串口抓包" } },
      { path: "tools/can", component: () => import("@/views/tools/CanToolView.vue"), meta: { title: "CAN 抓包" } },
      { path: "tools/wiring", component: () => import("@/views/tools/WiringView.vue"), meta: { title: "接线指引" } },
      { path: "config", component: () => import("@/views/ConfigView.vue"), meta: { title: "仪表参数" } },
      { path: "software", component: () => import("@/views/SoftwareView.vue"), meta: { title: "软件" } },
      { path: "settings", component: () => import("@/views/SettingsView.vue"), meta: { title: "设置" } },
      { path: "support", component: () => import("@/views/SupportView.vue"), meta: { title: "支持" } },
    ],
  },
]

export default createRouter({
  history: createWebHashHistory(),
  routes,
})
