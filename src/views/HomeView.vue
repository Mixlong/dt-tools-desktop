<template>
  <div class="home-page">
    <section class="home-section">
      <div class="section-head">
        <div>
          <span class="section-eyebrow">{{ t("home.hero.mainTools") }}</span>
          <h2>{{ t("home.hero.title") }}</h2>
        </div>
      </div>

      <div class="tool-grid">
        <article
          v-for="tool in toolCards"
          :key="tool.title"
          class="tool-card panel"
        >
          <div class="tool-card__icon" :class="`tool-card__icon--${tool.tone}`">
            <q-icon :name="tool.icon" size="30px" />
          </div>

          <div class="tool-card__body">
            <h3>{{ tool.title }}</h3>
            <p>{{ tool.description }}</p>
          </div>

          <q-btn
            class="home-action-btn home-action-btn--primary home-action-btn--full"
            color="primary"
            unelevated
            no-caps
            :label="t('home.hero.enterTool')"
            @click="handleToolEntry(tool)"
          />
        </article>
      </div>
    </section>

    <section class="home-grid">
      <article class="panel info-card">
        <div class="section-eyebrow">{{ t("home.info.deviceEyebrow") }}</div>
        <h2>{{ t("home.info.deviceTitle") }}</h2>
        <p>{{ t("home.info.deviceDescription") }}</p>

        <div class="info-card__actions">
          <q-btn class="home-action-btn" outline no-caps color="primary" :label="t('home.info.config')" to="/config" />
          <q-btn class="home-action-btn" outline no-caps color="primary" :label="t('home.info.software')" to="/software" />
        </div>
      </article>

      <article class="panel info-card">
        <div class="section-eyebrow">{{ t("home.info.supportEyebrow") }}</div>
        <h2>{{ t("home.info.supportTitle") }}</h2>
        <ul class="info-list">
          <li v-for="item in supportItems" :key="item">{{ item }}</li>
        </ul>

        <q-btn class="home-action-btn self-start" flat no-caps color="primary" :label="t('home.info.supportGuide')" to="/support" />
      </article>
    </section>
  </div>
</template>

<script setup>
import { computed } from "vue"
import { useI18n } from "vue-i18n"
import { useRouter } from "vue-router"
import { isNavRouteAvailable } from "@/config/navigation"
import { notifyInfo } from "@/services/ui"

const { t, tm } = useI18n()
const router = useRouter()

const supportItems = computed(() => tm("home.info.supportItems"))
const toolCards = computed(() => [
  {
    title: t("home.cards.wiringTitle"),
    description: t("home.cards.wiringDescription"),
    icon: "cable",
    to: { path: "/tools", query: { tab: "wiring" } },
    navPath: "/tools",
    tone: "primary",
  },
  {
    title: t("home.cards.serialTitle"),
    description: t("home.cards.serialDescription"),
    icon: "usb",
    to: { path: "/tools", query: { tab: "serial" } },
    navPath: "/tools",
    tone: "positive",
  },
  {
    title: t("home.cards.canTitle"),
    description: t("home.cards.canDescription"),
    icon: "directions_car",
    to: { path: "/tools", query: { tab: "can" } },
    navPath: "/tools",
    tone: "warning",
  },
])

function handleToolEntry(tool) {
  if (!isNavRouteAvailable(tool.navPath)) {
    notifyInfo(t("home.hero.entryUnavailable", { title: t("nav.tools") }))
    return
  }

  router.push(tool.to)
}
</script>

<style scoped lang="scss">
.home-page {
  display: flex;
  flex-direction: column;
  gap: 20px;
  flex: 1;
  min-height: 100%;
  height: 100%;
  padding: 12px;
  box-sizing: border-box;
}

.home-hero {
  display: grid;
  grid-template-columns: minmax(0, 1.45fr) minmax(300px, 0.85fr);
  gap: 20px;
  padding: 24px;
}

.home-hero__content {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.section-eyebrow {
  color: var(--dt-text-secondary);
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}

.home-hero__content h1 {
  margin: 0;
  font-family: var(--dt-font-ui);
  color: var(--dt-text-primary);
  font-size: 22px;
  font-weight: 700;
  line-height: 1.3;
  letter-spacing: 0;
}

.section-head h2,
.info-card h2 {
  margin: 0;
  font-family: var(--dt-font-ui);
  color: var(--dt-text-primary);
  font-size: 18px;
  font-weight: 700;
  line-height: 1.35;
  letter-spacing: 0;
}

.home-hero__content p,
.info-card p,
.info-list {
  margin: 0;
  color: var(--dt-text-secondary);
  line-height: 1.7;
}

.home-hero__actions,
.info-card__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.home-status {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}

.home-status__item {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 16px;
  border: 1px solid var(--dt-border);
  border-radius: var(--dt-radius-subtle);
  background: rgba(255, 255, 255, 0.02);
}

.home-status__item span {
  color: var(--dt-text-secondary);
  font-size: 13px;
}

.home-status__item strong {
  color: var(--dt-text-primary);
  font-size: 15px;
  line-height: 1.5;
  word-break: break-all;
}

.home-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.section-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.tool-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 18px;
}

.tool-card {
  display: flex;
  flex-direction: column;
  gap: 18px;
  padding: 22px;
  min-height: 228px;
}

.home-action-btn {
  min-height: 38px;
  border-radius: var(--dt-radius-button);
  font-size: 13px;
  font-weight: 700;
  padding: 0 18px;
}

.home-action-btn--full {
  width: 100%;
}

.tool-card__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 58px;
  height: 58px;
  border-radius: var(--dt-radius-subtle);
}

.tool-card__icon--primary {
  background: var(--dt-brand-accent-soft);
  color: var(--dt-brand-accent);
}

.tool-card__icon--positive {
  background: var(--dt-status-success-soft);
  color: var(--dt-success);
}

.tool-card__icon--warning {
  background: var(--dt-status-warning-soft);
  color: var(--dt-warning);
}

.tool-card__body {
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex: 1;
}

.tool-card__body h3 {
  margin: 0;
  font-family: var(--dt-font-ui);
  color: var(--dt-text-primary);
  font-size: 16px;
  font-weight: 700;
  line-height: 1.4;
  letter-spacing: 0;
}

.tool-card__body p {
  margin: 0;
  color: var(--dt-text-secondary);
  line-height: 1.7;
}

.home-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 18px;
  flex: 1;
  align-content: start;
}

.info-card {
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 22px;
  min-height: 100%;
}

.info-list {
  padding-left: 18px;
}

.self-start {
  align-self: flex-start;
}

.text-positive {
  color: var(--dt-success);
}

.text-warning {
  color: var(--dt-warning);
}

@media (max-width: 1180px) {
  .home-hero,
  .tool-grid,
  .home-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .home-page {
    padding: 12px;
  }

  .home-hero,
  .tool-card,
  .info-card {
    padding: 18px;
  }

  .home-status {
    grid-template-columns: 1fr;
  }

  .home-hero__content h1,
  .section-head h2,
  .info-card h2 {
    font-size: 18px;
  }

  .home-hero__content h1 {
    font-size: 20px;
  }
}
</style>
