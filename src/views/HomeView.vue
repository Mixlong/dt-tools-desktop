<template>
  <div class="home-page">
    <iframe class="site-frame" :src="siteUrl" title="DT-Tools Official Website" />
  </div>
</template>

<script setup>
const siteUrl = "https://digiwise-ebike.com/pc"
</script>

<style scoped lang="scss">
.home-page {
  display: flex;
  width: 100%;
  height: 100%;
  min-height: 0;
}

.site-frame {
  display: block;
  width: 100%;
  flex: 1;
  min-height: 0;
  height: 100%;
  border: 0;
  background: #fff;
}

@media (max-width: 960px) {
  .home-page {
    height: 100%;
  }
}
</style>
