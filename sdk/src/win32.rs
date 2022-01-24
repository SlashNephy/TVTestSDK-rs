use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{ptr, slice};
use windows::Win32::Foundation::LPARAM;

/// NULL 終端なワイド文字列
#[cfg_attr(test, derive(Debug))]
pub struct WideString(pub Vec<u16>);

impl WideString {
    /// 生ポインタに変換します
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    /// mutable な生ポインタに変換します
    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        self.0.as_mut_ptr()
    }

    /// WideStringPtr に変換します
    pub fn to_wide_string_ptr(&self) -> WideStringPtr {
        WideStringPtr(self.as_ptr())
    }
}

impl Default for WideString {
    fn default() -> Self {
        WideString(Vec::default())
    }
}

pub trait IntoWideString {
    /// NULL 終端なワイド文字列に変換します
    ///
    /// http://d.sunnyone.org/2015/06/rustwindowslpcwstr-lpwstr.html
    fn into_wide_string(self) -> WideString;
}

impl IntoWideString for &str {
    fn into_wide_string(self) -> WideString {
        let vec: Vec<u16> = OsStr::new(&self)
            .encode_wide()
            // 末尾に NULL 文字を付加
            .chain(Some(0).into_iter())
            .collect();

        WideString(vec)
    }
}

impl IntoWideString for String {
    fn into_wide_string(self) -> WideString {
        let vec: Vec<u16> = OsString::from(self)
            .encode_wide()
            // 末尾に NULL 文字を付加
            .chain(Some(0).into_iter())
            .collect();

        WideString(vec)
    }
}

impl IntoWideString for &[u16] {
    fn into_wide_string(self) -> WideString {
        let vec: Vec<u16> = OsString::from_wide(self).encode_wide().collect();

        WideString(vec)
    }
}

impl IntoWideString for Vec<u16> {
    fn into_wide_string(self) -> WideString {
        self.as_slice().into_wide_string()
    }
}

pub trait IntoRustString {
    /// UTF-8 な Rust 文字列に変換します
    fn into_string(self) -> String;
}

impl IntoRustString for WideString {
    fn into_string(self) -> String {
        let slice = self.0.as_slice();

        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

impl IntoRustString for &[u16] {
    fn into_string(self) -> String {
        self.into_wide_string().into_string()
    }
}

impl IntoRustString for Vec<u16> {
    fn into_string(self) -> String {
        self.into_wide_string().into_string()
    }
}

/// 固定長な NULL 終端ワイド文字列
#[cfg_attr(test, derive(Debug))]
pub struct FixedWideString<const N: usize>(pub [u16; N]);

impl<const N: usize> FixedWideString<N> {
    pub fn to_wide_string(&self) -> WideString {
        let vec = self.0.into_iter()
            // NULL 文字を除外する
            .filter(|x| *x != 0)
            .collect();

        WideString(vec)
    }
}

impl<const N: usize> IntoRustString for FixedWideString<N> {
    fn into_string(self) -> String {
        self.to_wide_string().into_string()
    }
}

impl<const N: usize> Default for FixedWideString<N> {
    fn default() -> Self {
        FixedWideString([0; N])
    }
}

/// NULL 終端なワイド文字列ポインタ
#[cfg_attr(test, derive(Debug))]
pub struct WideStringPtr(pub *const u16);

impl WideStringPtr {
    /// ぬるぽかどうか確認します
    pub fn is_null(&self) -> bool {
        let ptr = self.0;

        ptr.is_null()
    }

    /// 文字列の長さを返します
    pub unsafe fn get_length(&self) -> Option<usize> {
        let ptr = self.0;
        if self.is_null() {
            return None;
        }

        (0..isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap().into()
    }

    /// 文字列のスライスを取得します
    pub unsafe fn as_slice(&self) -> Option<&[u16]> {
        let ptr = self.0;
        let len = self.get_length()?;

        slice::from_raw_parts(ptr, len).into()
    }

    /// 非ポインターな WideString に変換します
    pub unsafe fn to_wide_string(&self) -> Option<WideString> {
        let slice = self.as_slice()?;

        slice.into_wide_string().into()
    }
}

impl Default for WideStringPtr {
    fn default() -> Self {
        WideStringPtr(ptr::null())
    }
}

pub unsafe trait UnsafeIntoRustString {
    /// NULL 終端なワイド文字列ポインタ先を読み取ります
    fn read_wide_string(self) -> Option<WideString>;

    /// NULL 終端なワイド文字列ポインタ先を読み取り UTF-8 な Rust 文字列に変換します
    fn read_string(self) -> Option<String>;
}

unsafe impl UnsafeIntoRustString for WideStringPtr {
    fn read_wide_string(self) -> Option<WideString> {
        let slice = unsafe {
            self.as_slice()?
        };

        slice.into_wide_string().into()
    }

    fn read_string(self) -> Option<String> {
        self.read_wide_string()?
            .into_string()
            .into()
    }
}

impl Into<WideStringPtr> for &str {
    fn into(self) -> WideStringPtr {
        self.into_wide_string().to_wide_string_ptr()
    }
}

#[inline]
pub(crate) fn make_lparam(l: u16, h: u16) -> LPARAM {
    LPARAM(
        make_long(l, h) as isize
    )
}

#[inline]
pub(crate) fn make_long(a: u16, b: u16) -> u32 {
    let a = ((a as usize) & 0xffff) as u32;
    let b = ((b as usize) & 0xffff) as u32;
    a | b << 16
}
