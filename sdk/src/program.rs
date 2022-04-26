use windows::Win32::Foundation::SYSTEMTIME;
use crate::WideStringPtr;

/// 番組の情報
#[repr(C)]
pub struct ProgramInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// サービスID
    pub service_id: u16,
    /// イベントID
    pub event_id: u16,
    /// イベント名
    pub event_name: WideStringPtr,
    /// イベント名の最大長
    pub max_event_name: i32,
    /// イベントテキスト
    pub event_text: WideStringPtr,
    /// イベントテキストの最大長
    pub max_event_text: i32,
    /// 追加イベントテキスト
    pub event_ext_text: WideStringPtr,
    /// 追加イベントテキストの最大長
    pub max_event_ext_text: i32,
    /// 開始日時(EPG 日時 : UTC+9)
    pub start_time: SYSTEMTIME,
    /// 長さ(秒単位)
    pub duration: u32,
}
