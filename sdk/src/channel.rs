use std::mem::size_of;
use enumflags2::{bitflags, BitFlag, BitFlags};
use crate::win32::FixedWideString;

/// チャンネルの情報
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ChannelInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// チューニング空間 (BonDriver のインデックス)
    pub space: i32,
    /// チャンネル (BonDriver のインデックス)
    pub channel: i32,
    /// リモコン ID
    pub remote_control_key_id: i32,
    /// ネットワーク ID
    pub network_id: u16,
    /// トランスポートストリーム ID
    pub transport_stream_id: u16,
    /// ネットワーク名
    pub network_name: FixedWideString<32>,
    /// トランスポートストリーム名
    pub transport_stream_name: FixedWideString<32>,
    /// チャンネル名
    pub channel_name: FixedWideString<64>,
    /// 物理チャンネル番号 (あまり信用できない)。不明の場合は0
    pub physical_channel: i32,
    /// サービスのインデックス (現在は意味を無くしているので使わない)
    pub service_index: u16,
    /// サービスID
    /// サービスはチャンネルファイルで設定されているものが取得される
    /// サービスはユーザーが切り替えられるので、実際に視聴中のサービスがこれであるとは限らない
    /// 実際に視聴中のサービスは MESSAGE_GETSERVICE で取得できる
    pub service_id: u16,
    /// 各種フラグ
    pub flags: BitFlags<ChannelFlag>
}

impl Default for ChannelInfo {
    fn default() -> ChannelInfo {
        Self {
            size: size_of::<Self>() as u32,
            space: 0,
            channel: 0,
            remote_control_key_id: 0,
            network_id: 0,
            transport_stream_id: 0,
            network_name: FixedWideString::<32>::default(),
            transport_stream_name: FixedWideString::<32>::default(),
            channel_name: FixedWideString::<64>::default(),
            physical_channel: 0,
            service_index: 0,
            service_id: 0,
            flags: ChannelFlag::empty(),
        }
    }
}

// チャンネルの情報のフラグ
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum ChannelFlag {
    // 無効にされている
    Disabled = 0x00000001
}
