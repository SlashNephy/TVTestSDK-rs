use crate::channel::ChannelInfo;
use crate::win32::{FixedWideString, UnsafePtr};

#[cfg_attr(test, derive(Debug))]
pub struct GetTuningSpaceNameInfo {
    pub length: usize,
    pub name: String,
}

/// チューニング空間の種類
#[repr(u32)]
pub enum TuningSpaceKind {
    /// 不明
    Unknown,
    /// 地上デジタル
    Terrestrial,
    /// BS
    BS,
    /// 110度CS
    CS110,
}

/// チューニング空間の情報
pub struct TuningSpaceInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// チューニング空間の種類
    /// 場合によっては信用できない
    pub kind: TuningSpaceKind,
    /// チューニング空間名
    pub name: FixedWideString<64>,
}

/// チューニング空間の情報
#[repr(C)]
pub struct DriverTuningSpaceInfo {
    /// フラグ(現在は常に0)
    pub flags: u32,
    /// チャンネル数
    pub num_channels: u32,
    /// チューニング空間の情報
    pub info: UnsafePtr<TuningSpaceInfo>,
    /// チャンネルのリスト
    pub channel_list: UnsafePtr<ChannelInfo>,
}

/// チューニング空間のリスト
#[repr(C)]
pub struct DriverTuningSpaceList {
    /// フラグ(現在は常に0)
    pub flags: u32,
    /// チューニング空間の数
    pub num_spaces: u32,
    /// チューニング空間のリスト
    pub space_list: UnsafePtr<DriverTuningSpaceInfo>,
}
