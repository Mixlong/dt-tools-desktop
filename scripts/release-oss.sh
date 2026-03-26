#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "[ERROR] missing command: $1"
    exit 1
  fi
}

require_env() {
  local name="$1"
  if [[ -z "${!name:-}" ]]; then
    echo "[ERROR] missing env: $name"
    exit 1
  fi
}

TARGET="${TARGET:-x86_64-pc-windows-msvc}"
BUNDLES="${BUNDLES:-nsis}"
OSS_DIR="${OSS_DIR:-d-space/dt-tools-desktop}"
OSS_BASE="${OSS_BASE:-}"

require_cmd pnpm
require_cmd ossutil
require_cmd find
require_env OSS_BASE
require_env TAURI_SIGNING_PRIVATE_KEY_PATH

if [[ ! -f "$TAURI_SIGNING_PRIVATE_KEY_PATH" ]]; then
  echo "[ERROR] signing key not found: $TAURI_SIGNING_PRIVATE_KEY_PATH"
  exit 1
fi

pnpm tauri build --target "$TARGET" --bundles "$BUNDLES"

BUNDLE_DIR="src-tauri/target/$TARGET/release/bundle"
EXE_FILE="$(find "$BUNDLE_DIR" -type f -name '*.exe' | head -n 1 || true)"
SIG_FILE="$(find "$BUNDLE_DIR" -type f -name '*.sig' | head -n 1 || true)"
LATEST_JSON="$(find "$BUNDLE_DIR" -type f -name 'latest.json' | head -n 1 || true)"

if [[ -z "$EXE_FILE" || -z "$SIG_FILE" || -z "$LATEST_JSON" ]]; then
  echo "[ERROR] missing release artifacts"
  exit 1
fi

DEST="${OSS_BASE%/}/$OSS_DIR"
FIXED_EXE_NAME="DT-Tools-x64-setup.exe"
FIXED_SIG_NAME="DT-Tools-x64-setup.exe.sig"

ossutil cp "$EXE_FILE" "$DEST/$FIXED_EXE_NAME" --update
ossutil cp "$SIG_FILE" "$DEST/$FIXED_SIG_NAME" --update
ossutil cp "$LATEST_JSON" "$DEST/latest.json" --update

echo "[OK] uploaded artifacts to $DEST"
