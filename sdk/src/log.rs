use enumflags2::BitFlags;
use crate::WideStringPtr;

/// ログの種類
#[cfg_attr(test, derive(Debug))]
pub enum LogKind {
    /// 情報
    Information, 
    /// 警告
    Warning,     
    /// エラー
    Error,       
}

/// ログ取得のフラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum GetLogFlag {
    /// シリアルナンバーから取得
    BySerial = 0x00000001,
}

/// ログ取得の情報
/// Index は現在保持されているログの中でのインデックスを、
/// Serial は起動時からの連番を表します。
#[repr(C)]
pub struct GetLogInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ
    pub flags: BitFlags<GetLogFlag>,
    /// ログのインデックス
    pub index: u32,
    /// ログのシリアルナンバー
    pub serial: u32,
    /// 取得する文字列
    pub text: WideStringPtr,
    /// 文字列の最大長
    pub max_text: u32,
    /// ログの種類
    pub kind: LogKind,
}
