const fs = require("fs")
const path = require("path")

const rootDir = path.resolve(__dirname, "..")
const packageJsonPath = path.join(rootDir, "package.json")
const tauriConfigPath = path.join(rootDir, "src-tauri", "tauri.conf.json")

const nextVersion = String(process.argv[2] || "").trim()

if (!nextVersion) {
  throw new Error("请提供版本号，例如：pnpm version:sync 0.1.1")
}

const versionPattern = /^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/
if (!versionPattern.test(nextVersion)) {
  throw new Error(`版本号格式不合法: ${nextVersion}`)
}

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"))
const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, "utf8"))

packageJson.version = nextVersion
tauriConfig.version = nextVersion

fs.writeFileSync(packageJsonPath, `${JSON.stringify(packageJson, null, 2)}\n`, "utf8")
fs.writeFileSync(tauriConfigPath, `${JSON.stringify(tauriConfig, null, 2)}\n`, "utf8")

console.log(`已同步版本号到 ${nextVersion}`)
