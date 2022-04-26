use enumflags2::BitFlags;
use windows::Win32::Foundation::{POINT, RECT, SYSTEMTIME};
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::UI::WindowsAndMessaging::HMENU;
use crate::WideStringPtr;

/// 番組表の番組の情報
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ProgramGuideProgramInfo {
    /// ネットワークID
    pub network_id: u16,
    /// ストリームID
    pub transport_stream_id: u16,
    /// サービスID
    pub service_id: u16,
    /// イベントID
    pub event_id: u16,
    /// 開始日時(EPG 日時 : UTC+9)
    pub start_time: SYSTEMTIME,
    /// 長さ(秒単位)
    pub duration: u32,
}

/// 番組表の番組の背景描画の情報
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ProgramGuideProgramDrawBackgroundInfo {
    /// 描画先DCハンドル
    pub hdc: HDC,
    /// 項目全体の位置
    pub item_rect: RECT,
    /// タイトルの位置
    pub title_rect: RECT,
    /// 番組内容の位置
    pub content_rect: RECT,
    /// 背景色
    pub background_color: u32,
}

/// 番組表のメニューの情報
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ProgramGuideInitializeMenuInfo {
    /// メニューのハンドル
    pub hmenu: HMENU,
    /// 項目のID
    pub command: u32,
    /// 予約
    pub reserved: u32,
}

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ProgramGuideProgramInitializeMenuInfo {
    /// メニューのハンドル
    pub hmenu: HMENU,
    /// 項目のID
    pub command: u32,
    /// 予約
    pub reserved: u32,
    /// カーソル位置
    pub cursor_pos: POINT,
    /// 番組の位置
    pub item_rect: RECT,
}

/// 番組表のイベントのフラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ProgramGuideEventFlag {
    /// 全体のイベント
    General = 0x0001,
    /// 各番組のイベント
    Program = 0x0002,
}

/// 番組表のコマンドの種類
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u16)]
pub enum ProgramGuideCommandKind {
    /// 各番組
    Program = 0x0001,
}

pub struct ProgramGuideCommandInfo {
    /// 種類
    pub kind: BitFlags<ProgramGuideCommandKind>,
    /// 各種フラグ(現在は常に0)
    pub flags: u16,
    /// 識別子
    pub id: u32,
    /// コマンドの文字列
    pub text: WideStringPtr,
    /// コマンドの名前
    pub name: WideStringPtr,
}

/// 番組表のコマンド実行の操作の種類
#[repr(u32)]
pub enum ProgramGuideCommandAction {
    /// マウスなど
    Mouse,
    /// キーボード
    Keyboard,
}

/// 番組表のコマンド実行時の情報
#[repr(C)]
pub struct ProgramGuideCommandParam {
    /// 識別子
    pub id: u32,
    /// 操作の種類
    pub action: ProgramGuideCommandAction,
    /// 番組の情報
    pub program: ProgramGuideProgramInfo,
    /// カーソル位置
    pub cursor_pos: POINT,
    /// 項目の位置
    pub item_rect: RECT,
}
