use enumflags2::BitFlags;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{HBITMAP, HDC};
use crate::WideStringPtr;

/// コマンドの情報
#[repr(C)]
pub struct CommandInfo {
    /// 識別子
    pub id: i32,
    /// コマンドの文字列
    pub text: WideStringPtr,
    /// コマンドの名前
    pub name: WideStringPtr,
}

/// コマンドの情報
#[repr(C)]
pub struct AppCommandInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 取得するコマンドのインデックス
    pub index: u32,
    /// コマンドの文字列
    pub text: WideStringPtr,
    /// コマンドの文字列のバッファ長
    pub max_text: u32,
    /// コマンドの名前
    pub name: WideStringPtr,
    /// コマンドの名前のバッファ長
    pub max_name: u32,
}

/// プラグインのコマンドのフラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PluginCommandFlag {
    /// アイコン表示(サイドバーなどに表示される)
    Iconize        = 0x00000001,
    /// アイコン描画の通知(EVENT_DRAWCOMMANDICON で描画を行う)
    NotifyDrawIcon = 0x00000002,
}

/// プラグインコマンドの状態フラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PluginCommandState {
    /// 無効
    Disabled = 0x00000001,
    /// チェック
    Checked  = 0x00000002,
}

/// プラグインのコマンドの情報
/// CommandInfo が拡張されたものです。
#[repr(C)]
pub struct PluginCommandInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ
    pub flags: BitFlags<PluginCommandFlag>,
    /// 状態フラグ
    pub state: BitFlags<PluginCommandState>,
    /// 識別子
    pub id: i32,
    /// コマンドの文字列
    pub text: WideStringPtr,
    /// コマンドの名前
    pub name: WideStringPtr,
    /// コマンドの説明
    pub description: WideStringPtr,
    /// アイコン
    pub hbm_icon: HBITMAP,
}

/// プラグインコマンドの通知の種類
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PluginCommandNotify {
    /// アイコンを再描画する
    ChangeIcon = 0x00000001,
}

/// コマンドアイコンの状態フラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
pub enum CommandIconState {
    /// 無効状態
    Disabled = 0x0001,
    /// チェック状態
    Checked  = 0x0002,
    /// フォーカスが当たっている
    Hot      = 0x0004,
}

#[repr(C)]
pub struct DrawCommandIconInfo {
    pub id: i32,
    pub flags: u16,
    pub state: BitFlags<CommandIconState>,
    pub style: WideStringPtr,
    pub hdc: HDC,
    pub draw_rect: RECT,
    pub color: u32,
    pub opacity: u8,
    pub reserved: [u8; 3],
}
