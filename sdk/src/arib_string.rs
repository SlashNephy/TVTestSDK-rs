use std::ffi::c_void;
use crate::WideStringPtr;

#[repr(u32)]
pub enum AribStringDecodeFlag {
    Default,
}

/// ARIB文字列のデコード情報
#[repr(C)]
pub struct AribStringDecodeInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// フラグ(現在は常に0)
    pub flags: AribStringDecodeFlag,
    /// 変換元データ
    pub source_data: *const c_void,
    /// 変換元サイズ(バイト単位)
    pub source_length: u32,
    /// 変換先バッファ
    pub destination: WideStringPtr,
    /// 変換先バッファのサイズ(文字単位)
    pub destination_length: u32,
}
