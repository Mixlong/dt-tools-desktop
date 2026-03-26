<template>
  <div class="wiring-page">
    <div class="grid-two">
      <div class="page-card image-card">
        <div class="card-scroll">
          <span class="section-eyebrow">Diagram</span>
          <h3>接线流程图</h3>
          <img src="/logo.png" alt="wiring guide" />
          <p>首版先按型号和通信方式展示静态流程图，后续替换为真实接线素材。</p>
        </div>
      </div>
      <div class="page-card">
        <div class="card-scroll">
          <span class="section-eyebrow">Procedure</span>
          <h3>步骤说明</h3>
          <ol class="steps">
            <li>确认当前型号与通信方式。</li>
            <li>按照接线图依次连接电源、控制器、调试线束。</li>
            <li>连接完成后在左侧设备入口选择正确端口。</li>
            <li>确认在线状态后再进入抓包、配置或升级页面。</li>
          </ol>
          <h4>注意事项</h4>
          <ul class="clean-list">
            <li>通电前再次确认正负极。</li>
            <li>CAN 模式下确认波特率与帧类型一致。</li>
            <li>异常发热或异响时立即断开连接。</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.wiring-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}

.grid-two {
  display: grid;
  grid-template-columns: 1.1fr 0.9fr;
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
