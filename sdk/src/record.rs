use std::mem::size_of;
use enumflags2::{bitflags, BitFlag, BitFlags};
use windows::Win32::Foundation::FILETIME;
use crate::win32::WideStringPtr;

/// 録画情報のマスク
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum RecordMask {
    /// Flags が有効
    Flags     = 0x00000001,
    /// pszFileName が有効
    Filename  = 0x00000002,
    /// StartTime が有効
    StartTime = 0x00000004,
    /// StopTime が有効
    StopTime  = 0x00000008,
}

/// 録画フラグ
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum RecordFlag {
    /// キャンセル
    Cancel  = 0x10000000,
    /// UTC 日時
    UTC     = 0x00000001,
}

/// 録画開始時間の指定方法
#[repr(u32)]
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum RecordStart {
    /// 未指定
    NotSpecified,
    /// 時刻指定
    Time,
    /// 長さ指定
    Delay,
}

/// 録画停止時間の指定方法
#[repr(u32)]
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum RecordStop {
    /// 未指定
    NotSpecified,
    /// 時刻指定
    Time,
    /// 長さ指定
    Duration,
}

#[repr(C)]
pub union RecordStartTime {
    // 録画開始時刻(StartTimeSpec==RECORD_START_TIME)
    // ローカル時刻(Flags に RECORD_FLAG_UTC を指定した場合 UTC)
    pub time: FILETIME,
    // 録画開始時間(StartTimeSpec==RECORD_START_DELAY)
    // 録画を開始するまでの時間(ms)
    pub delay: u64,
}

#[repr(C)]
pub union RecordStopTime {
    // 録画停止時刻(StopTimeSpec==RECORD_STOP_TIME)
    // ローカル時刻(Flags に RECORD_FLAG_UTC を指定した場合 UTC)
    pub time: FILETIME,
    // 録画停止時間(StopTimeSpec==RECORD_STOP_DURATION)
    // 開始時間からのミリ秒
    pub duration: u64,
}

/// 録画情報
#[repr(C)]
pub struct RecordInfo {
    // 構造体のサイズ
    pub size: u32,
    // マスク
    pub mask: BitFlags<RecordMask>,
    // フラグ
    pub flags: BitFlags<RecordFlag>,
    // ファイル名(nullptrでデフォルト)
    // %～% で囲まれた置換キーワードを使用できます
    pub filename: WideStringPtr,
    // ファイル名の最大長(MESSAGE_GETRECORDのみで使用)
    pub max_filename: i32,
    // 録画予約された時刻(MESSAGE_GETRECORDのみで使用)
    // ローカル時刻(Flags に RECORD_FLAG_UTC を指定した場合 UTC)
    pub reserve_time: FILETIME,
    // 録画開始時間の指定方法
    pub start_time_spec: RecordStart,
    pub start_time: RecordStartTime,
    // 録画停止時間の指定方法
    pub stop_time_spec: RecordStop,
    pub stop_time: RecordStopTime,
}

impl Default for RecordInfo {
    fn default() -> Self {
        Self {
            size: size_of::<Self>() as u32,
            mask: RecordMask::empty(),
            flags: RecordFlag::empty(),
            filename: Default::default(),
            max_filename: 0,
            reserve_time: Default::default(),
            start_time_spec: RecordStart::NotSpecified,
            start_time: RecordStartTime {
                time: FILETIME::default()
            },
            stop_time_spec: RecordStop::NotSpecified,
            stop_time: RecordStopTime {
                time: FILETIME::default()
            }
        }
    }
}

#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct StartRecordInfo;
