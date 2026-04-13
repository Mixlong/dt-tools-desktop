<template>
  <div class="tool-shell">
    <section class="panel tool-shell__main">
      <div class="tool-shell__head">
        <div>
          <span class="section-eyebrow">{{ t("tools.shell.operationPanel") }}</span>
          <h2>{{ title }}</h2>
          <p>{{ description }}</p>
        </div>
        <span class="metric-chip">
          <span class="metric-chip__dot" />
          {{ t("tools.shell.desktopWorkflow") }}
        </span>
      </div>
      <q-scroll-area class="tool-shell__scroll">
        <slot name="main" />
      </q-scroll-area>
    </section>

    <aside class="panel tool-shell__side">
      <div class="tool-shell__side-head">
        <span class="section-eyebrow">{{ t("tools.shell.inspector") }}</span>
        <h3>{{ t("tools.shell.helperTitle") }}</h3>
        <p>{{ t("tools.shell.helperDescription") }}</p>
      </div>
      <q-scroll-area class="tool-shell__scroll">
        <slot name="side" />
      </q-scroll-area>
    </aside>
  </div>
</template>

<script setup>
import { useI18n } from "vue-i18n"

const { t } = useI18n()

defineProps({
  title: String,
  description: String,
})
</script>

<style scoped lang="scss">
.tool-shell {
  display: grid;
  grid-template-columns: minmax(0, 1.6fr) minmax(320px, 0.8fr);
  gap: var(--dt-space-3);
  height: 100%;
  min-height: 0;
}

.tool-shell__main,
.tool-shell__side {
  display: flex;
  flex-direction: column;
  min-height: 0;
  padding: 20px;
}

.tool-shell__main {
  background: var(--dt-gloss-surface);
}

.tool-shell__head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.tool-shell__head p {
  margin: 10px 0 0;
  color: var(--dt-text-secondary);
  line-height: 1.7;
}

.tool-shell__side-head {
  margin-bottom: 16px;
}

.tool-shell__side-head p {
  margin: 8px 0 0;
  color: var(--dt-text-secondary);
  line-height: 1.65;
}

.tool-shell__scroll {
  flex: 1;
  min-height: 0;
}

@media (max-width: 1080px) {
  .tool-shell {
    grid-template-columns: 1fr;
  }

  .tool-shell__scroll {
    min-height: auto;
  }
}
</style>
