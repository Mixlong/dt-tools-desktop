export function normalizeVersion(value) {
  const normalized = String(value || "").trim()
  const match = normalized.match(/v?(\d+(?:\.\d+)+)/i)
  return match?.[1] || normalized.replace(/^v/i, "")
}

export function compareVersions(left, right) {
  const leftParts = normalizeVersion(left)
    .split(".")
    .map((item) => Number.parseInt(item, 10) || 0)
  const rightParts = normalizeVersion(right)
    .split(".")
    .map((item) => Number.parseInt(item, 10) || 0)
  const maxLength = Math.max(leftParts.length, rightParts.length)

  for (let index = 0; index < maxLength; index += 1) {
    const leftValue = leftParts[index] || 0
    const rightValue = rightParts[index] || 0
    if (leftValue > rightValue) {
      return 1
    }
    if (leftValue < rightValue) {
      return -1
    }
  }

  return 0
}
