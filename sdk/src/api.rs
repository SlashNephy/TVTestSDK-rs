use std::cmp::min;
use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;
use std::sync::Arc;

use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::Foundation::HINSTANCE;

use crate::channel::ChannelInfo;
use crate::{ClientData};
use crate::event::EventCallbackFunc;
use crate::log::LogKind;
use crate::message::Message;
use crate::plugin::PluginParam;
use crate::service::{GetServiceInfo, ServiceInfo};
use crate::tuning_space::GetTuningSpaceNameInfo;
use crate::version::Version;
use crate::win32::{IntoRustString, IntoWideString, make_long, make_lparam, UnsafePtr};

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
        let result = self.param.send_message(Message::GetVersion, LPARAM(0), LPARAM(0)).0 as u32;

        result.into()
    }

    // 指定されたメッセージに対応しているか問い合わせる
    pub fn query_message(&self, message: Message) -> bool {
        self.param.send_message_bool(Message::QueryMessage, LPARAM(message as isize), LPARAM(0))
    }

    // メモリ再確保
    // data が nullptr で新しい領域を確保
    // Size が0で領域を解放
    pub fn memory_realloc(&self, data: UnsafePtr<c_void>, size: isize) -> UnsafePtr<c_void> {
        let param1 = match data {
            Some(p) => p.as_ptr() as isize,
            None => 0,
        };
        let result = self.param.send_message(Message::MemoryAlloc, LPARAM(param1), LPARAM(size));

        NonNull::new(result.0 as *mut c_void)
    }

    // メモリ確保
    pub fn memory_alloc(&self, size: isize) -> UnsafePtr<c_void> {
        self.memory_realloc(None, size)
    }

    // メモリ開放
    pub fn memory_free(&self, data: UnsafePtr<c_void>) {
        self.memory_realloc(data, 0);
    }

    // 文字列複製
    // pub unsafe fn string_duplicate(&self, string: PWSTR) -> PWSTR {
    //     let size = (lstrlenW(string) + 1) as usize * size_of::<u16>();
    //     let dup = self.memory_alloc(size as isize);
    //     if !dup.is_null() {
    //         let ptr = string.0 as *mut c_void;
    //         copy(ptr, dup, size);
    //     }
    //
    //     unimplemented!("TODO")
    //     // return dup;
    // }

    // イベントハンドル用コールバックの設定
    // pClientData はコールバックの呼び出し時に渡されます。
    // 一つのプラグインで設定できるコールバック関数は一つだけです。
    // Callback に nullptr を渡すと設定が解除されます。
    pub unsafe fn set_event_callback(&self, callback: EventCallbackFunc) -> bool {
        let ptr = NonNull::new_unchecked(callback as *mut EventCallbackFunc);
        let ptr2 = ptr::null::<c_void>();
        self.param.send_message_bool(Message::SetEventCallback, LPARAM(ptr.as_ptr() as isize), LPARAM(ptr2 as isize))
    }
    pub unsafe fn set_event_callback_with_client_data(&self, callback: EventCallbackFunc, client_data: &ClientData) -> bool {
        let ptr = NonNull::new_unchecked(callback as *mut EventCallbackFunc);
        let ptr2 = NonNull::from(client_data);
        self.param.send_message_bool(Message::SetEventCallback, LPARAM(ptr.as_ptr() as isize), LPARAM(ptr2.as_ptr() as isize))
    }
    pub fn unset_event_callback(&self) -> bool {
        let ptr = ptr::null::<EventCallbackFunc>();
        let ptr2 = ptr::null::<c_void>();
        self.param.send_message_bool(Message::SetEventCallback, LPARAM(ptr as isize), LPARAM(ptr2 as isize))
    }

    // 現在のチャンネルの情報を取得する
    pub fn get_current_channel_info(&self) -> Option<ChannelInfo> {
        let info = ChannelInfo::default();
        let ptr = &info as *const ChannelInfo;
        let result = self.param.send_message_bool(Message::GetCurrentChannelInfo, LPARAM(ptr as isize), LPARAM(0));

        if result {
            info.into()
        } else {
            None
        }
    }

    // チャンネルを設定する
    // 機能が追加された MESSAGE_SELECTCHANNEL もあります。
    pub fn set_channel(&self, space: i32, channel: i32) -> bool {
        self.param.send_message_bool(Message::SetChannel, LPARAM(space as isize), LPARAM(channel as isize))
    }
    pub fn set_channel_with_service_id(&self, space: i32, channel: i32, service_id: u16) -> bool {
        let param = make_long(channel as u16, service_id);

        self.param.send_message_bool(Message::SetChannel, LPARAM(space as isize), LPARAM(param as isize))
    }

    // 現在のサービス及びサービス数を取得する
    // サービスのインデックスが返る。エラー時は-1が返ります。
    // pNumServices が nullptr でない場合は、サービスの数が返されます。
    pub fn get_service_index(&self) -> Option<i32> {
        let ptr = std::ptr::null::<i32>();
        let index = self.param.send_message(Message::GetService, LPARAM(ptr as isize), LPARAM(0)).0;

        if index != -1 {
            (index as i32).into()
        } else {
            None
        }
    }
    pub fn get_service(&self) -> Option<GetServiceInfo> {
        let num = 0;
        let ptr = NonNull::from(&num);
        let index = self.param.send_message(Message::GetService, LPARAM(ptr.as_ptr() as isize), LPARAM(0)).0;

        if index != -1 {
            GetServiceInfo {
                index: index as i32,
                num_services: num,
            }.into()
        } else {
            None
        }
    }

    // サービスを設定する
    // fByID=false の場合はインデックス、fByID=true の場合はサービスID
    pub fn set_service_by_index(&self, index: i32) -> bool {
        self.param.send_message_bool(Message::SetService, LPARAM(index as isize), LPARAM(false as isize))
    }
    pub fn set_service_by_id(&self, service_id: i32) -> bool {
        self.param.send_message_bool(Message::SetService, LPARAM(service_id as isize), LPARAM(true as isize))
    }

    // チューニング空間名を取得する
    // チューニング空間名の長さが返ります。Indexが範囲外の場合は0が返ります。
    // pszName を nullptr で呼べば長さだけを取得できます。
    // MaxLength には pszName の先に格納できる最大の要素数(終端の空文字を含む)を指定します。
    pub fn get_tuning_space_name_length(&self, index: i32) -> Option<usize> {
        let ptr = ptr::null::<u16>();
        let param = make_lparam(index as u16,  0xFFFF);
        let result = self.param.send_message(Message::GetTuningSpaceName, LPARAM(ptr as isize), param).0;

        if result > 0 {
            (result as usize).into()
        } else {
            None
        }
    }
    pub fn get_tuning_space_name(&self, index: i32, max_length: u16) -> Option<GetTuningSpaceNameInfo> {
        let vec: Vec<u16> = Vec::with_capacity(max_length as usize);
        let ptr = vec.as_ptr();
        let param = make_lparam(index as u16, min(max_length, 0xFFFF));
        let result = self.param.send_message(Message::GetTuningSpaceName, LPARAM(ptr as isize), param).0;

        if result > 0 {
            GetTuningSpaceNameInfo {
                length: result as usize,
                name: vec.into_wide_string().into_string(),
            }.into()
        } else {
            None
        }
    }

    // チャンネルの情報を取得する
    // 事前に ChannelInfo の Size メンバを設定しておきます。
    // szNetworkName, szTransportStreamName は MESSAGE_GETCURRENTCHANNEL でしか取得できません。
    // NetworkID, TransportStreamID はチャンネルスキャンしていないと取得できません。
    // 取得できなかった場合は0になります。
    pub fn get_channel_info(&self, space: i32, index: i32) -> Option<ChannelInfo> {
        let info = ChannelInfo::default();
        let ptr = &info as *const ChannelInfo;
        let result = self.param.send_message_bool(Message::GetChannelInfo, LPARAM(ptr as isize), make_lparam(space as u16, index as u16));

        if result {
            info.into()
        } else {
            None
        }
    }

    // サービスの情報を取得する
    // 現在のチャンネルのサービスの情報を取得します。
    // 事前に ServiceInfo の Size メンバを設定しておきます。
    pub fn get_service_info(&self, index: i32) -> Option<ServiceInfo> {
        let info = ServiceInfo::default();
        let ptr = NonNull::from(&info);
        let result = self.param.send_message_bool(Message::GetServiceInfo, LPARAM(index as isize), LPARAM(ptr.as_ptr() as isize));

        if result {
            info.into()
        } else {
            None
        }
    }

    // BonDriverのファイル名を取得する
    // 戻り値はファイル名の長さ(終端のNullを除く)が返ります。
    // pszName を nullptr で呼べば長さだけを取得できます。
    // 取得されるのは、ディレクトリを含まないファイル名のみか、相対パスの場合もあります。
    // フルパスを取得したい場合は MsgGetDriverFullPathName を使用してください。
    // pub fn get_driver_name_length(&self) -> Option<usize> {
    //     let ptr = ptr::null::<u16>();
    //     let param = make_lparam(index as u16,  0xFFFF);
    //     let result = self.param.send_message(Message::GetTuningSpaceName, LPARAM(ptr as isize), param).0;
    //
    //     if result > 0 {
    //         (result as usize).into()
    //     } else {
    //         None
    //     }
    // }
    // pub fn get_driver_name_name(&self, index: i32, max_length: u16) -> Option<GetTuningSpaceNameInfo> {
    //     let vec: Vec<u16> = Vec::with_capacity(max_length as usize);
    //     let ptr = vec.as_ptr();
    //     let param = make_lparam(index as u16, min(max_length, 0xFFFF));
    //     let result = self.param.send_message(Message::GetTuningSpaceName, LPARAM(ptr as isize), param).0;
    //
    //     if result > 0 {
    //         GetTuningSpaceNameInfo {
    //             length: result as usize,
    //             name: vec.into_string(),
    //         }.into()
    //     } else {
    //         None
    //     }
    // }

    // ログを記録する
    // 設定のログの項目に表示されます。
    pub fn add_log(&self, text: String) -> bool {
        let encoded = text.into_wide_string();
        let ptr = encoded.0.as_ptr();

        self.param.send_message_bool(Message::AddLog, LPARAM(ptr as isize), LPARAM(0))
    }
    pub fn add_log_with_kind(&self, text: String, kind: LogKind) -> bool {
        let encoded = text.into_wide_string();
        let ptr = encoded.0.as_ptr();
        let log_type = kind as isize;

        self.param.send_message_bool(Message::AddLog, LPARAM(ptr as isize), LPARAM(log_type))
    }
}
