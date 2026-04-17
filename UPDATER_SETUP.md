# DT-Tools Desktop Updater Setup

## 1. Generate signing key

```bash
pnpm tauri signer generate -w ~/.tauri/dt-tools-desktop-updater.key
```

Fill the generated public key into:

- `src-tauri/tauri.conf.json` -> `plugins.updater.pubkey`

## 2. Configure updater endpoint

Use the OSS path for DT-Tools Desktop:

- `https://<bucket>.oss-<region>.aliyuncs.com/d-space/dt-tools-desktop/<target>/latest.json`

## 3. Update version before publishing

程序内自动更新依赖版本号比较。

如果发布新版本但不修改版本号，客户端通常不会认为有新版本可更新。

发布前至少保持以下两个位置的 `version` 一致：

- `package.json`
- `src-tauri/tauri.conf.json`

例如：

- `0.1.0` -> `0.1.1`

也可以直接使用同步命令：

```bash
pnpm version:sync 0.1.1
```

如果希望直接准备发布版本，可以使用：

```bash
pnpm release:version 0.1.1
pnpm release:version 0.1.1 -- --push
```

说明：

- `pnpm release:version 0.1.1`：同步版本号、提交版本变更、创建 `v0.1.1` 标签
- `pnpm release:version 0.1.1 -- --push`：在上面的基础上继续推送分支和 tag，触发基于 tag 的 `release.yml`

## 4. Required GitHub secrets

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- `OSS_ACCESS_KEY_ID`
- `OSS_ACCESS_KEY_SECRET`
- `OSS_ENDPOINT`
- `OSS_REGION`
- `OSS_BUCKET`

`TAURI_SIGNING_PRIVATE_KEY` 可以保存两种形式，当前 workflow 都会自动兼容：

- 当前 Tauri CLI 生成的单行 base64 私钥文本
- 老格式 `*.key` 文件的完整两行文本

如果 GitHub Secret 被保存成单行并包含字面量 `\n`，workflow 也会在 runner 上自动还原并转成当前 Tauri CLI 需要的 base64 形式。

私钥内容应当类似这样开头:

```text
untrusted comment: rsign encrypted secret key
RWRTY...
```

如果缺少第一行注释，Tauri 在 CI 中会报错:

```text
failed to decode secret key: incorrect updater private key password: Missing comment in secret key
```

## 5. Remote publishing

Workflows:

- `.github/workflows/windows-tauri-build.yml`
- `.github/workflows/macos-tauri-build.yml`
- `.github/workflows/publish-oss-from-artifacts.yml`

## 6. Related docs

- 安全改程序名称但不影响更新链路：`docs/safe-app-renaming.md`
