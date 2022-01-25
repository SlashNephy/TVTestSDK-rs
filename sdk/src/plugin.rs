use std::ffi::c_void;
use enumflags2::{bitflags, BitFlags};
use windows::Win32::Foundation::HWND;
use crate::ClientData;
use crate::message::MessageCallbackFunc;
use crate::win32::WideStringPtr;

/// プラグインの種類
#[repr(u32)]
#[cfg_attr(test, derive(Debug))]
pub enum PluginKind {
    /// 普通
    Normal,
}

/// プラグインのフラグ
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum PluginFlag {
    /// 設定ダイアログがある
    HasSettings       = 0x00000001,
    /// デフォルトで有効
    EnableDefault     = 0x00000002,
    /// 起動時は必ず無効
    DisableOnStart    = 0x00000004,
    /// 終了時以外アンロード不可
    NoUnload          = 0x00000008,
    /// 有効/無効の区別がない
    /// メニューに表示されず、Event::PluginEnable が送られません
    NoEnabledDisabled = 0x00000010,
}

/// プラグインの情報
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct PluginInfo
{
    /// 種類
    pub kind: PluginKind,
    /// フラグ
    pub flags: BitFlags<PluginFlag>,
    /// プラグイン名
    pub name: WideStringPtr,
    /// 著作権情報
    pub copyright: WideStringPtr,
    /// 説明文
    pub description: WideStringPtr,
}

type InternalData = *mut c_void;

/// プラグインパラメータ
#[repr(C)]
pub struct PluginParam
{
    /// コールバック関数
    pub callback: MessageCallbackFunc,
    /// メインウィンドウのハンドル
    pub hwnd_app: HWND,
    /// プラグイン側で好きに使えるデータ
    pub client_data: ClientData,
    /// TVTest側で使用するデータ。アクセス禁止
    internal_data: InternalData,
}
