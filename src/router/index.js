import { createRouter, createWebHashHistory } from "vue-router"

const Layout = () => import("@/layout/AppLayout.vue")

export const shellRoutes = [
  {
    path: "home",
    name: "home",
    component: () => import("@/views/HomeView.vue"),
    meta: { titleKey: "nav.home", icon: "home", order: 1, keepAlive: true },
  },
  {
    path: "tools",
    name: "tools",
    component: () => import("@/views/tools/ToolsView.vue"),
    meta: { titleKey: "nav.tools", icon: "construction", nav: true, order: 2, keepAlive: true },
  },
  {
    path: "tools/serial",
    name: "tools-serial",
    redirect: () => ({ path: "/tools", query: { tab: "serial" } }),
    meta: { titleKey: "nav.toolsSerial" },
  },
  {
    path: "tools/can",
    name: "tools-can",
    redirect: () => ({ path: "/tools", query: { tab: "can" } }),
    meta: { titleKey: "nav.toolsCan" },
  },
  {
    path: "tools/wiring",
    name: "tools-wiring",
    redirect: () => ({ path: "/tools", query: { tab: "wiring" } }),
    meta: { titleKey: "nav.toolsWiring" },
  },
  {
    path: "config",
    name: "config",
    component: () => import("@/views/ConfigView.vue"),
    meta: { titleKey: "nav.settings", navLabelKey: "nav.settings", icon: "tune", nav: true, order: 3, keepAlive: true },
  },
  {
    path: "software",
    name: "software",
    component: () => import("@/views/SoftwareView.vue"),
    meta: { titleKey: "nav.software", navLabelKey: "nav.software", icon: "system_update_alt", nav: true, order: 4, keepAlive: true },
  },
  {
    path: "unimaster-about",
    name: "unimaster-about",
    component: () => import("@/views/UniMasterAboutView.vue"),
    meta: { titleKey: "nav.unimasterAbout", navLabelKey: "nav.unimasterAbout", icon: "info", nav: true, order: 5, keepAlive: true },
  },
  {
    path: "settings",
    name: "settings",
    component: () => import("@/views/SettingsView.vue"),
    meta: { titleKey: "nav.systemSettings", navLabelKey: "nav.systemSettings", icon: "settings", order: 6, keepAlive: true },
  },
  {
    path: "support",
    name: "support",
    component: () => import("@/views/SupportView.vue"),
    meta: { titleKey: "nav.support", navLabelKey: "nav.support", icon: "help", nav: true, order: 7, keepAlive: true },
  },
]

const routes = [
  {
    path: "/",
    component: Layout,
    redirect: "/config",
    children: shellRoutes,
  },
]

export default createRouter({
  history: createWebHashHistory(),
  routes,
})
