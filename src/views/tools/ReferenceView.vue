<template>
  <div class="reference-page">
    <section class="page-intro">
      <div>
        <span class="section-eyebrow">Command Reference</span>
        <h2>参数参考</h2>
        <p>把常用指令和现场排障要点集中到同一页面，避免在升级和配置过程中切换文档窗口。</p>
      </div>
    </section>

    <div class="grid-two">
      <div class="page-card">
        <div class="card-scroll">
          <span class="section-eyebrow">Quick Commands</span>
          <h3>常用指令</h3>
          <ul class="clean-list">
            <li>读取参数：`AA 55 10 01`</li>
            <li>写入参数：`AA 55 11 01`</li>
            <li>心跳命令：`AA 55 01 00`</li>
          </ul>
        </div>
      </div>
      <div class="page-card">
        <div class="card-scroll">
          <span class="section-eyebrow">Troubleshooting</span>
          <h3>排障要点</h3>
          <ul class="clean-list">
            <li>确认型号和通信方式一致</li>
            <li>连接前检查接线与串口号</li>
            <li>升级前确认设备已稳定在线</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.reference-page {
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
  margin: 0;
  padding-left: 18px;
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
