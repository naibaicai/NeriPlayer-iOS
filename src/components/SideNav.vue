<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useSettingsStore } from '@/stores/settings'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()
const settings = useSettingsStore()

const navItems = computed(() => {
  const items = [
    { path: '/', section: 'home', icon: 'home', key: 'nav.home' },
    { path: '/explore', section: 'explore', icon: 'explore', key: 'nav.explore' },
    { path: '/library', section: 'library', icon: 'library_music', key: 'nav.library' },
    { path: '/settings', section: 'settings', icon: 'settings', key: 'nav.settings' },
  ]
  if (settings.devModeEnabled) {
    items.push({ path: '/debug', section: 'debug', icon: 'bug_report', key: 'settings.debug' })
  }
  return items
})

// 按路由的归属板块判高亮：歌单/歌手/统计等二级页都标了 section=library，
// 进详情后音乐库 TAB 保持点亮；精确比 path 会在任何二级路由上全部熄灭
const activeSection = computed(() => (route.meta.section as string) ?? route.name)
</script>

<template>
  <nav class="side-nav">
    <div class="nav-list">
      <div
        v-for="item in navItems"
        :key="item.path"
        class="nav-item"
        :class="{ active: activeSection === item.section }"
        role="link"
        tabindex="0"
        :aria-current="activeSection === item.section ? 'page' : undefined"
        @click="router.push(item.path)"
        @keydown.enter="router.push(item.path)"
        @keydown.space.prevent="router.push(item.path)"
      >
        <div class="nav-pill">
          <span class="material-symbols-rounded" :class="{ filled: activeSection === item.section }">
            {{ item.icon }}
          </span>
        </div>
        <span class="nav-label">{{ t(item.key) }}</span>
      </div>
    </div>
  </nav>
</template>

<style scoped lang="scss">
.side-nav {
  width: var(--side-nav-width, 80px);
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  background: var(--md-surface-container-low);
  border-right: 1px solid var(--md-surface-container-high);
  flex-shrink: 0;
  padding: 12px 0;
}

.nav-list {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  padding: 4px 0;
  transition: transform var(--duration-short) var(--ease-standard);

  &:active { transform: scale(0.92); }

  &:hover .nav-pill {
    background: var(--md-surface-container-highest);
  }

  &.active .nav-pill {
    background: var(--md-primary-container);
    color: var(--md-on-primary-container);
  }

  &.active .nav-label {
    font-weight: 700;
    color: var(--md-on-surface);
  }
}

.nav-pill {
  width: 56px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-xl);
  color: var(--md-on-surface-variant);
  transition: background var(--duration-short) var(--ease-standard),
              color var(--duration-short) var(--ease-standard);
}

.nav-label {
  font-size: 11px;
  font-weight: 500;
  color: var(--md-on-surface-variant);
  transition: all var(--duration-short);
}

/* 竖屏 / 窄窗口：左侧竖排导航占宽太大，改为贴底横向标签栏。
   宽度交给 App.vue 的 --side-nav-width=0，这里只负责变成横条。
   ★ 必须放在所有基础规则之后：同优先级下源码顺序决定胜负，
     若置于基础规则之前，下列覆盖会被 .nav-list/.nav-item 等基础规则逐条反杀。 */
@media (max-width: 640px), (orientation: landscape) and (max-height: 500px) {
  .side-nav {
    width: 100%;
    height: var(--side-nav-height, 64px);
    flex-direction: row;
    justify-content: space-around;
    border-right: none;
    border-top: 1px solid var(--md-surface-container-high);
    padding: 4px 0 calc(4px + env(safe-area-inset-bottom, 0px));
  }

  .nav-list {
    flex-direction: row;
    justify-content: space-around;
    width: 100%;
    gap: 0;
  }

  .nav-item {
    flex: 1;
    min-width: 0;
  }

  .nav-pill {
    width: 48px;
  }

  .nav-label {
    font-size: 10px;
  }
}
</style>
