// 字体工具：从已删除的 desktopLyrics/style 里抽出，应用字体设置与字体选择器共用。

export const FONT_FAMILY_PRESETS = [
  'MiSans', 'Microsoft YaHei', 'PingFang SC', 'Noto Sans SC', 'Source Han Sans SC',
  'SimHei', 'KaiTi', 'SimSun', 'Inter', 'Segoe UI',
] as const

/// 只保留字母/数字/空格与 . _ -，限长 64，避免把任意 CSS 注入 font-family
export function sanitizeFontFamily(value: unknown): string {
  if (typeof value !== 'string') return ''
  const family = value.replace(/[^\p{L}\p{N} ._-]/gu, '').replace(/\s+/g, ' ').trim()
  return Array.from(family).slice(0, 64).join('')
}

export function fontFamilyStack(value: string): string {
  const family = sanitizeFontFamily(value)
  return family ? `"${family}", var(--font-family-default)` : 'var(--font-family-default)'
}

export function applyFontSettings(
  appFontFamily: string,
  lyricFontFamily: string,
  root: Pick<HTMLElement, 'style'> = document.documentElement,
): void {
  const appStack = fontFamilyStack(appFontFamily)
  root.style.setProperty('--font-family', appStack)
  root.style.setProperty('--lyric-font-family', fontFamilyStack(lyricFontFamily))
  for (const key of ['--font-display', '--font-hand']) {
    if (sanitizeFontFamily(appFontFamily)) root.style.setProperty(key, appStack)
    else root.style.removeProperty(key)
  }
}
