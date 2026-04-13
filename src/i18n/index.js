import { createI18n } from "vue-i18n"
import { Lang } from "quasar"
import quasarLangEn from "quasar/lang/en-US"
import quasarLangZh from "quasar/lang/zh-CN"
import commonEn from "./common/en"
import commonZh from "./common/zh"
import configEn from "./config/en"
import configZh from "./config/zh"
import deviceEn from "./device/en"
import deviceZh from "./device/zh"
import homeEn from "./home/en"
import homeZh from "./home/zh"
import layoutEn from "./layout/en"
import layoutZh from "./layout/zh"
import navEn from "./nav/en"
import navZh from "./nav/zh"
import settingsEn from "./settings/en"
import settingsZh from "./settings/zh"
import softwareEn from "./software/en"
import softwareZh from "./software/zh"
import supportEn from "./support/en"
import supportZh from "./support/zh"
import toolsEn from "./tools/en"
import toolsZh from "./tools/zh"
import updaterEn from "./updater/en"
import updaterZh from "./updater/zh"
import { getStoredLocale, saveLocale } from "@/utils/preferences"

export const DEFAULT_LOCALE = "zh-CN"

const LOCALES = new Set(["zh-CN", "en-US"])

const quasarLangMap = {
  "zh-CN": quasarLangZh,
  "en-US": quasarLangEn,
}

const messages = {
  "zh-CN": {
    common: commonZh,
    nav: navZh,
    layout: layoutZh,
    home: homeZh,
    tools: toolsZh,
    config: configZh,
    software: softwareZh,
    settings: settingsZh,
    support: supportZh,
    device: deviceZh,
    updater: updaterZh,
  },
  "en-US": {
    common: commonEn,
    nav: navEn,
    layout: layoutEn,
    home: homeEn,
    tools: toolsEn,
    config: configEn,
    software: softwareEn,
    settings: settingsEn,
    support: supportEn,
    device: deviceEn,
    updater: updaterEn,
  },
}

export function normalizeLocale(locale) {
  return LOCALES.has(locale) ? locale : DEFAULT_LOCALE
}

export const i18n = createI18n({
  legacy: false,
  locale: normalizeLocale(getStoredLocale()),
  fallbackLocale: DEFAULT_LOCALE,
  messages,
})

export function translate(key, params) {
  return i18n.global.t(key, params)
}

export function getCurrentLocale() {
  return normalizeLocale(i18n.global.locale.value)
}

export function getTargetLocale(currentLocale = getCurrentLocale()) {
  return currentLocale === "zh-CN" ? "en-US" : "zh-CN"
}

export function getLocaleSwitchLabel(currentLocale = getCurrentLocale()) {
  return currentLocale === "zh-CN" ? "中" : "EN"
}

export function getDeviceLanguageCode(locale) {
  return normalizeLocale(locale) === "zh-CN" ? 0 : 1
}

export async function applyLocale(locale) {
  const resolvedLocale = normalizeLocale(locale)
  i18n.global.locale.value = resolvedLocale
  Lang.set(quasarLangMap[resolvedLocale])
  saveLocale(resolvedLocale)
  return resolvedLocale
}
