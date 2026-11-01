//! 所有窗口共用一个 WebView2 环境（同一个用户数据目录），附加浏览器参数必须与主窗口完全一致，
//! 否则之后创建的窗口会因环境参数不同而创建失败。参数只在 tauri.conf.json 的主窗口里写一份
use tauri::{Manager, Runtime, WebviewWindowBuilder};

pub trait MainBrowserArgs<R: Runtime>: Sized {
    /// 套用主窗口配置的附加浏览器参数
    fn main_browser_args(self, manager: &impl Manager<R>) -> Self;
}

impl<'a, R: Runtime, M: Manager<R>> MainBrowserArgs<R> for WebviewWindowBuilder<'a, R, M> {
    fn main_browser_args(self, manager: &impl Manager<R>) -> Self {
        let config = manager.config();
        let args = config
            .app
            .windows
            .iter()
            .find(|window| window.label == "main")
            .or_else(|| config.app.windows.first())
            .and_then(|window| window.additional_browser_args.as_deref());
        match args {
            Some(args) => self.additional_browser_args(args),
            None => self,
        }
    }
}

/// 桌面专属的窗口构建方法。
///
/// `center` / `skip_taskbar` 在 iOS/Android 上不存在（Tauri 按平台裁剪了这部分 API），
/// 而调用点散落在登录窗口、网易云取令牌窗口、YouTube player 窗口里，
/// 逐处加 cfg 会让业务代码难以阅读，所以统一收敛到这个扩展 trait。
/// 移动端退化为 no-op：移动端只有全屏主窗口，「居中」与「不进任务栏」本就没有意义。
pub trait DesktopWindowExt<R: Runtime>: Sized {
    fn desktop_center(self) -> Self;
    /// 隐藏窗口不进任务栏：移动端无此概念，参数保留只为调用处语义统一
    fn desktop_skip_taskbar(self) -> Self;
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
impl<'a, R: Runtime, M: Manager<R>> DesktopWindowExt<R> for WebviewWindowBuilder<'a, R, M> {
    fn desktop_center(self) -> Self {
        self.center()
    }

    fn desktop_skip_taskbar(self) -> Self {
        self.skip_taskbar(true)
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
impl<'a, R: Runtime, M: Manager<R>> DesktopWindowExt<R> for WebviewWindowBuilder<'a, R, M> {
    fn desktop_center(self) -> Self {
        self
    }

    fn desktop_skip_taskbar(self) -> Self {
        self
    }
}
