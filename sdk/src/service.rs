use std::mem::size_of;

#[cfg_attr(test, derive(Debug))]
pub struct GetServiceInfo {
    pub index: i32,
    pub num_services: i32,
}

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct ServiceInfo {
    pub size: u32,                     // 構造体のサイズ
    pub service_id: u16,               // サービスID
    pub video_pid: u16,                // ビデオストリームのPID
    pub num_audio_pids: i32,           // 音声PIDの数
    pub audio_pids: [u16; 4],          // 音声ストリームのPID
    pub service_name: [u16; 32],       // サービス名
    pub audio_component_type: [u8; 4], // 音声コンポーネントタイプ
    pub subtitle_pid: u16,             // 字幕ストリームのPID(無い場合は0)
    pub reserved: u16,                 // 予約
}

impl Default for ServiceInfo {
    fn default() -> Self {
        ServiceInfo {
            size: size_of::<Self>() as u32,
            service_id: 0,
            video_pid: 0,
            num_audio_pids: 0,
            audio_pids: [0; 4],
            service_name: [0; 32],
            audio_component_type: [0; 4],
            subtitle_pid: 0,
            reserved: 0
        }
    }
}
