/// パンスキャンの種類
#[repr(i32)]
pub enum PanScanKind {
    // なし
    None,
    // レターボックス
    LetterBox,
    // ピラーボックス
    PillarBox,
    // 超額縁
    WindowBox,
}

/// パンスキャンの情報
#[repr(C)]
pub struct PanScanInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 種類
    pub kind: PanScanKind,
    /// 水平アスペクト比
    pub x_aspect: i32,
    /// 垂直アスペクト比
    pub y_aspect: i32,
}
