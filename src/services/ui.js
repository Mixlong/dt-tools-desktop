import { Dialog, Loading, Notify } from "quasar"
import { translate } from "@/i18n"

function normalizeMessage(message) {
  if (message instanceof Error) {
    return normalizeMessage(message.message)
  }

  const normalized = String(message ?? "")
    .replace(/^error:\s*/i, "")
    .replace(/^错误:\s*/i, "")
    .trim()

  return normalized || translate("updater.unknownError")
}

function baseNotify(message, color, icon) {
  Notify.create({
    message: normalizeMessage(message),
    color,
    icon,
    textColor: "white",
    classes: "dt-notify",
  })
}

export function notifySuccess(message) {
  baseNotify(message, "positive", "check_circle")
}

export function notifyError(message) {
  baseNotify(message, "negative", "error")
}

export function notifyInfo(message) {
  baseNotify(message, "info", "info")
}

export async function confirmAction({ title = "", message, ok = "", cancel = "" }) {
  return new Promise((resolve) => {
    Dialog.create({
      title: title || translate("common.actions.confirm"),
      message,
      ok: {
        label: ok || translate("common.actions.confirm"),
        color: "primary",
        unelevated: true,
      },
      cancel: {
        label: cancel || translate("common.actions.cancel"),
        flat: true,
      },
      persistent: true,
    })
      .onOk(() => resolve(true))
      .onCancel(() => resolve(false))
      .onDismiss(() => resolve(false))
  })
}

export async function withLoading(task, options = {}) {
  const { message = translate("common.messages.processing") } = options
  Loading.show({
    message,
    delay: 120,
  })

  try {
    return await task()
  } finally {
    Loading.hide()
  }
}
