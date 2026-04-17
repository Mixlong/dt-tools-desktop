# DT-Tools Desktop

DT-Tools desktop application built with Tauri + Vue.

## Development

```bash
pnpm install
pnpm dev:web
pnpm dev:desktop
```

## Build

```bash
pnpm build
pnpm build:desktop
```

## Version Sync

```bash
pnpm version:sync 0.1.1
```

This updates both `package.json` and `src-tauri/tauri.conf.json`.

## Release

```bash
pnpm release:version 0.1.1
pnpm release:version 0.1.1 -- --push
pnpm release:version 0.1.1 -- --force --push
```

The release command syncs versions, creates a release commit, and tags `v<version>`.
Use `--push` to push the current branch and tag to GitHub and trigger the tag-based release workflow.
Use `--force` only when you need to replace an existing local/remote tag for the same version.
