use std::ffi::c_void;
use crate::win32::UnsafePtr;

/// フィルタグラフの情報
/// フィルタグラフ関係のイベントで渡されます。
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct FilterGraphInfo {
    /// 各種フラグ(現在は常に0)
    pub flags: u32,
    /// 映像 stream_type
    pub video_stream_type: u8,
    /// 予約
    pub reserved: [u8; 3],
    /// IGraphBuilder
    pub graph_builder: UnsafePtr<c_void>,  // TODO
}
