// iOS 系统媒体会话：锁屏 / 控制中心「正在播放」+ 远程控制
//
// souvlaki 只覆盖 Windows SMTC / Linux MPRIS，iOS 侧由 Objective-C++
// （gen/apple/Sources/neri-player-ios/NowPlaying.mm）实现，这里做 Rust 封装。

use std::sync::OnceLock;

/// 与 NowPlaying.mm 的 NP_CMD_* 对齐
pub const CMD_PLAY: i32 = 1;
pub const CMD_PAUSE: i32 = 2;
pub const CMD_TOGGLE: i32 = 3;
pub const CMD_NEXT: i32 = 4;
pub const CMD_PREV: i32 = 5;
pub const CMD_SEEK: i32 = 6;
pub const CMD_STOP: i32 = 7;

#[allow(dead_code)]
extern "C" {
    fn neri_now_playing_update(
        title: *const std::os::raw::c_char,
        artist: *const std::os::raw::c_char,
        album: *const std::os::raw::c_char,
        duration_seconds: f64,
        cover_url: *const std::os::raw::c_char,
    );
    fn neri_now_playing_set_playback(
        is_playing: i32,
        position_seconds: f64,
        duration_seconds: f64,
    );
    fn neri_now_playing_clear();
    fn neri_now_playing_set_command_handler(handler: Option<extern "C" fn(i32)>);
    fn neri_now_playing_activate_audio_session();
    fn neri_now_playing_last_seek_seconds() -> f64;
    fn neri_now_playing_set_last_seek_seconds(seconds: f64);
}

/// 全局命令发送端：Objective-C 回调可能来自任意线程，只做一次 send
static ACTION_TX: OnceLock<std::sync::mpsc::Sender<super::media_action::MediaAction>> =
    OnceLock::new();

extern "C" fn command_trampoline(cmd: i32) {
    let Some(tx) = ACTION_TX.get() else { return };
    let action = match cmd {
        CMD_PLAY => super::media_action::MediaAction::Play,
        CMD_PAUSE => super::media_action::MediaAction::Pause,
        CMD_TOGGLE => super::media_action::MediaAction::Toggle,
        CMD_NEXT => super::media_action::MediaAction::Next,
        CMD_PREV => super::media_action::MediaAction::Previous,
        CMD_STOP => super::media_action::MediaAction::Pause,
        CMD_SEEK => {
            // seek 位置已由 NowPlaying.mm 存在静态变量里，这里读出后转成比例
            let secs = unsafe { neri_now_playing_last_seek_seconds() };
            let pos_ms = (secs.max(0.0) * 1000.0) as u64;
            super::media_action::MediaAction::SeekTo(pos_ms)
        }
        _ => return,
    };
    let _ = tx.send(action);
}

pub struct IosNowPlaying;

fn cstring(value: &str) -> std::ffi::CString {
    // 内部 NUL 会让 CString::new 失败；替换掉再传
    let cleaned: String = value.chars().filter(|c| *c != '\0').collect();
    std::ffi::CString::new(cleaned).unwrap_or_default()
}

impl IosNowPlaying {
    /// 注册远程控制回调并激活后台音频会话
    pub fn new(
        action_tx: std::sync::mpsc::Sender<super::media_action::MediaAction>,
    ) -> Option<Self> {
        let _ = ACTION_TX.set(action_tx);
        unsafe {
            neri_now_playing_activate_audio_session();
            neri_now_playing_set_command_handler(Some(command_trampoline));
        }
        log::info!(target: "media_session", "iOS Now Playing 已注册");
        Some(Self)
    }

    pub fn update_metadata(
        &self,
        title: &str,
        artist: &str,
        album: &str,
        cover_url: Option<&str>,
        duration_ms: u64,
    ) {
        let t = cstring(title);
        let a = cstring(artist);
        let al = cstring(album);
        let cover = cover_url.map(cstring);
        unsafe {
            neri_now_playing_update(
                t.as_ptr(),
                a.as_ptr(),
                al.as_ptr(),
                duration_ms as f64 / 1000.0,
                cover.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            );
        }
    }

    pub fn update_playback(&self, is_playing: bool, position_ms: u64) {
        // 时长由最近一次 update_metadata 写入 nowPlayingInfo，这里传 0 表示沿用
        unsafe {
            neri_now_playing_set_playback(
                i32::from(is_playing),
                position_ms as f64 / 1000.0,
                0.0,
            );
        }
    }

    pub fn stop(&self) {
        unsafe { neri_now_playing_clear() };
    }
}
