// ログの種類
#[cfg_attr(test, derive(Debug))]
pub enum LogType {
    Information, // 情報
    Warning,     // 警告
    Error,       // エラー
}
