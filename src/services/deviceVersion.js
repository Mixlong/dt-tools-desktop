export const REMOTE_VERSION_REFRESH_EVENT = "dt-tools:remote-version-refresh"

export function requestRemoteVersionRefresh(detail = {}) {
  if (typeof window === "undefined") {
    return
  }

  window.dispatchEvent(new CustomEvent(REMOTE_VERSION_REFRESH_EVENT, { detail }))
}
