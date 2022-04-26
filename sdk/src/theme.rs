use enumflags2::BitFlags;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{HBITMAP, HDC};
use crate::WideStringPtr;

/// テーマ描画フラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ThemeDrawBackgroundFlag {
    /// クライアント領域を取得
    AdjustRect = 0x00000001,
}

/// テーマの背景描画情報
#[repr(C)]
pub struct ThemeDrawBackgroundInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ
    pub flags: BitFlags<ThemeDrawBackgroundFlag>,
    /// スタイル名
    pub style: WideStringPtr,
    /// 描画先DC
    pub hdc: HDC,
    /// 描画領域
    pub draw_rect: RECT,
    /// DPI の指定(0でメインウィンドウと同じ)
    pub dpi: i32,
}

/// テーマの文字列描画情報
#[repr(C)]
pub struct ThemeDrawTextInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ(現在は常に0)
    pub flags: u32,
    /// スタイル名
    pub style: WideStringPtr,
    /// 描画先DC
    pub hdc: HDC,
    /// 描画する文字列
    pub text: WideStringPtr,
    /// 描画先の領域
    pub draw_rect: RECT,
    /// 描画フラグ(DrawText API の DT_*)
    pub draw_flags: u32,
    /// 描画する色(CLR_INVALID でデフォルトの色)
    pub color: u32,
}

/// テーマのアイコン描画情報
#[repr(C)]
pub struct ThemeDrawIconInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ(現在は常に0)
    pub flags: u32,
    /// スタイル名
    pub style: WideStringPtr,
    /// 描画先DC
    pub hdc: HDC,
    /// 描画するビットマップ
    pub hbm: HBITMAP,
    /// 描画先の領域
    pub dest_rect: RECT,
    /// 描画元の領域
    pub src_rect: RECT,
    /// 描画する色(CLR_INVALID でデフォルトの色)
    pub color: u32,
    /// 不透明度(1-255)
    pub opacity: u8,
    /// 予約領域
    pub reserved: [u8; 3],
}
