use crate::win32::WideStringPtr;

/// 変数取得の情報
/// Event::GetVariable で渡されます。
#[repr(C)]
#[cfg_attr(test, derive(Debug))]
pub struct GetVariableInfo {
    /// 識別子
    pub keyword: WideStringPtr,

    // 値
    pub value: WideStringPtr,
}
