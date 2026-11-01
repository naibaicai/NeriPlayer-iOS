<script setup lang="ts">
// 评论独立页面：复用 CommentsPanel 的全部逻辑（加载/分页/回复/预览），
// 只把它从「侧边弹层」改为「整页」。移动端没有多窗口，弹层体验差。
import { ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { usePlayerStore } from '@/stores/player'
import CommentsPanel from '@/components/CommentsPanel.vue'

const router = useRouter()
const { t } = useI18n()
const player = usePlayerStore()

const open = ref(true)

function goBack() {
  // 直接进入本页（无历史）时回首页，避免退到站外
  if (window.history.length > 1) router.back()
  else void router.push({ name: 'home' })
}

// 面板关闭（返回键/遮罩/Esc）统一转成路由返回
watch(open, value => {
  if (!value) goBack()
})
</script>

<template>
  <div class="comments-page">
    <header class="comments-page-header">
      <button class="cp-back" :aria-label="t('common.back')" @click="goBack">
        <span class="material-symbols-rounded">arrow_back</span>
      </button>
      <h1 class="cp-title">{{ t('player.comments') }}</h1>
      <span class="cp-spacer" aria-hidden="true" />
    </header>

    <div class="comments-page-body">
      <CommentsPanel v-model:open="open" :track="player.currentTrack" />
    </div>
  </div>
</template>

<style scoped lang="scss">
.comments-page {
  position: fixed;
  inset: 0;
  z-index: 20;
  display: flex;
  flex-direction: column;
  background: var(--md-surface);
}

.comments-page-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: calc(env(safe-area-inset-top, 0px) + 8px) 12px 8px;
  border-bottom: 1px solid var(--md-outline-variant);
  flex-shrink: 0;
}

.cp-back {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-full);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--md-on-surface);
  flex-shrink: 0;

  &:active { background: var(--md-surface-variant); }
}

.cp-title {
  font-size: 16px;
  font-weight: 600;
  flex: 1;
  text-align: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 与返回按钮等宽，让标题真正居中 */
.cp-spacer {
  width: 40px;
  flex-shrink: 0;
}

.comments-page-body {
  flex: 1;
  min-height: 0;
  position: relative;

  /* 面板内部本来是全屏遮罩，这里让位给页面容器 */
  :deep(.comments-overlay) {
    position: absolute;
    inset: 0;
    background: transparent;
    backdrop-filter: none;
    -webkit-backdrop-filter: none;
  }

  :deep(.comments-panel) {
    position: absolute;
    inset: 0;
    width: 100%;
    max-width: none;
    border-radius: 0;
    box-shadow: none;
    padding-bottom: env(safe-area-inset-bottom, 0px);
  }
}
</style>
