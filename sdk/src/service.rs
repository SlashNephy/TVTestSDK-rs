use std::mem::size_of;
use crate::win32::FixedWideString;

#[cfg_attr(test, derive(Debug))]
pub struct GetServiceInfo {
    pub index: i32,
    pub num_services: i32,
}

/// サービスの情報
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ServiceInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// サービスID
    pub service_id: u16,
    /// ビデオストリームのPID
    pub video_pid: u16,
    /// 音声PIDの数
    pub num_audio_pids: i32,
    /// 音声ストリームのPID
    pub audio_pids: [u16; 4],
    /// サービス名
    pub service_name: FixedWideString<32>,
    /// 音声コンポーネントタイプ
    pub audio_component_type: [u8; 4],
    /// 字幕ストリームのPID(無い場合は0)
    pub subtitle_pid: u16,
    /// 予約
    pub reserved: u16,
}

impl Default for ServiceInfo {
    fn default() -> Self {
        Self {
            size: size_of::<Self>() as u32,
            service_id: 0,
            video_pid: 0,
            num_audio_pids: 0,
            audio_pids: [0; 4],
            service_name: FixedWideString::<32>::default(),
            audio_component_type: [0; 4],
            subtitle_pid: 0,
            reserved: 0
        }
    }
}
