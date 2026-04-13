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

## 3. Required GitHub secrets

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- `OSS_ACCESS_KEY_ID`
- `OSS_ACCESS_KEY_SECRET`
- `OSS_ENDPOINT`
- `OSS_REGION`
- `OSS_BUCKET`

`TAURI_SIGNING_PRIVATE_KEY` 推荐直接保存 `*.key` 文件的完整文本内容。
如果 GitHub Secret 被保存成单行并包含字面量 `\n`，或者你更方便存成 base64，当前 workflow 也会在 runner 上自动还原。

私钥内容应当类似这样开头:

```text
untrusted comment: minisign encrypted secret key
RWRTY...
```

如果缺少第一行注释，Tauri 在 CI 中会报错:

```text
failed to decode secret key: incorrect updater private key password: Missing comment in secret key
```

## 4. Remote publishing

Workflows:

- `.github/workflows/windows-tauri-build.yml`
- `.github/workflows/macos-tauri-build.yml`
- `.github/workflows/publish-oss-from-artifacts.yml`
