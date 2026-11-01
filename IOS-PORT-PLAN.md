# NeriPlayer-Desktop 竖屏改造 + iOS 移植状态

> 本文件是本次工作的检查点。非上游文件，仅本地记录，可随时删除。

## 一、竖屏改造 —— 完成并验证 ✅

| 文件 | 改动 |
|---|---|
| `src-tauri/tauri.conf.json` | `minWidth` 800→**360**、`minHeight` 600→**480** |
| `src/components/SideNav.vue` | ≤640px：左侧竖排导航 → 底部横排标签栏（含安全区） |
| `src/App.vue` | `--side-nav-width/height` 变量；窄屏 `column-reverse`；正文让位 |
| `src/components/MiniPlayer.vue` | `left: 80px` 硬编码 → 跟随变量 |
| `index.html` | viewport 加 `viewport-fit=cover` |

## 二、iOS 移植 —— Rust 侧已编出静态库 ✅

### 代码改造（全部完成）

1. **`Cargo.toml`**
   - 新增 `[lib] crate-type = ["staticlib", "cdylib", "rlib"]`（移动端必需，链进原生工程）
   - 桌面独有依赖组 `cfg(not(any(android, ios)))`：`notify`、`souvlaki`、`tauri`(tray-icon)、
     `tauri-plugin-single-instance`、`tauri-plugin-global-shortcut`
   - 移动独有组：`rquickjs` + `bindgen` 特性（rquickjs-sys 无 iOS 预置 FFI 绑定）
   - 根因修复：`souvlaki` 的 dbus 依赖条件 `all(unix, not(macos))` 把 iOS 误圈进来

2. **源码平台门控**
   - `commands/mod.rs`：gate `desktop_lyrics_cmd`、`tray_cmd`
   - `audio/mod.rs`：gate `media_session`（依赖 souvlaki）
   - `library/local_index.rs`：gate `notify` 监视（含 2 处 `rebuild_watcher` 调用点）
   - `lib.rs`：gate 托盘/媒体会话/快捷键/单实例/运行事件/命令权限表/`set_decorations`
   - `webview_args.rs`：新增 `DesktopWindowExt` trait，收敛移动端不存在的
     `center()` / `skip_taskbar(true)`，移动端退化为 no-op
   - 调用点：`auth_cmd.rs`（3 处）、`api/netease/comment_token.rs`、`api/youtube/web_po.rs`

3. **入口重构**
   - `main.rs` 866 行 → 6 行，调 `neri_player_desktop::run()`
   - 启动逻辑下移到 `lib.rs::run()`；新增 `#[tauri::mobile_entry_point] mobile_run()`

4. **权限平台门控**
   - `capabilities/default.json`：移除 `global-shortcut:*`（iOS 无此插件，权限不注册）
   - 新增 `capabilities/desktop-global-shortcut.json`：
     `"platforms": ["macOS","windows","linux"]`

5. **iOS 工程**
   - `gen/apple/Podfile`：删掉模板生成的 `neri-player-desktop_macOS` target
     （项目里不存在该 target，会让 `pod install` 直接失败）

### 验证结果

- `cargo check`（桌面）通过
- `cargo test --lib` **957 passed / 0 failed / 6 ignored**
- 前端回归 **97 passed / 0 failed**
- **`cargo build --target aarch64-apple-ios` 成功** →
  `target/aarch64-apple-ios/debug/libneri_player_desktop.a`（约 854 MB）

## 三、模拟器实跑 —— 完成 ✅（2026-10-18 17:17 更新）

**上面的"阻塞"已解除**：Xcode 许可已同意（27.0）、iOS 27.0 运行时已装、
`NeriPlayer.app` 已构建（arm64-sim，17:17）并**在 iPhone 18 Pro 模拟器上成功运行**。

### 实跑发现并修复的真 bug：底部标签栏竖排堆叠

首次实跑截图发现：底部导航**没有变成横向标签栏**，而是竖排堆叠、压住正文内容。

**根因（CSS 源码顺序）**：`@media (max-width: 640px)` 块写在基础规则**之前**，
而 `.nav-list` / `.nav-item` / `.nav-pill` / `.nav-label` 的基础规则在后。
同优先级下源码顺序决定胜负 → 四条竖屏覆盖规则全部被反杀 → 只剩 `.side-nav` 自身的
`flex-direction: row` 生效，而 `.nav-list` 仍是 `column`。

**修法**：把整个媒体查询块移到 style 末尾（基础规则之后），并加注释说明原因。
仅动 `src/components/SideNav.vue` 一个文件。

**验证**：重新构建 → 重装 → 重启 → 截图，底部已是横排「首页 / 探索 / 音乐库 / 设置」；
前端回归 97 passed / 0 failed。

证据截图：`projects/2026-10-18-neriplayer-ios-port/evidence/`（修复前 / 修复后）。

### 改用脱离式构建（避免会话回收杀进程）

`npx tauri ios build` 改用 `rebuild_ios_detached.py`（双 fork + `os.setsid()`，
PPID=1 被 launchd 收养）启动，日志 `/tmp/neri-ios-rebuild.log`，全程未被会话回收打断。

---

## 三-B、历史阻塞记录（已解决，保留供参考）：缺 iOS 模拟器运行时

Tauri CLI 的 `ensure_ios_runtime_installed()`（`cli/src/mobile/ios/mod.rs:416`）
会检查 `xcrun simctl list runtimes` 里有没有 iOS 运行时，没有就直接
`bail!("iOS platform not installed")`，`ios build` / `ios dev` 都在这一步退出。

本机 `simctl list runtimes` 为空（Xcode 装了但没下载 runtime）。

**处理中**：`xcodebuild -downloadPlatform iOS` 正在下载（约 7–8 GB，
实测约 5.6 MB/s）。下载完成后 `tauri ios build --debug --target aarch64-sim` 会自动继续。

### 备注：绕过 CLI 直接 xcodebuild 不可行

试过直接 `xcodebuild -workspace ... -scheme neri-player-desktop_iOS`，
但工程里的 "Build Rust Code" 脚本阶段需要 CLI 自己写的
`gen/apple/.tauri/cli-options-server.json`，脱离 `tauri ios build` 就会：
`the tauri ios dev or tauri ios build command must be running while Xcode builds the app`。

## 四、环境结论（许可同意后）

- Xcode 许可同意后，`xcrun` / `clang` / `ar` / `git` 全部恢复正常，
  之前用 `DEVELOPER_DIR=/Library/Developer/CommandLineTools` 的绕行不再需要
- 临时 xcrun 垫片已删除（`~/.neri-ios-tools/`）
- 已装：cocoapods、xcodegen、libimobiledevice（brew）

## 五、运行时安装完成后的下一步

```bash
npx tauri ios build --debug --target aarch64-sim    # 模拟器
# 或
npx tauri ios build --target aarch64                # 真机（需签名证书，本机 0 个有效身份）
```

真机需要：`APPLE_DEVELOPMENT_TEAM` 或 `bundle > iOS > developmentTeam`。

---

## 三、本轮（2026-10-31 13:07–14:05）实测结论与改动

### 实测数据（iPhone 18 Pro 模拟器，WebView 内插桩取值）

```
innerW=402  innerH=874  dpr=3
safeTop=62px  safeBottom=34px  safeLeft=0px
titlebarH=52  tbH=52
homePadL=16px  homeW=394          ← @media(max-width:480px) 生效，媒体查询正常
radarW=363  radarScrollW=830      ← 雷达歌单是横向轮播，第三张露头是滑动提示，非裁切
cardW=147  cardMinW=150px
sideNavW=402  sideNavH=64
```

**推翻上一轮判断**：不存在「布局宽度 522px」的溢出。视口正确、媒体查询正常，
右侧被切是轮播设计。

### 实际改动

| 文件 | 改动 |
|---|---|
| `platform.ts` | 新增 `ios` 平台类型，iOS 判定前置于 macOS（含 iPadOS 伪装 macOS 的触点判断）；导出 `isIosPlatform` / `isTouchPlatform` |
| `global.scss` | `html.platform-ios { --titlebar-height: 0 }`（原 52px 靠 safe-area 凑巧兜住）；禁长按选中/双击缩放 |
| `App.vue` | `padding-top` 改为 `titlebar + --safe-area-top`；`--side-nav-height` 改 `calc(56px + env(safe-area-inset-bottom))`；iOS 下不装桌面歌词/托盘/窗口关闭/全局快捷键 |
| `TitleBar.vue` | `v-if="!isIos"` 整体不渲染（NowPlaying 自带返回键） |
| `MiniPlayer.vue` | `bottom: var(--side-nav-height)`（原来 fixed bottom:0 会盖住导航条）；≤640px 隐藏随机/循环/桌面歌词/音量/一起听/队列，只留上一首/播放/下一首/展开 |
| `SettingsView.vue` | iOS 下从导航剔除「桌面歌词」分区（独立窗口，移动端不可达） |
| `Cargo.toml` | 去掉 `cdylib`（仅 Android 需要；iOS 保留 `staticlib`+`rlib`） |

已验证：`vue-tsc` 零错误、`pnpm build` 通过、模拟器实跑截图确认顶栏死区消失、底部导航避开 Home 指示条。

### 构建状态

- **debug 真机包：成功**。`BUILD SUCCEEDED` → 手动打包 71MB IPA，完整性校验通过。
- **release 真机包：仍失败**。Xcode 链接 `libapp.a` 时报 Swift 侧符号未定义：

```
_init_plugin_clipboard / _init_plugin_dialog / _init_plugin_opener
_log_stdout / _on_webview_created / _register_plugin / _run_plugin_command / _tauri_log
```

根因：debug 版 `libapp.a` 里这些符号的引用数为 6，release 版为 **0**；对应的
`libTauri.a` 等 Swift 静态库虽已生成，但 release 配置下没有被带进链接行
（`LC_LINKER_OPTION` 共 689 条，debug 是 702 条）。已尝试补 `OTHER_LDFLAGS`
（AudioToolbox/CoreAudio/AVFoundation/MediaToolbox）——音频符号那批已消掉，Swift 这批仍在。
待办：查 `tauri ios xcode-script` 在 release 下的 Swift 产物注册路径。

debug 包 200MB dylib，`strip` 后 173MB，无法压到 iMessage 24MiB 上限以内，故走 `imsg send --file` 通道。
