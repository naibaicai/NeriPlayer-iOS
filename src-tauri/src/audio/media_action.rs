// 媒体会话的公共类型：所有平台共用，不受 souvlaki 可用性影响。
//
// 拆分原因：MediaAction 原本定义在 media_session.rs 里，而后者在 iOS 上
// 被 cfg 整段编译掉（souvlaki 不支持 iOS），iOS 的 Now Playing 实现
// 因此无法引用该类型。类型本身与平台无关，放在这里三端共用。

/// 媒体事件回调类型
#[derive(Debug, Clone)]
pub enum MediaAction {
    Play,
    Pause,
    Toggle,
    Next,
    Previous,
    SeekTo(u64), // position_ms
}
