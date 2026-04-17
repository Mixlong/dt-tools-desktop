# DT-Tools 安全改名说明

## 目标

在不影响现有安装链路、自动更新链路、发布脚本和 OSS 下载地址的前提下，只修改用户可见的程序名称。

## 可以改的内容

这些内容只影响用户看到的名称，一般不会影响 updater 链路：

- `src-tauri/tauri.conf.json` 中的 `productName`
- `src-tauri/tauri.conf.json` 中 `app.windows[].title`
- 前端界面里的品牌文案、标题文案、帮助页文案

## 不要改的内容

这些内容会影响应用身份、自动更新或发布地址，除非明确要做链路迁移，否则不要改：

- `src-tauri/tauri.conf.json` 中的 `identifier`
- `src-tauri/tauri.conf.json` 中 `plugins.updater.endpoints`
- `src-tauri/tauri.conf.json` 中 `plugins.updater.pubkey`
- `.github/workflows/windows-tauri-build.yml` 中固定安装包文件名
- `.github/workflows/macos-tauri-build.yml` 中固定安装包文件名
- `src/updater.js` 中手动下载兜底地址

## 当前更新链路

当前 updater 与下载链路依赖以下固定地址和文件名：

- Windows 安装包：`https://bikewise.oss-cn-shenzhen.aliyuncs.com/d-space/dt-tools-desktop/windows/DT-Tools-x64-setup.exe`
- macOS 安装包：`https://bikewise.oss-cn-shenzhen.aliyuncs.com/d-space/dt-tools-desktop/darwin/DT-Tools.dmg`
- Windows updater：`https://bikewise.oss-cn-shenzhen.aliyuncs.com/d-space/dt-tools-desktop/windows/latest.json`
- macOS updater：`https://bikewise.oss-cn-shenzhen.aliyuncs.com/d-space/dt-tools-desktop/darwin/latest.json`

如果只是改程序显示名称，以上地址建议保持不变。

## 推荐做法

如果需求只是“把程序名称换成新的展示名”，按下面范围修改即可：

1. 修改 `productName`
2. 修改窗口标题
3. 修改界面文案
4. 不改 `identifier`
5. 不改 updater 公钥、更新地址、固定安装包名

## 改名后的检查项

改名后建议至少检查下面几项：

- 应用窗口标题是否正确
- 安装后的系统应用名称是否符合预期
- 应用内“检查更新”是否还能正常工作
- 现有下载地址是否仍然可用
- GitHub Actions 打包产物名称是否仍然符合发布脚本预期

## 什么时候需要整体迁移

如果你想一起修改下面任意内容，就不再属于“安全改名”，而是“发布链路迁移”：

- 安装包文件名
- OSS 路径
- updater 地址
- 应用标识 `identifier`
- updater 签名密钥

这种情况需要同步修改：

- Tauri 配置
- GitHub Actions workflow
- OSS 发布地址
- 前端手动下载地址
- 用户分发文案
