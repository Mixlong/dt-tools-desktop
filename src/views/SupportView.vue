<template>
  <div class="support-page">
    <section class="page-intro">
      <div>
        <span class="section-eyebrow">Support Center</span>
        <h2>使用说明与支持</h2>
        <p>把初始操作规范和常见问题放在工作台里，现场人员不需要离开工具就能完成基础排障。</p>
      </div>
    </section>

    <div class="grid-two">
      <div class="page-card">
        <div class="card-scroll">
          <span class="section-eyebrow">Usage Guide</span>
          <h3>使用说明</h3>
          <ul class="clean-list">
            <li>先在左侧菜单下方选择通用机型，再打开设备入口配置通信方式。</li>
            <li>接线完成后进入抓包、配置或升级流程。</li>
            <li>升级失败时使用内置手动下载兜底。</li>
          </ul>
        </div>
      </div>
      <div class="page-card">
        <div class="card-scroll">
          <span class="section-eyebrow">FAQ</span>
          <h3>FAQ</h3>
          <ul class="clean-list">
            <li>Q: 为什么没有设备数据？ A: 先检查接线与连接状态。</li>
            <li>Q: 为什么更新失败？ A: 检查 updater 目录和签名配置。</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.support-page {
  display: flex;
  flex-direction: column;
  gap: 18px;
  height: 100%;
  min-height: 0;
}

.grid-two {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 18px;
  flex: 1;
  min-height: 0;
}

.page-card {
  padding: 22px;
  min-height: 0;
}

.card-scroll {
  height: 100%;
  min-height: 0;
  overflow: auto;
  padding-right: 4px;
}

.page-card h3 {
  margin: 0 0 12px;
}

.clean-list {
  line-height: 2;
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
