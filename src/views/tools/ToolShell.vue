<template>
  <div class="tool-shell">
    <section class="page-card main-card">
      <div class="section-header">
        <div>
          <span class="section-eyebrow">Operation Panel</span>
          <h2>{{ title }}</h2>
          <p>{{ description }}</p>
        </div>
      </div>
      <div class="card-scroll">
        <slot name="main" />
      </div>
    </section>
    <aside class="page-card side-card">
      <div class="side-card__head">
        <span class="section-eyebrow">Inspector</span>
        <h3>辅助操作区</h3>
      </div>
      <div class="card-scroll">
        <slot name="side" />
      </div>
    </aside>
  </div>
</template>

<script setup>
defineProps({
  title: String,
  description: String,
})
</script>

<style scoped lang="scss">
.tool-shell {
  display: grid;
  grid-template-columns: 1.6fr 0.8fr;
  gap: 18px;
  height: 100%;
  min-height: 0;
}

.page-card {
  padding: 22px;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.main-card {
  background: linear-gradient(180deg, #ffffff, #f8fbfe);
}

.side-card {
  background: linear-gradient(180deg, #fdfefe, #f6f9fd);
}

.section-header h2 {
  margin: 0 0 8px;
  font-size: 24px;
}

.section-header p {
  margin: 0 0 20px;
  max-width: 760px;
  color: #6e7d94;
  line-height: 1.7;
}

.side-card__head {
  margin-bottom: 18px;
}

.side-card__head h3 {
  margin: 0;
  font-size: 16px;
}

.card-scroll {
  flex: 1;
  min-height: 0;
  overflow: auto;
  padding-right: 4px;
}

.log-panel {
  min-height: 480px;
  border-radius: var(--dt-radius-subtle);
  background: linear-gradient(180deg, #0f1828, #162235);
  border: 1px solid rgba(99, 126, 175, 0.24);
  padding: 16px;
  color: #d9ebff;
}

.log-line {
  display: grid;
  grid-template-columns: 90px 44px 1fr;
  gap: 10px;
  padding: 8px 0;
  font-size: 13px;
}

@media (max-width: 1080px) {
  .tool-shell {
    grid-template-columns: 1fr;
    height: auto;
  }

  .card-scroll {
    overflow: visible;
    padding-right: 0;
  }
}
</style>
