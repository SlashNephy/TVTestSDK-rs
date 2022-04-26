use enumflags2::{bitflags, BitFlags};
use crate::ClientData;
use crate::win32::UnsafePtr;

type StreamCallbackFunc = unsafe extern "system" fn(
    data: UnsafePtr<u8>,
    client_data: ClientData
) -> bool;

/// ストリームコールバックフラグ
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum StreamCallbackFlag {
    /// コールバックの削除
    Remove = 0x00000001,
}

/// ストリームコールバックの情報
pub struct StreamCallbackInfo {
    /// 構造体のサイズ
    pub size: u32,
    /// フラグ
    pub flags: BitFlags<StreamCallbackFlag>,
    /// コールバック関数
    pub callback: StreamCallbackFunc,
    /// コールバック関数に渡されるデータ
    pub client_data: ClientData,
}
