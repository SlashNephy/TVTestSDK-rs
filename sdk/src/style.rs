use crate::WideStringPtr;

/// スタイル値の単位
#[repr(u32)]
pub enum StyleUnit {
    /// 未定義
    Undefined,
    /// 論理ピクセル(96 DPI におけるピクセル単位)
    LogicalPixel,
    /// 物理ピクセル
    PhysicalPixel,
    /// ポイント(1/72インチ)
    Point,
    /// dip(1/160インチ)
    Dip,
}

/// スタイル値の情報
#[repr(C)]
pub struct StyleValueInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ(現在は常に0)
    pub flags: u32,
    /// スタイル名
    pub name: WideStringPtr,
    /// 取得する値の単位
    pub unit: StyleUnit,
    /// DPI の指定
    pub dpi: i32,
    /// 取得された値
    pub value: i32,
}
