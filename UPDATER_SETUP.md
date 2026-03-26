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

## 4. Remote publishing

Workflows:

- `.github/workflows/windows-tauri-build.yml`
- `.github/workflows/macos-tauri-build.yml`
- `.github/workflows/publish-oss-from-artifacts.yml`
