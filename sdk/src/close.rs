use enumflags2::bitflags;

/// ウィンドウクローズのフラグ
#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum CloseFlag {
    /// 必ず終了させる
    ForceClose = 0x00000001,
}
