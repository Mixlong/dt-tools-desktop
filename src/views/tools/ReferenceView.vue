<template>
  <div class="reference-page">
    <section class="reference-window">
      <header class="window-bar">
        <div class="window-bar__lights">
          <span class="window-bar__light window-bar__light--close" />
          <span class="window-bar__light window-bar__light--min" />
          <span class="window-bar__light window-bar__light--max" />
        </div>
        <div class="window-bar__title">仪表参数</div>
        <div class="window-bar__spacer" />
      </header>

      <div class="window-body">
        <section class="reference-hero">
          <div>
            <p class="reference-hero__eyebrow">Instrument Parameters</p>
            <h2>参数参考</h2>
            <p class="reference-hero__copy">把常用指令、排障条件和现场核对要点集中到一个窗口，避免在升级和配置过程中来回切换文档。</p>
          </div>
        </section>

        <div class="reference-grid">
          <section class="reference-panel">
            <div class="reference-panel__head">
              <h3>常用指令</h3>
              <span>Quick Commands</span>
            </div>

            <div class="command-list">
              <div class="command-item">
                <strong>读取参数</strong>
                <code>AA 55 10 01</code>
                <p>读取当前仪表参数块，适合上电后快速确认基础状态。</p>
              </div>
              <div class="command-item">
                <strong>写入参数</strong>
                <code>AA 55 11 01</code>
                <p>写入单次修改后的参数数据，执行前先确认机型与通信方式。</p>
              </div>
              <div class="command-item">
                <strong>心跳命令</strong>
                <code>AA 55 01 00</code>
                <p>用于维持在线交互和检测接线状态，排障时优先测试这一条。</p>
              </div>
            </div>
          </section>

          <section class="reference-panel reference-panel--muted">
            <div class="reference-panel__head">
              <h3>排障要点</h3>
              <span>Troubleshooting</span>
            </div>

            <ol class="rule-list">
              <li>确认机型、协议和通信方式与当前仪表一致。</li>
              <li>接线完成后先检查供电，再确认串口号或 CAN 配置。</li>
              <li>升级前先用读取参数或心跳命令验证设备已稳定在线。</li>
              <li>出现超时或异常回包时，优先回到接线与端口选择重新核对。</li>
            </ol>

            <div class="note-box">
              <strong>现场提示</strong>
              <p>如果需要连续调试多个参数，建议先记录原始值，再执行写入和校验，避免覆盖后无法回退。</p>
            </div>
          </section>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped lang="scss">
.reference-page {
  display: flex;
  align-items: stretch;
  justify-content: center;
  min-height: 100%;
}

.reference-window {
  display: flex;
  flex-direction: column;
  width: min(1180px, 100%);
  min-height: 100%;
  background: #ffffff;
  border: 1px solid #dbe4ef;
  border-radius: var(--dt-radius-panel);
  box-shadow:
    0 28px 80px rgba(16, 28, 45, 0.08),
    inset 0 1px 0 rgba(255, 255, 255, 0.92);
  overflow: hidden;
}

.window-bar {
  display: grid;
  grid-template-columns: 84px 1fr 84px;
  align-items: center;
  min-height: 42px;
  padding: 0 16px;
  background: linear-gradient(180deg, #fbfcfe, #f5f8fc);
  border-bottom: 1px solid #e6edf5;
}

.window-bar__lights {
  display: flex;
  gap: 8px;
}

.window-bar__light {
  width: 12px;
  height: 12px;
  border-radius: 999px;
}

.window-bar__light--close {
  background: #ff6058;
}

.window-bar__light--min {
  background: #ffbd2e;
}

.window-bar__light--max {
  background: #28c840;
}

.window-bar__title {
  text-align: center;
  color: #49566c;
  font-size: 13px;
  font-weight: 600;
}

.window-body {
  display: flex;
  flex: 1;
  flex-direction: column;
  min-height: 0;
  padding: 28px 30px 30px;
  background:
    radial-gradient(circle at top right, rgba(22, 119, 255, 0.06), transparent 28%),
    linear-gradient(180deg, #ffffff, #f7fafe);
}

.reference-hero {
  padding-bottom: 22px;
  border-bottom: 1px solid #e6edf5;
}

.reference-hero__eyebrow {
  margin: 0 0 10px;
  color: #5f6f88;
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.04em;
}

.reference-hero h2 {
  margin: 0;
  color: #1b3050;
  font-size: 34px;
  font-weight: 700;
  line-height: 1.1;
}

.reference-hero__copy {
  margin: 12px 0 0;
  max-width: 760px;
  color: #64748c;
  font-size: 15px;
  line-height: 1.8;
}

.reference-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.15fr) minmax(320px, 0.85fr);
  gap: 22px;
  padding-top: 24px;
  flex: 1;
  min-height: 0;
}

.reference-panel {
  display: flex;
  flex-direction: column;
  gap: 18px;
  padding: 22px;
  border: 1px solid #dde6f0;
  border-radius: var(--dt-radius-subtle);
  background: linear-gradient(180deg, #ffffff, #f9fbfe);
}

.reference-panel--muted {
  background: linear-gradient(180deg, #fbfcfe, #f5f8fc);
}

.reference-panel__head {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: baseline;
}

.reference-panel__head h3 {
  margin: 0;
  color: #21395f;
  font-size: 22px;
  font-weight: 700;
}

.reference-panel__head span {
  color: #7a879c;
  font-size: 12px;
  font-weight: 600;
}

.command-list {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.command-item {
  padding: 16px 18px;
  border: 1px solid #e2e9f3;
  border-radius: var(--dt-radius-subtle);
  background: #f7fafe;
}

.command-item strong {
  display: block;
  margin-bottom: 10px;
  color: #243c61;
  font-size: 16px;
}

.command-item code {
  display: inline-block;
  margin-bottom: 10px;
  padding: 6px 10px;
  border-radius: 999px;
  background: #e9f2ff;
  color: #1668dc;
  font-size: 13px;
  font-weight: 700;
}

.command-item p {
  margin: 0;
  color: #6b7890;
  font-size: 14px;
  line-height: 1.75;
}

.rule-list {
  margin: 0;
  padding-left: 20px;
  color: #5d6b83;
}

.rule-list li {
  font-size: 15px;
  line-height: 1.9;
}

.rule-list li + li {
  margin-top: 8px;
}

.note-box {
  margin-top: auto;
  padding: 16px 18px;
  border: 1px solid #d7e3f4;
  border-radius: var(--dt-radius-subtle);
  background: #edf4ff;
}

.note-box strong {
  display: block;
  margin-bottom: 8px;
  color: #224a88;
  font-size: 14px;
}

.note-box p {
  margin: 0;
  color: #526780;
  font-size: 14px;
  line-height: 1.75;
}

@media (max-width: 1080px) {
  .reference-window {
    border-radius: var(--dt-radius-panel);
  }

  .window-body {
    padding: 22px 20px 20px;
  }

  .reference-hero h2 {
    font-size: 28px;
  }

  .reference-grid {
    grid-template-columns: 1fr;
  }
}
</style>
