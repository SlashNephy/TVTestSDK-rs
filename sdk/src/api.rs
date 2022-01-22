use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::copy;
use std::sync::Arc;

use windows::Win32::Foundation::{HWND, PWSTR};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::Globalization::lstrlenW;

use crate::channel::ChannelInfo;
use crate::event::EventCallbackFunc;
use crate::log::LogType;
use crate::message::Message;
use crate::plugin::PluginParam;
use crate::version::Version;

#[cfg_attr(test, derive(Debug))]
pub struct PluginApi {
    pub dll: Arc<HINSTANCE>,
    pub param: Arc<PluginParam>,
}

impl PluginApi  {
    pub fn get_app_window(&self) -> HWND {
        self.param.hwnd_app
    }

    // プログラム(TVTest)のバージョンを取得する
    pub fn get_version(&self) -> Version {
        let result = self.param.send_message(Message::GetVersion, 0, 0) as u32;

        result.into()
    }

    // 指定されたメッセージに対応しているか問い合わせる
    pub fn query_message(&self, message: Message) -> bool {
        self.param.send_message_bool(Message::QueryMessage, message as isize, 0)
    }

    // メモリ再確保
    // data が nullptr で新しい領域を確保
    // Size が0で領域を解放
    pub fn memory_realloc(&self, data: *mut c_void, size: isize) -> *mut c_void {
        let result = self.param.send_message(Message::MemoryAlloc, data as isize, size);

        result as *mut c_void
    }

    // メモリ確保
    pub fn memory_alloc(&self, size: isize) -> *mut c_void {
        let ptr = std::ptr::null_mut::<c_void>();

        self.memory_realloc(ptr, size)
    }

    // メモリ開放
    pub fn memory_free(&self, data: *mut c_void) {
        self.memory_realloc(data, 0);
    }

    // 文字列複製
    pub unsafe fn string_duplicate(&self, string: PWSTR) -> PWSTR {
        let size = (lstrlenW(string) + 1) as usize * size_of::<u16>();
        let dup = self.memory_alloc(size as isize);
        if !dup.is_null() {
            let ptr = string.0 as *mut c_void;
            copy(ptr, dup, size);
        }

        unimplemented!("TODO")
        // return dup;
    }

    // イベントハンドル用コールバックの設定
    // pClientData はコールバックの呼び出し時に渡されます。
    // 一つのプラグインで設定できるコールバック関数は一つだけです。
    // Callback に nullptr を渡すと設定が解除されます。
    pub fn set_event_callback(&self, callback: EventCallbackFunc, client_data: *mut c_void) -> bool {
        self.param.send_message_bool(Message::SetEventCallback, callback as isize, client_data as isize)
    }

    // 現在のチャンネルの情報を取得する
    pub fn get_current_channel_info(&self) -> Option<ChannelInfo> {
        let info = ChannelInfo::default();
        let ptr = &info as *const _;

        if self.param.send_message_bool(Message::GetCurrentChannelInfo, ptr as isize, 0) {
            Some(info)
        } else {
            None
        }
    }

    // チャンネルを設定する
    // 機能が追加された MESSAGE_SELECTCHANNEL もあります。
    pub fn set_channel<T: Into<Option<u16>>>(&self, space: i32, channel: i32, service_id: T) -> bool {
        self.param.send_message_bool(Message::SetChannel, space as isize, channel as isize)
    }

    // ログを記録する
    // 設定のログの項目に表示されます。
    pub fn add_log(&self, text: PWSTR) -> bool {
        let ptr = text.0 as *const u16;

        self.param.send_message_bool(Message::AddLog, ptr as isize, 0)
    }
    pub fn add_log_with_type(&self, text: PWSTR, log_type: LogType) -> bool {
        let ptr = text.0 as *const u16;
        let log_type = match log_type.into() {
            Some(t) => t as isize,
            None => 0
        };

        self.param.send_message_bool(Message::AddLog, ptr as isize, log_type)
    }
}
