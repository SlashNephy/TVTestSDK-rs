use enumflags2::BitFlags;
use windows::Win32::Foundation::{FILETIME, SYSTEMTIME};
use crate::WideStringPtr;
use crate::win32::UnsafePtr;

/// イベントの取得方法
#[repr(u8)]
pub enum EpgEventQueryMethod {
    /// イベントID
    EventId,
    /// 日時
    Time,
}

pub union EpgEventQuery {
    /// イベントID
    pub event_id: u16,
    /// 日時(UTC)
    pub time: FILETIME,
}

/// イベントの取得のための情報
pub struct EpgEventQueryInfo {
    /// ネットワークID
    pub network_id: u16,
    /// ストリームID
    pub transport_stream_id: u16,
    /// サービスID
    pub service_id: u16,
    /// 取得方法
    pub method: EpgEventQueryMethod,
    /// フラグ(現在は常に0)
    pub flags: u8,
    pub query: EpgEventQuery,
}

/// 映像の情報
#[repr(C)]
pub struct EpgEventVideoInfo {
    /// stream_content
    pub stream_content: u8,
    /// component_type
    /// (0x01 = 480i[4:3] / 0x03 = 480i[16:9] / 0xB1 = 1080i[4:3] / 0xB3 = 1080i[16:9])
    pub component_type: u8,
    /// component_tag
    pub component_tag: u8,
    /// 予約
    pub reserved: u8,
    /// 言語コード
    pub language_code: u32,
    /// テキスト(無い場合はnullptr)
    pub text: WideStringPtr,
}

/// 音声のフラグ
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum EpgEventAudioFlag {
    /// 二ヶ国語
    Multilingual  = 0x01,
    /// 主音声
    MainComponent = 0x02,
}

/// 音声の情報
#[repr(C)]
pub struct EpgEventAudioInfo {
    /// フラグ
    pub flags: BitFlags<EpgEventAudioFlag>,
    /// stream_content
    pub stream_content: u8,
    /// component_type (1 = Mono / 2 = Dual Mono / 3 = Stereo / 9 = 5.1ch)
    pub component_type: u8,
    /// component_tag
    pub component_tag: u8,
    /// simulcast_group_tag
    pub simulcast_group_tag: u8,
    /// quality_indicator
    pub quality_indicator: u8,
    /// サンプリング周波数の種類
    pub sampling_rate: u8,
    /// 予約
    pub reserved: u8,
    /// 言語コード(主音声)
    pub language_code: u32,
    /// 言語コード(副音声)
    pub language_code2: u32,
    /// テキスト(無い場合は nullptr)
    pub text: WideStringPtr,
}

/// ジャンルの情報
/// (意味は STD-B10 第2部 付録H 等参照)
#[repr(C)]
pub struct EpgEventContentInfo {
    /// 大分類
    pub content_nibble_level1: u8,
    /// 中分類
    pub content_nibble_level2: u8,
    pub user_nibble1: u8,
    pub user_nibble2: u8,
}

/// イベントグループのイベントの情報
pub struct EpgGroupEventInfo {
    /// ネットワークID
    pub network_id: u16,
    /// ストリームID
    pub transport_stream_id: u16,
    /// サービスID
    pub service_id: u16,
    /// イベントID
    pub event_id: u16,
}

/// イベントグループの情報
pub struct EpgEventGroupInfo {
    /// 種類
    pub group_type: u8,
    /// イベントのリストの要素数
    pub event_list_length: u8,
    /// 予約
    pub reserved: [u8; 6],
    /// イベントのリスト
    pub event_list: UnsafePtr<EpgGroupEventInfo>,
}

/// イベントの情報
#[repr(C)]
pub struct EpgEventInfo {
    /// イベントID
    pub event_id: u16,
    /// running_status
    pub running_status: u8,
    /// free_CA_mode
    pub free_ca_mode: u8,
    /// 予約
    pub reserved: u32,
    /// 開始日時(EPG 日時 : UTC+9)
    pub start_time: SYSTEMTIME,
    /// 長さ(秒単位)
    pub duration: u32,
    /// 映像の情報の数
    pub video_list_length: u8,
    /// 音声の情報の数
    pub audio_list_length: u8,
    /// ジャンルの情報の数
    pub content_list_length: u8,
    /// イベントグループの情報の数
    pub event_group_list_length: u8,
    /// イベント名(無い場合はnullptr)
    pub event_name: WideStringPtr,
    /// テキスト(無い場合はnullptr)
    pub event_text: WideStringPtr,
    /// 拡張テキスト(無い場合はnullptr)
    pub event_extended_text: WideStringPtr,
    /// 映像の情報のリスト(無い場合はnullptr)
    pub video_list: *const EpgEventVideoInfo,
    /// 音声の情報のリスト(無い場合はnullptr)
    pub audio_list: UnsafePtr<EpgEventAudioInfo>,
    /// ジャンルの情報(無い場合はnullptr)
    pub content_list: UnsafePtr<EpgEventContentInfo>,
    /// イベントグループ情報(無い場合はnullptr)
    pub event_group_list: UnsafePtr<EpgEventGroupInfo>,
}

/// イベントのリスト
#[repr(C)]
pub struct EpgEventList {
    /// ネットワークID
    pub network_id: u16,
    /// ストリームID
    pub transport_stream_id: u16,
    /// サービスID
    pub service_id: u16,
    /// イベントの数
    pub num_events: u16,
    /// リスト
    pub event_list: UnsafePtr<EpgEventInfo>,
}

/// EPG 取得状況のステータス
#[bitflags]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum EpgCaptureStatus {
    /// schedule basic が揃っている
    ScheduleBasicCompleted    = 0x00000001,
    /// schedule extended が揃っている
    ScheduleExtendedCompleted = 0x00000002,
    /// schedule basic が存在する
    HasScheduleBasic          = 0x00000004,
    /// schedule extended が存在する
    HasScheduleExtended       = 0x00000008,
}

/// EPG 取得状況の情報
#[repr(C)]
pub struct EpgCaptureStatusInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// 各種フラグ(現在は常に0)
    pub flags: u16,
    /// ネットワークID
    pub network_id: u16,
    /// ストリームID
    pub transport_stream_id: u16,
    /// サービスID
    pub service_id: u16,
    /// ステータス
    pub status: BitFlags<EpgCaptureStatus>,
}
