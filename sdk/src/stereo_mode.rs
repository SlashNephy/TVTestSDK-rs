/// ステレオモード
#[repr(isize)]
#[derive(UnsafeFromPrimitive)]
pub enum StereoMode {
    /// ステレオ
    Stereo,
    /// 左(主音声)
    Left,
    /// 右(副音声)
    Right,
}
