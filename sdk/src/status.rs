/// ステータス情報
#[repr(C)]
pub struct StatusInfo {
    /// 構造体のサイズ
    pub size: u32,
    // 信号レベル(dB)
    pub signal_level: f32,
    // ビットレート(Bits/Sec)
    pub bit_rate: u32,
    // エラーパケット数
    // DropPacketCount も含まれる
    pub error_packet_count: u32,
    // 復号漏れパケット数
    pub scramble_packet_count: u32,
    // ドロップパケット数
    pub drop_packet_count: u32,
    // 予約
    pub reserved: u32,
}
