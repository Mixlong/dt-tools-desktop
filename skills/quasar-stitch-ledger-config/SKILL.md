---
name: quasar-stitch-ledger-config
description: Restyle Quasar desktop configuration pages to match a Stitch reference using the Architectural Ledger system: tonal surface layering, restrained blue emphasis, editorial hierarchy, and no-line sectioning while preserving business logic.
---

# Quasar Stitch Ledger Config

Use this skill when a Quasar desktop page should be rebuilt from a Stitch reference that uses:

- light editorial industrial UI
- tonal surface layering instead of hard dividers
- restrained primary blue emphasis
- compact but readable parameter forms
- right-side status or inspector context

This skill is intended for dense desktop tooling pages like parameter editors, flashing tools, protocol consoles, or operator workbenches.

## Core Rules

1. Preserve logic first.
- Do not rewrite API calls, store wiring, watchers, computed values, or validation flow unless required by the brief.
- Keep field keys, option values, and transport logic unchanged.

2. Spacing must follow repository tokens.
- Do not hardcode page-level padding in view root containers.
- First inherit spacing from the existing shell or page wrapper instead of adding local outer padding.
- When local panel/content padding is still required, use existing spacing tokens from `src/styles.scss`, with `var(--dt-space-3)` as the default baseline unless the surrounding page already uses another tokenized value.
- Do not introduce raw values like `padding: 24px` or `padding: 20px` in page-scoped styles when a `--dt-space-*` token should be used.

3. Use tonal hierarchy, not line-heavy cards.
- Canvas: `surface`
- Main work area: `surface_container_low`
- Interactive content blocks: `surface_container_lowest`
- Only use borders as ghost boundaries with low opacity.

4. Follow the no-line rule.
- Do not separate sections with strong 1px borders.
- Use background shifts, spacing, and tonal layering to define hierarchy.
- If a boundary is still necessary, use a low-opacity outline only.

5. Keep emphasis limited.
- Primary action: blue gradient or solid primary blue.
- Secondary action: neutral surface.
- Status badges: muted neutral or soft state tint.
- Avoid multiple accent colors on the same page.

6. Keep industrial density.
- Prefer 2-3 column form grids on desktop.
- Use small labels and strong value contrast.
- Keep operator-critical controls above the fold.

## Layout Pattern

For configuration pages, prefer this structure:

1. Top operation rail
- transport mode
- baud/frame settings
- read/write/import/export actions

2. Main parameter workspace
- group pills or tabs for sections
- repeated section blocks with title + one-line descriptor
- dense form grid inside each section

3. Secondary context rail
- device summary
- runtime status
- workflow hints or constraints

## Quasar Guidance

- Prefer `QBtnGroup + QBtn` over `QBtnToggle` when exact active/inactive styling must match a design reference.
- Use `QSelect`, `QInput`, `QScrollArea`, `QDialog`, `QCard`, and `QBtn` as the base primitives.
- Use local scoped styles in the page first; only promote tokens to globals when multiple pages share them.

## Styling Targets

- Page background: soft gray `surface`
- Panel background: `surface_container_low`
- Content block background: white `surface_container_lowest`
- Radius: 8px
- Shadows: soft ambient only for floating or glass panels
- Labels: `on_surface_variant`
- Values/headings: `on_surface`

## Editing Workflow

1. Read the page component, surrounding layout, and shared theme styles.
2. Identify the existing shell and keep it.
3. Move the page toward tonal layers, simpler cards, and calmer emphasis.
4. Remove visually heavy bars, gradients, and divider lines unless they encode real meaning.
5. Verify that all interactions still work.
6. Run the project build after changes.

## Output Contract

After applying this skill, report:

- which page structure changed
- which colors and surface layers now map to the Stitch reference
- what logic was intentionally preserved
- whether build verification passed
