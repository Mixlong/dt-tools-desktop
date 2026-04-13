<template>
  <div class="wiring-page">
    <div class="grid-two">
      <div class="panel panel-card image-card">
        <div class="card-scroll">
          <span class="section-eyebrow">{{ t("tools.wiring.page.diagram") }}</span>
          <h3>{{ t("tools.wiring.page.title") }}</h3>
          <img src="/logo.png" alt="wiring guide" />
          <p>{{ t("tools.wiring.page.description") }}</p>
        </div>
      </div>
      <div class="panel panel-card">
        <div class="card-scroll">
          <span class="section-eyebrow">{{ t("tools.wiring.page.procedure") }}</span>
          <h3>{{ t("tools.wiring.page.stepsTitle") }}</h3>
          <ol class="steps">
            <li v-for="item in steps" :key="item">{{ item }}</li>
          </ol>
          <h4>{{ t("tools.wiring.page.notesTitle") }}</h4>
          <ul class="clean-list">
            <li v-for="item in notes" :key="item">{{ item }}</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from "vue"
import { useI18n } from "vue-i18n"

const { t, tm } = useI18n()
const steps = computed(() => tm("tools.wiring.page.steps"))
const notes = computed(() => tm("tools.wiring.page.notes"))
</script>

<style scoped lang="scss">
.wiring-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}

.panel-card {
  padding: 22px;
  min-height: 0;
}

.card-scroll {
  height: 100%;
  min-height: 0;
  overflow: auto;
  padding-right: 4px;
}

.page-card h3,
.page-card h4 {
  margin: 0 0 12px;
}

.image-card img {
  width: 100%;
  max-width: 260px;
  display: block;
  margin: 30px auto;
}

.image-card p,
.steps,
.clean-list {
  line-height: 1.9;
  color: var(--dt-text-secondary);
}

@media (max-width: 960px) {
  .grid-two {
    grid-template-columns: 1fr;
  }

  .card-scroll {
    overflow: visible;
    padding-right: 0;
  }
}
</style>
