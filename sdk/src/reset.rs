/// リセットのフラグ
#[repr(isize)]
pub enum ResetFlag {
    /// 全て
    All    = 0x00000000,
    /// ビューアのみ
    Viewer = 0x00000001,
}
