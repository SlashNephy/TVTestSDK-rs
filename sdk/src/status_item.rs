use enumflags2::BitFlags;
use crate::WideStringPtr;

/// ステータス項目のフラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum StatusItemFlag {
    /// 定期的に更新する(STATUS_ITEM_EVENT_UPDATETIMER が呼ばれる)
    TimerUpdate = 0x00000001,
}

/// ステータス項目のスタイルフラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum StatusItemStyle {
    /// 可変幅
    VariableWidth = 0x00000001,
    /// 一行表示(表示領域が足りなければ一行表示にならないこともある)
    FullRow       = 0x00000002,
    /// 強制一行表示(常に一行表示になり、常に表示される)
    ForceFullRow  = 0x00000004,
}

/// ステータス項目の情報
#[repr(C)]
pub struct StatusItemInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ
    pub flags: BitFlags<StatusItemFlag>,
    /// スタイルフラグ
    pub style: BitFlags<StatusItemStyle>,
    /// 識別子
    pub id: i32,
    /// 識別子文字列
    pub id_text: WideStringPtr,
    /// 名前
    pub name: WideStringPtr,
    /// 最小の幅
    pub min_width: i32,
    /// 最大の幅(-1で制限なし)
    pub max_width: i32,
    /// デフォルト幅(正数ではピクセル単位、負数ではフォントの高さの-1/1000単位)
    pub default_width: i32,
    /// 最小の高さ
    pub min_height: i32,
}

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct StatusItemDrawInfo;

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct StatusItemEventInfo;

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct StatusItemMouseEventInfo;
