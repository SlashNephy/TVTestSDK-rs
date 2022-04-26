use crate::{Version, WideStringPtr};

/// ホストプログラムの情報
#[repr(C)]
pub struct HostInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// プログラム名 ("TVTest"、"TVH264" など)
    pub app_name: WideStringPtr,
    pub version: Version,
    /// バージョン文字列 ("1.2.0" など)
    pub version_text: WideStringPtr,
    /// 対応しているプラグインのバージョン
    pub supported_plugin_version: u32,
}
