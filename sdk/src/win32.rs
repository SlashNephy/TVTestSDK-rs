use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::slice;
use windows::Win32::Foundation::{LPARAM, LRESULT};

/// NULL 終端なワイド文字列
pub struct WideString(pub Vec<u16>);

pub trait EncodeIntoWideString<WSTR: Sized> {
    fn into_wide_string(self) -> WSTR;
}

pub trait DecodeFromWideString<STR: Sized> {
    fn into_string(self) -> STR;
}

/// http://d.sunnyone.org/2015/06/rustwindowslpcwstr-lpwstr.html
impl EncodeIntoWideString<WideString> for &str {
    fn into_wide_string(self) -> WideString {
        let vec: Vec<u16> = OsStr::new(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        WideString(vec)
    }
}

impl EncodeIntoWideString<WideString> for String {
    fn into_wide_string(self) -> WideString {
        let vec: Vec<u16> = OsString::from(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        WideString(vec)
    }
}

impl EncodeIntoWideString<WideString> for &[u16] {
    fn into_wide_string(self) -> WideString {
        let vec: Vec<u16> = OsString::from_wide(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        WideString(vec)
    }
}

impl DecodeFromWideString<String> for WideString {
    fn into_string(self) -> String {
        let slice = self.0.as_slice();

        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

impl DecodeFromWideString<String> for &[u16] {
    fn into_string(self) -> String {
        OsString::from_wide(self).to_string_lossy().into_owned()
    }
}

impl DecodeFromWideString<String> for Vec<u16> {
    fn into_string(self) -> String {
        self.as_slice().into_string()
    }
}

pub struct WideStringPtr(pub *const u16);

impl WideStringPtr {
    pub unsafe fn length(&self) -> usize {
        let ptr = self.0;
        assert!(!ptr.is_null());

        (0..isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap()
    }

    pub unsafe fn as_slice(&self) -> &[u16] {
        let ptr = self.0;
        assert!(!ptr.is_null());

        let len = self.length();
        slice::from_raw_parts(ptr, len)
    }
}

impl EncodeIntoWideString<WideStringPtr> for &str {
    fn into_wide_string(self) -> WideStringPtr {
        let vec: Vec<u16> = OsStr::new(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();
        let ptr = vec.as_ptr();

        WideStringPtr(ptr)
    }
}

impl EncodeIntoWideString<WideStringPtr> for String {
    fn into_wide_string(self) -> WideStringPtr {
        let vec: Vec<u16> = OsString::from(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();
        let ptr = vec.as_ptr();

        WideStringPtr(ptr)
    }
}

impl DecodeFromWideString<String> for WideStringPtr {
    fn into_string(self) -> String {
        let ptr = self.0;
        assert!(!ptr.is_null());

        unsafe {
            let slice = self.as_slice();
            OsString::from_wide(slice).to_string_lossy().into_owned()
        }
    }
}

pub(crate) struct LongParam(LPARAM);

impl Into<LPARAM> for LongParam {
    fn into(self) -> LPARAM {
        self.0
    }
}

impl From<isize> for LongParam {
    fn from(value: isize) -> Self {
        LongParam(
            LPARAM(value)
        )
    }
}

impl From<i32> for LongParam {
    fn from(value: i32) -> Self {
        LongParam(
            LPARAM(value as isize)
        )
    }
}

pub(crate) struct LongResult(LRESULT);

impl From<LRESULT> for LongResult {
    fn from(value: LRESULT) -> LongResult {
        LongResult(value)
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
