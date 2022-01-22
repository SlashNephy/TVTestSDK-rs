use std::mem::size_of;

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ChannelInfo {
    pub size: u32,                        // 構造体のサイズ
    pub space: i32,                       // チューニング空間(BonDriverのインデックス)
    pub channel: i32,                     // チャンネル(BonDriverのインデックス)
    pub remote_control_key_id: i32,       // リモコンID
    pub network_id: u16,                  // ネットワークID
    pub transport_stream_id: u16,         // トランスポートストリームID
    pub network_name: [u16; 32],          // ネットワーク名
    pub transport_stream_name: [u16; 32], // トランスポートストリーム名
    pub channel_name: [u16; 64],          // チャンネル名
    pub physical_channel: i32,            // 物理チャンネル番号(あまり信用できない)。不明の場合は0
    pub service_index: u16,               // サービスのインデックス(現在は意味を無くしているので使わない)
    pub service_id: u16,                  // サービスID
    // サービスはチャンネルファイルで設定されているものが取得される
    // サービスはユーザーが切り替えられるので、実際に視聴中のサービスがこれであるとは限らない
    // 実際に視聴中のサービスは MESSAGE_GETSERVICE で取得できる
    pub flags: ChannelFlag                // 各種フラグ
}

impl Default for ChannelInfo {
    fn default() -> ChannelInfo {
        ChannelInfo {
            size: size_of::<Self>() as u32,
            space: 0,
            channel: 0,
            remote_control_key_id: 0,
            network_id: 0,
            transport_stream_id: 0,
            network_name: [0; 32],
            transport_stream_name: [0; 32],
            channel_name: [0; 64],
            physical_channel: 0,
            service_index: 0,
            service_id: 0,
            flags: ChannelFlag::Normal
        }
    }
}

#[repr(u32)]
#[cfg_attr(test, derive(Debug))]
pub enum ChannelFlag {
    Normal = 0x00000000,
    Disabled = 0x00000001  // 無効にされている
}
