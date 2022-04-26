use std::ffi::c_void;
use crate::WideStringPtr;

/// 設定の値の型
#[repr(u32)]
pub enum SettingKind {
    /// 未定義
    Undefined,
    /// int
    Int,
    /// unsigned int
    UnsignedInt,
    /// 文字列
    String,
    /// データ
    Data,
}

#[repr(C)]
pub union SettingValue {
    /// int
    pub int: i32,
    /// unsigned int
    pub uint: u32,
    /// 文字列
    pub string: WideStringPtr,
    /// データ
    pub data: *const c_void,
}

/// 設定の情報
pub struct SettingInfo {
    /// 設定名
    pub name: WideStringPtr,
    /// 値の型
    pub kind: SettingKind,
    pub value: SettingValue,
    // 値のサイズ (バイト単位)
    pub value_size: u32,
}
