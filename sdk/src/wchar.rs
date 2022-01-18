use windows::Win32::Foundation::PWSTR;

pub trait ConvertToPWSTR {
    fn as_pwstr(&self) -> PWSTR;
}

impl ConvertToPWSTR for &str {
    fn as_pwstr(&self) -> PWSTR {
        let slice = format!("{}{}", self, "\0");
        let mut vec: Vec<u16> = slice.encode_utf16().collect();
        PWSTR(vec.as_mut_ptr())
    }
}
