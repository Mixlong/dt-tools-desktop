function bytesToText(bytes) {
  return new TextDecoder("utf-8").decode(new Uint8Array(bytes))
}

function hexStringToBytes(hexString) {
  const normalized = String(hexString || "").replace(/^0x/i, "").trim()
  const bytes = []

  for (let index = 0; index < normalized.length; index += 2) {
    bytes.push(Number.parseInt(normalized.slice(index, index + 2), 16))
  }

  return bytes
}

function getExtension(fileName = "") {
  const normalized = String(fileName).split("?")[0]
  const segments = normalized.split(".")
  return segments.length > 1 ? segments.pop().toLowerCase() : ""
}

export function parseUniMasterAppHexChunks(bytes) {
  const lines = bytesToText(bytes).split("\r\n")
  let lineIndex = 0
  let line = lines[lineIndex]
  const startAddress = [0, 0]
  const chunks = []
  const maxFrameLen = 116
  let current = []

  while (line) {
    if (!line.startsWith(":")) {
      break
    }

    if (line.length > 11) {
      const record = hexStringToBytes(line.substring(1))
      const recordType = record[3]

      if (recordType === 0x04) {
        startAddress[0] = record[4]
        startAddress[1] = record[5]
        line = lines[++lineIndex]
        continue
      }

      if (recordType === 0x01 || recordType === 0x05) {
        line = lines[++lineIndex]
        continue
      }

      const dataLength = record[0]
      if ((current.length + dataLength) > maxFrameLen && current.length) {
        chunks.push(current)
        current = []
      }

      const address = record.slice(1, 3)
      if (!current.length) {
        current.push(...startAddress)
        current.push(...address)
      }

      current.push(...record.slice(4, 4 + dataLength))
    }

    line = lines[++lineIndex]
  }

  if (current.length) {
    chunks.push(current)
  }

  return chunks
}

export function parseUniMasterUiTxtChunks(bytes) {
  const lines = bytesToText(bytes).split("\r\n")
  const chunks = []
  const maxFrameLen = 128
  let current = ""
  let emptyLineCount = 0

  for (const rawLine of lines) {
    const line = rawLine.trim()

    if (!line) {
      emptyLineCount += 1
      if (emptyLineCount >= 5) {
        break
      }
      continue
    }

    emptyLineCount = 0

    if (line.startsWith("//")) {
      continue
    }

    const segments = line.split(":")
    if (segments.length < 2) {
      continue
    }

    const address = segments[0]
    const payload = segments[1]

    if ((current.length + payload.length) > maxFrameLen && current) {
      chunks.push(hexStringToBytes(current))
      current = ""
    }

    if (!current) {
      current += address
    }
    current += payload
  }

  if (current) {
    chunks.push(hexStringToBytes(current))
  }

  return chunks
}

export function parseUniMasterUiBinChunks(bytes) {
  const source = Array.from(bytes || [])
  const chunks = []
  let address = 0

  for (let index = 0; index < source.length; index += 128) {
    const payload = source.slice(index, index + 128)
    const frame = [
      (address >> 24) & 0xff,
      (address >> 16) & 0xff,
      (address >> 8) & 0xff,
      address & 0xff,
      ...payload,
    ]
    chunks.push(frame)
    address += payload.length
  }

  return chunks
}

export function buildUniMasterVersionChunks(kind, fileName, bytes) {
  const extension = getExtension(fileName)

  if (kind === "app") {
    return parseUniMasterAppHexChunks(bytes)
  }

  if (kind === "ui") {
    if (extension === "txt") {
      return parseUniMasterUiTxtChunks(bytes)
    }
    if (extension === "bin") {
      return parseUniMasterUiBinChunks(bytes)
    }
  }

  throw new Error(`UniMaster 版本升级不支持的文件类型: ${kind}.${extension}`)
}
