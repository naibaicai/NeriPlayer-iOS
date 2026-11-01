pub mod analyzer;
pub mod buffered;
pub mod decoder;
pub mod effects;
pub mod ffmpeg;
pub mod growing;
pub mod hls;
pub mod media_action;
// 系统媒体会话（SMTC / MPRIS / Now Playing）依赖 souvlaki，后者仅覆盖桌面平台
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub mod media_session;
// iOS 的 Now Playing / 远程控制由 Objective-C++（MPNowPlayingInfoCenter）实现
#[cfg(target_os = "ios")]
pub mod now_playing_ios;
pub mod metrics;
pub mod pcm;
pub mod player;
pub mod queue;
pub mod remote;
pub mod stretch;
pub mod dsp;
pub mod dsp_settings;
