# AGENTS.md

## Purpose

This repository is `dt-tools-desktop`, a Tauri desktop app built with Vue 3, Vite, Quasar, and Pinia.
This file is for coding agents working inside this repo.
Follow the observed repository conventions here instead of inventing new ones.

## Tech Stack

- Package manager: `pnpm`
- Frontend bundler: `vite`
- Desktop runtime: `tauri`
- UI framework: `quasar`
- App framework: `vue@3`
- State management: `pinia`
- Router: `vue-router`
- Language in app code: JavaScript + Vue SFCs
- Styling: SCSS in Vue SFCs and `src/styles.scss`

## Repository Structure

- `src/main.js` — app bootstrap, Quasar plugin setup, theme/init logic
- `src/router/index.js` — route table and lazy-loaded views
- `src/layout/AppLayout.vue` — main shell layout
- `src/views/` — page-level Vue components
- `src/views/tools/` — tool-specific pages
- `src/store/` — Pinia stores
- `src/api/` — thin wrappers around Tauri IPC calls
- `src/services/` — UI/service helpers
- `src/utils/` — pure utilities and data transforms
- `src/constants/` — protocol constants and option lists
- `src/styles.scss` — global tokens and shared styling
- `src-tauri/` — Tauri-side application assets/config

## Install / Run / Build Commands

Use `pnpm` for all package operations.

### Install

```bash
pnpm install
```

### Web development

```bash
pnpm dev:web
```

Notes:
- Runs Vite on port `3011`
- `--strictPort` is enabled, so port conflicts fail fast

### Default dev entry

```bash
pnpm dev
```

This currently aliases to `pnpm dev:web`.

### Desktop development

```bash
pnpm dev:desktop
```

### Web build

```bash
pnpm build
```

### Desktop build

```bash
pnpm build:desktop
```

### Preview production web build

```bash
pnpm preview
```

## Lint / Format / Typecheck Status

There is currently **no dedicated lint script**, **no dedicated format script**, and **no dedicated typecheck script** in `package.json`.

There is also no ESLint, Prettier, or TypeScript config file at the repo root based on the current repository scan.

Implications for agents:
- Do not claim lint passed unless you added and ran lint tooling yourself
- Do not claim typecheck passed for the whole repo; this is a JavaScript project, not a configured TS project
- Keep edits consistent with the existing source formatting instead of introducing a new formatter style

## Test Status

There is currently **no test script** in `package.json` and no detected Jest/Vitest config.

Observed current state:
- no `test` script
- no `vitest.config.*`
- no `jest.config.*`
- no `*.test.*` or `*.spec.*` files detected in the repo scan

### Running tests

At the moment, there is no repository-supported test command.

### Running a single test

Also not currently supported, because the repo does not contain a configured test runner.

If a future change adds tests, document the exact single-test command in this file.
Until then, do **not** invent or assume one.

## Routing and Navigation Conventions

- Router lives in `src/router/index.js`
- History mode is `createWebHashHistory()`
- Route components are lazy-loaded with `() => import(...)`
- Route metadata uses `meta.title` for page titles shown in layout
- New top-level pages should usually be added through the layout route tree rather than as isolated app roots

## Vue Component Conventions

- Use Vue 3 SFCs
- Prefer `<script setup>` for view components when consistent with surrounding code
- Keep page-level components under `src/views/`
- Keep layout concerns in `src/layout/`
- Use Quasar components for UI primitives instead of raw HTML where existing UI already uses Quasar equivalents
- Keep user-facing copy primarily in Chinese where the existing UI is already Chinese

Naming conventions seen in the repo:
- Vue component files: `PascalCase.vue`
- Store files: lower camel / descriptive file names like `device.js`, `meterConfig.js`
- Store composables: `useXxxStore`
- Functions and variables: `camelCase`
- Constants: `UPPER_SNAKE_CASE` for true constants, e.g. `ADAPTER_BAUD_RATE`

## Import Conventions

- Use ES modules everywhere
- Use double quotes, not single quotes
- Omit semicolons
- Group imports simply and keep them readable; the repo does not show a strict auto-sorted import convention
- Use the `@` alias for imports from `src`, e.g. `@/store/device`
- Use relative imports for nearby same-folder modules when that is already the local pattern

Examples from the codebase:
- `src/router/index.js` lazy-loads `@/views/...`
- `src/layout/AppLayout.vue` imports from `@/config`, `@/services`, `@/store`
- `src/main.js` imports local app modules with relative paths

## State Management Conventions

- Use Pinia for app/domain state
- Define stores with `defineStore(...)`
- Keep store state serializable and explicit
- Use getters for derived labels and display-friendly projections
- Put async side effects in store actions when they are state-centric
- Keep IPC details out of components when a store or API wrapper can own them

Patterns to follow:
- state in `state: () => ({ ... })`
- derived display data in `getters`
- async workflows in `actions`
- component code should call store actions instead of duplicating connection logic

## API and IPC Conventions

- `src/api/unimaster.js` is the model for Tauri IPC wrappers
- Keep IPC wrappers thin
- Export small named functions
- Pass explicit argument objects to `invoke(...)`
- Keep parsing/formatting helpers close to the API layer only when they are transport-related
- Put heavier protocol transforms in `src/utils/`

Do not:
- call Tauri `invoke(...)` directly from many unrelated components if an API wrapper already exists
- bury protocol constants inside view files

## Error Handling Conventions

- Throw `Error` with actionable human-readable messages when a workflow cannot continue
- In UI-facing async handlers, catch errors and surface them through shared UI helpers
- Prefer `notifyError(...)`, `notifySuccess(...)`, `notifyInfo(...)` from `src/services/ui.js`
- Use `confirmAction(...)` for confirmation dialogs
- Use `withLoading(...)` for operations that need a loading overlay

Observed pattern:
- store actions may throw
- layout/view components catch and call `notifyError(error)`
- avoid silent failures

Do not:
- swallow errors with empty `catch` blocks
- add ad hoc alert/dialog patterns when the shared UI service already fits

## Styling Conventions

- Global design tokens live in `src/styles.scss`
- Component-local styles commonly use `<style scoped lang="scss">`
- Reuse existing CSS variables such as `--dt-*` tokens before inventing new colors/spacings
- Preserve the current visual language: desktop control panels, cards, bordered surfaces, muted text, primary action emphasis

## Agent Workflow Expectations

- Prefer small, focused changes
- Match existing patterns before introducing new abstractions
- Do not add new dependencies unless necessary and justified
- Do not introduce a testing framework, lint setup, or formatter as part of an unrelated task unless explicitly requested
- When changing UI, inspect nearby Quasar-based views/layouts first
- When changing device workflows, inspect store + API wrapper + relevant view together

## Files Worth Reading Before Editing

- `package.json`
- `README.md`
- `vite.config.js`
- `src/main.js`
- `src/router/index.js`
- `src/layout/AppLayout.vue`
- `src/store/device.js`
- `src/api/unimaster.js`
- `src/services/ui.js`
- the target view/store/util file you plan to modify

## Repository-Specific Rule Files

No repository-local Cursor or Copilot instruction files were found during analysis.

Not found:
- `.cursor/rules/`
- `.cursorrules`
- `.github/copilot-instructions.md`

If any of these files are added later, merge their instructions into this document.

## Safe Change Checklist

Before finishing a task, verify:
- commands you mention actually exist in `package.json`
- any claimed test command is real
- imports use existing alias/style conventions
- new routes follow the hash-history lazy-load pattern
- new store logic fits existing Pinia structure
- UI errors surface through shared helpers
- styling reuses existing tokens and Quasar patterns where possible

## Bottom Line

This repo is a pnpm-managed Vue + Quasar + Tauri desktop app with Pinia stores and thin Tauri IPC wrappers.
Favor consistency over novelty.
Do not assume missing tooling exists.
If a command, rule file, test runner, or lint setup is absent from the repo, say so explicitly.
