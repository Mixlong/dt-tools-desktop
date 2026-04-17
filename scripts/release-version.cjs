const { execFileSync } = require("child_process")
const fs = require("fs")
const path = require("path")

const rootDir = path.resolve(__dirname, "..")
const packageJsonPath = path.join(rootDir, "package.json")

const args = process.argv.slice(2)
const version = String(args[0] || "").trim()
const shouldPush = args.includes("--push")
const shouldForce = args.includes("--force")

if (!version) {
  throw new Error("请提供版本号，例如：pnpm release:version 0.1.1")
}

const versionPattern = /^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/
if (!versionPattern.test(version)) {
  throw new Error(`版本号格式不合法: ${version}`)
}

function run(cmd, cmdArgs) {
  execFileSync(cmd, cmdArgs, {
    cwd: rootDir,
    stdio: "inherit",
  })
}

function getOutput(cmd, cmdArgs) {
  return execFileSync(cmd, cmdArgs, {
    cwd: rootDir,
    stdio: ["ignore", "pipe", "pipe"],
    encoding: "utf8",
  }).trim()
}

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"))
const currentVersion = String(packageJson.version || "").trim()
const tagName = `v${version}`

function hasRemoteTag(name) {
  return Boolean(getOutput("git", ["ls-remote", "--tags", "origin", name]))
}

if (currentVersion === version) {
  console.log(`当前版本已经是 ${version}，继续检查 git tag 和提交状态`)
} else {
  run("pnpm", ["version:sync", version])
}

const existingTag = getOutput("git", ["tag", "--list", tagName])
if (existingTag) {
  if (!shouldForce) {
    throw new Error(`git tag 已存在: ${tagName}`)
  }

  run("git", ["tag", "-d", tagName])
}

if (shouldForce && hasRemoteTag(tagName)) {
  run("git", ["push", "origin", `:refs/tags/${tagName}`])
}

run("git", ["add", "package.json", "src-tauri/tauri.conf.json"])

const diffCached = getOutput("git", ["diff", "--cached", "--name-only"])
if (!diffCached) {
  console.log("版本文件没有变化，跳过提交")
} else {
  run("git", ["commit", "-m", `chore: release ${tagName}`])
}

run("git", ["tag", "-a", tagName, "-m", `Release ${tagName}`])

if (shouldPush) {
  const branch = getOutput("git", ["rev-parse", "--abbrev-ref", "HEAD"])
  run("git", ["push", "origin", branch])
  run("git", ["push", "origin", tagName])
  console.log(`已推送分支 ${branch} 和标签 ${tagName}`)
} else {
  console.log(`已创建标签 ${tagName}`)
  console.log(`下一步执行：git push origin HEAD && git push origin ${tagName}`)
}
