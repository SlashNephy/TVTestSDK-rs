use std::ffi::c_void;
use enumflags2::BitFlags;
use windows::Win32::Foundation::HWND;
use crate::{ClientData, WideStringPtr};

/// 画像のボタンの位置(画像が無い場合は無視される)
pub struct ControllerButtonRect {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
}

/// 画像の選択ボタンの位置(画像が無い場合は無視される)
pub struct ControllerSelectButtonPosition {
    pub left: u16,
    pub top: u16,
}

/// コントローラのボタンの情報
pub struct ControllerButtonInfo {
    /// ボタンの名称("音声切替" など)
    pub name: WideStringPtr,
    /// デフォルトのコマンド(MsgDoCommand と同じもの)
    /// 指定しない場合はnullptr
    pub default_command: WideStringPtr,
    pub button_rect: ControllerButtonRect,
    pub select_button_position: ControllerSelectButtonPosition,
    /// 予約領域(0にしてください)
    pub reserved: u32,
}

// コントローラのフラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ControllerFlag {
    /// アクティブ時のみ使用できる
    ActiveOnly = 0x00000001,
}

pub type TranslateMessageCallback = unsafe extern "system" fn(
    hwnd: HWND,
    message: *const c_void,
    client_data: ClientData,
);

/// コントローラの情報
pub struct ControllerInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ
    pub flags: BitFlags<ControllerFlag>,
    /// コントローラ識別名
    pub name: WideStringPtr,
    /// コントローラの名称("HDUSリモコン" など)
    pub text: WideStringPtr,
    /// ボタンの数
    pub num_buttons: u32,
    /// ボタンのリスト
    pub buttons: *const ControllerButtonInfo,
    /// 設定ファイル名(nullptr にすると TVTest の Ini ファイル)
    pub ini_filename: WideStringPtr,
    /// 設定のセクション名
    pub section_name: WideStringPtr,
    /// コントローラの画像の識別子(無い場合は0)
    pub controller_image_id: u32,
    /// 選択ボタン画像の識別子(無い場合は0)
    pub selection_buttons_image_id: u32,
    /// メッセージの変換コールバック(必要無ければ nullptr)
    pub translate_message_callback: *const TranslateMessageCallback,
    /// コールバックに渡すパラメータ
    pub client_data: ClientData,
}

/// コントローラの設定マスク
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ControllerSettingsMask {
    /// Flags が有効
    Flags = 0x00000001,
}

/// コントローラの設定フラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ControllerSettingsFlag {
    /// アクティブ時のみ
    ActiveOnly = 0x00000001,
}

/// コントローラの設定
#[repr(C)]
pub struct ControllerSettings {
    pub mask: BitFlags<ControllerSettingsMask>,
    pub flags: BitFlags<ControllerSettingsFlag>,
}
