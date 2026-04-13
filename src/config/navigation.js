import { shellRoutes } from "@/router"

const HIDDEN_NAV_PATHS = new Set([
  "tools",
  "support",
])

export function isNavRouteAvailable(path) {
  const normalizedPath = String(path || "").replace(/^\/+/, "")

  return shellRoutes.some((route) => (
    route.path === normalizedPath
    && route.meta?.nav
    && !HIDDEN_NAV_PATHS.has(route.path)
  ))
}

export const navSections = shellRoutes
  .filter((route) => isNavRouteAvailable(route.path))
  .sort((left, right) => (left.meta?.order ?? 0) - (right.meta?.order ?? 0))
  .map((route) => ({
    labelKey: route.meta?.navLabelKey || route.meta?.titleKey || route.path,
    icon: route.meta?.icon || "chevron_right",
    to: `/${route.path}`,
  }))
