use std::ffi::c_void;
use windows::Win32::Foundation::{HWND, PWSTR};
use crate::message::MessageCallbackFunc;

/// プラグインの種類
#[repr(u32)]
#[cfg_attr(test, derive(Debug))]
pub enum PluginType {
    Normal  // 普通
}

/// プラグインのフラグ
#[repr(u32)]
#[cfg_attr(test, derive(Debug))]
pub enum PluginFlag {
    Normal = 0x00000000,
    HasSettings = 0x00000001,       // 設定ダイアログがある
    EnableDefault = 0x00000002,     // デフォルトで有効
    DisableOnStart = 0x00000004,    // 起動時は必ず無効
    NoUnload = 0x00000008,          // 終了時以外アンロード不可
    NoEnabledDisabled = 0x00000010, // 有効/無効の区別がない, メニューに表示されず、Event::PluginEnable が送られません
}

/// プラグインの情報
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct PluginInfo
{
    // 種類 (PluginType)
    pub types: PluginType,
    // フラグ (PluginFlag)
    pub flags: PluginFlag,
    // プラグイン名
    pub name: PWSTR,
    // 著作権情報
    pub copyright: PWSTR,
    // 説明文
    pub description: PWSTR,
}

/// プラグインパラメータ
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct PluginParam
{
    // コールバック関数
    pub callback: MessageCallbackFunc,
    // メインウィンドウのハンドル
    pub hwnd_app: HWND,
    // プラグイン側で好きに使えるデータ
    pub client_data: *mut c_void,
    // TVTest側で使用するデータ。アクセス禁止
    internal_data: *mut c_void,
}
