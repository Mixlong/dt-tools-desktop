<template>
  <div class="support-page">
    <section class="page-intro panel support-hero">
      <div class="support-hero__copy">
        <span class="section-eyebrow">{{ t("support.eyebrow") }}</span>
        <h2>{{ t("support.title") }}</h2>
        <p>{{ t("support.description") }}</p>

        <div class="support-actions support-actions--hero">
          <q-btn
            push
            color="primary"
            icon="system_update_alt"
            :label="t('support.upgrade.primaryAction')"
            to="/software"
          />
          <q-btn
            outline
            color="primary"
            icon="menu_book"
            :label="t('support.manual.primaryAction')"
            @click="scrollToTopOfManual"
          />
        </div>
      </div>

      <div class="support-hero__side">
        <div class="support-stats">
          <article v-for="stat in heroStats" :key="stat.label" class="support-stat">
            <strong>{{ stat.value }}</strong>
            <span>{{ stat.label }}</span>
          </article>
        </div>

        <article class="support-status">
          <span class="section-eyebrow">{{ t("support.status.eyebrow") }}</span>
          <h3>{{ t("support.status.title") }}</h3>
          <p>{{ t("support.status.note") }}</p>
        </article>
      </div>
    </section>

    <section class="support-workspace">
      <article class="panel support-card support-card--feature">
        <div class="support-card__header">
          <div>
            <span class="section-eyebrow">{{ t("support.upgrade.eyebrow") }}</span>
            <h3>{{ t("support.upgrade.title") }}</h3>
            <p>{{ t("support.upgrade.description") }}</p>
          </div>

          <q-btn
            flat
            color="primary"
            icon="open_in_new"
            :label="t('support.upgrade.secondaryAction')"
            to="/software"
          />
        </div>

        <q-scroll-area class="support-card__scroll">
          <div class="support-step-list">
            <article v-for="(step, index) in upgradeSteps" :key="step.title" class="support-step">
              <div class="support-step__index">{{ index + 1 }}</div>
              <div class="support-step__content">
                <h4>{{ step.title }}</h4>
                <p>{{ step.description }}</p>
              </div>
            </article>
          </div>

          <div class="support-highlight-grid">
            <article v-for="item in upgradeHighlights" :key="item.title" class="support-highlight">
              <span class="support-icon-badge">
                <q-icon :name="item.icon" size="18px" />
              </span>
              <div>
                <h4>{{ item.title }}</h4>
                <p>{{ item.description }}</p>
              </div>
            </article>
          </div>
        </q-scroll-area>

        <div class="support-actions">
          <q-btn
            push
            color="primary"
            icon="system_update_alt"
            :label="t('support.upgrade.primaryAction')"
            to="/software"
          />
        </div>
      </article>

      <div class="support-side">
        <article class="panel support-card support-card--checklist">
          <div class="support-card__header">
            <span class="section-eyebrow">{{ t("support.checklist.eyebrow") }}</span>
            <h3>{{ t("support.checklist.title") }}</h3>
          </div>

          <q-scroll-area class="support-card__scroll">
            <ul class="support-checklist">
              <li v-for="item in checklistItems" :key="item">
                <span class="support-icon-badge support-icon-badge--success">
                  <q-icon name="task_alt" size="18px" />
                </span>
                <span>{{ item }}</span>
              </li>
            </ul>
          </q-scroll-area>
        </article>

        <article class="panel support-card support-card--manual">
          <div class="support-card__header">
            <div>
              <span class="section-eyebrow">{{ t("support.manual.eyebrow") }}</span>
              <h3>{{ t("support.manual.title") }}</h3>
              <p>{{ t("support.manual.description") }}</p>
            </div>
          </div>

          <q-scroll-area ref="manualScrollRef" class="support-card__scroll">
            <div class="support-entry-grid">
              <article v-for="item in entryCards" :key="item.title" class="support-entry">
                <span class="support-icon-badge">
                  <q-icon :name="item.icon" size="18px" />
                </span>
                <div>
                  <h4>{{ item.title }}</h4>
                  <p>{{ item.description }}</p>
                </div>
              </article>
            </div>

            <div class="support-manual-grid">
              <article v-for="section in manualSections" :key="section.title" class="support-manual">
                <div class="support-manual__head">
                  <span class="support-icon-badge">
                    <q-icon :name="section.icon" size="18px" />
                  </span>
                  <div>
                    <h4>{{ section.title }}</h4>
                    <p>{{ section.description }}</p>
                  </div>
                </div>

                <ul class="support-list">
                  <li v-for="point in section.points" :key="point">{{ point }}</li>
                </ul>
              </article>
            </div>

            <div class="support-faq-block">
              <div class="support-faq-block__head">
                <span class="section-eyebrow">{{ t("support.faqEyebrow") }}</span>
                <h3>{{ t("support.faqTitle") }}</h3>
              </div>

              <div class="support-faq-list">
                <q-expansion-item
                  v-for="item in faqItems"
                  :key="item.question"
                  dense
                  dense-toggle
                  expand-separator
                  icon="help_outline"
                  expand-icon="keyboard_arrow_down"
                  header-class="support-faq__header"
                  class="support-faq"
                  :label="item.question"
                >
                  <div class="support-faq__content">
                    {{ item.answer }}
                  </div>
                </q-expansion-item>
              </div>
            </div>
          </q-scroll-area>
        </article>
      </div>
    </section>
  </div>
</template>

<script setup>
import { computed, ref } from "vue"
import { useI18n } from "vue-i18n"

const { t, tm } = useI18n()

const manualScrollRef = ref(null)

const normalizeList = (value) => Array.isArray(value) ? value : []

const heroStats = computed(() => normalizeList(tm("support.heroStats")))
const upgradeSteps = computed(() => normalizeList(tm("support.upgrade.steps")))
const checklistItems = computed(() => normalizeList(tm("support.checklist.items")))
const faqItems = computed(() => normalizeList(tm("support.faqItems")))

const upgradeHighlights = computed(() => [
  {
    icon: "cloud_download",
    title: t("support.upgrade.highlights.onlineTitle"),
    description: t("support.upgrade.highlights.onlineDescription"),
  },
  {
    icon: "folder_open",
    title: t("support.upgrade.highlights.offlineTitle"),
    description: t("support.upgrade.highlights.offlineDescription"),
  },
  {
    icon: "verified",
    title: t("support.upgrade.highlights.safetyTitle"),
    description: t("support.upgrade.highlights.safetyDescription"),
  },
])

const entryCards = computed(() => [
  {
    icon: "system_update_alt",
    title: t("support.entry.upgradeTitle"),
    description: t("support.entry.upgradeDescription"),
  },
  {
    icon: "menu_book",
    title: t("support.entry.manualTitle"),
    description: t("support.entry.manualDescription"),
  },
  {
    icon: "troubleshoot",
    title: t("support.entry.troubleshootTitle"),
    description: t("support.entry.troubleshootDescription"),
  },
])

const manualSectionIcons = ["cable", "inventory_2", "troubleshoot"]

const manualSections = computed(() =>
  normalizeList(tm("support.manual.sections")).map((item, index) => ({
    ...item,
    icon: manualSectionIcons[index] || "description",
  })),
)

function scrollToTopOfManual() {
  manualScrollRef.value?.setScrollPosition("vertical", 0, 250)
}
</script>

<style scoped lang="scss">
.support-page {
  display: flex;
  flex-direction: column;
  gap: var(--dt-space-3);
  height: 100%;
  min-height: 0;
  overflow: hidden;
  padding: var(--dt-space-3);
}

.support-hero {
  flex: 0 0 auto;
  gap: var(--dt-space-6);
}

.support-hero__copy {
  flex: 1 1 0;
  min-width: 0;
}

.support-hero__copy h2,
.support-card__header h3,
.support-status h3 {
  margin: 0;
  font-family: var(--dt-font-ui);
  color: var(--dt-text-primary);
  font-size: 18px;
  font-weight: 700;
  line-height: 1.35;
  letter-spacing: 0;
}

.support-highlight h4,
.support-entry h4,
.support-manual h4,
.support-faq-block__head h3,
.support-step__content h4 {
  margin: 0;
  font-family: var(--dt-font-ui);
  color: var(--dt-text-primary);
  font-size: 15px;
  font-weight: 700;
  line-height: 1.4;
}

.support-hero__copy p,
.support-status p,
.support-card__header p,
.support-step__content p,
.support-highlight p,
.support-entry p,
.support-manual p,
.support-faq__content {
  margin: 0;
  color: var(--dt-text-secondary);
  font-size: 14px;
  line-height: 1.65;
}

.support-hero__side {
  display: flex;
  flex-direction: column;
  gap: var(--dt-space-3);
  width: min(100%, 360px);
}

.support-stats {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--dt-space-3);
}

.support-stat,
.support-status,
.support-highlight,
.support-entry,
.support-manual,
.support-step,
.support-faq {
  border: 1px solid var(--dt-border);
  border-radius: var(--dt-radius-panel);
  background: color-mix(in srgb, var(--dt-bg-panel-soft) 72%, transparent);
}

.support-stat {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 14px 16px;
}

.support-stat strong {
  font-size: 18px;
  line-height: 1;
}

.support-stat span {
  color: var(--dt-text-muted);
  font-size: 11px;
}

.support-status,
.support-card {
  padding: 20px;
}

.support-status {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.support-workspace {
  display: grid;
  grid-template-columns: minmax(0, 1.4fr) minmax(320px, 0.8fr);
  gap: var(--dt-space-3);
  flex: 1;
  min-height: 0;
}

.support-side {
  display: grid;
  grid-template-rows: minmax(0, 0.85fr) minmax(0, 1.15fr);
  gap: var(--dt-space-3);
  min-height: 0;
}

.support-card {
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.support-card__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--dt-space-3);
  flex: 0 0 auto;
}

.support-card__scroll {
  flex: 1;
  min-height: 0;
  margin-top: var(--dt-space-4);
}

.support-card__scroll :deep(.q-scrollarea__content) {
  padding-right: 2px;
}

.support-step-list,
.support-highlight-grid,
.support-entry-grid,
.support-manual-grid,
.support-faq-list {
  display: grid;
  gap: var(--dt-space-3);
}

.support-highlight-grid,
.support-entry-grid {
  margin-top: var(--dt-space-3);
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.support-step-list {
  margin-bottom: var(--dt-space-3);
}

.support-step {
  display: grid;
  grid-template-columns: 24px minmax(0, 1fr);
  gap: 12px;
  padding: 16px;
}

.support-step__index {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 999px;
  background: var(--dt-brand-primary-soft);
  color: var(--dt-text-contrast);
  font-size: 12px;
  font-weight: 700;
}

.support-step__content {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.support-highlight,
.support-entry,
.support-manual {
  display: flex;
  gap: 12px;
  padding: 16px;
}

.support-manual {
  flex-direction: column;
  gap: var(--dt-space-3);
}

.support-manual__head {
  display: grid;
  grid-template-columns: 40px minmax(0, 1fr);
  gap: 12px;
}

.support-icon-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 12px;
  border: 1px solid color-mix(in srgb, var(--dt-accent) 18%, var(--dt-border));
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.98), rgba(243, 236, 224, 0.98));
  color: var(--dt-accent);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.92), 0 6px 14px rgba(168, 127, 50, 0.12);
  flex-shrink: 0;
}

.support-icon-badge--success {
  border-color: color-mix(in srgb, var(--dt-success) 28%, var(--dt-border));
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.98), rgba(225, 247, 236, 0.98));
  color: var(--dt-success);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.92), 0 6px 14px rgba(16, 185, 129, 0.12);
}

.support-checklist {
  display: flex;
  flex-direction: column;
  gap: var(--dt-space-3);
  min-height: 100%;
  margin: 0;
  padding: 0;
  list-style: none;
}

.support-checklist li {
  display: grid;
  grid-template-columns: 40px minmax(0, 1fr);
  align-items: flex-start;
  gap: var(--dt-space-3);
  padding: var(--dt-space-3);
  border: 1px solid var(--dt-border);
  border-radius: var(--dt-radius-panel);
  background: color-mix(in srgb, var(--dt-bg-panel-soft) 72%, transparent);
  color: var(--dt-text-secondary);
  font-size: 14px;
  line-height: 1.65;
}

.support-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin: 0;
  padding-left: 18px;
  color: var(--dt-text-secondary);
  font-size: 14px;
  line-height: 1.7;
}

.support-faq-block {
  display: flex;
  flex-direction: column;
  gap: var(--dt-space-3);
  margin-top: var(--dt-space-3);
  padding-top: var(--dt-space-3);
  border-top: 1px dashed color-mix(in srgb, var(--dt-border) 72%, white);
}

.support-faq-block__head {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.support-faq {
  overflow: hidden;
  transition: border-color 180ms ease, background 180ms ease;
}

.support-faq:hover {
  border-color: var(--dt-border-strong);
  background: rgba(255, 255, 255, 0.04);
}

.support-faq__content {
  padding: 0 var(--dt-space-3) var(--dt-space-3);
}

.support-faq :deep(.support-faq__header) {
  min-height: 52px;
  color: var(--dt-text-primary);
  font-weight: 600;
}

.support-faq :deep(.q-expansion-item__toggle-icon) {
  color: var(--dt-accent);
  font-size: 22px;
}

.support-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--dt-space-3);
  flex: 0 0 auto;
  margin-top: var(--dt-space-3);
}

.support-actions--hero {
  margin-top: var(--dt-space-4);
}

@media (max-width: 1200px) {
  .support-workspace,
  .support-highlight-grid,
  .support-entry-grid {
    grid-template-columns: 1fr;
  }

  .support-side {
    grid-template-rows: minmax(280px, auto) minmax(360px, auto);
  }

  .support-hero {
    flex-direction: column;
  }

  .support-hero__side {
    width: 100%;
  }
}

@media (max-width: 720px) {
  .support-page {
    overflow: auto;
  }

  .support-workspace {
    min-height: auto;
  }

  .support-side {
    display: flex;
  }

  .support-stats {
    grid-template-columns: 1fr;
  }

  .support-hero__copy h2,
  .support-card__header h3,
  .support-status h3,
  .support-faq-block__head h3 {
    font-size: 16px;
  }
}
</style>
