#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppStatus {
    Connecting,
    Connected,
    SyncingTime,
    Ready
}