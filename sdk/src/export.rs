use std::ptr::NonNull;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT};
use crate::{DrawCommandIconInfo, FilterGraphInfo, GetVariableInfo, PanelItemEventInfo, ProgramGuideCommandParam, ProgramGuideInitializeMenuInfo, ProgramGuideProgramDrawBackgroundInfo, ProgramGuideProgramInfo, ProgramGuideProgramInitializeMenuInfo, RecordStatus, StartRecordInfo, StatusItemDrawInfo, StatusItemEventInfo, StatusItemMouseEventInfo, StereoMode, TVTestEventHandler, WideStringPtr};
use crate::event::Event;

#[repr(u32)]
pub enum DllLoadReason {
    ProcessDetach,
    ProcessAttach,
    ThreadAttach,
    ThreadDetach,
}

// TVTEST_PLUGIN_CLASS_IMPLEMENT を再現するマクロ
#[macro_export]
macro_rules! export_plugin {
    ($type: path) => {
        #[deprecated]
        static mut __UNSAFE_DLL__: Option<std::sync::Arc<tvtest::windows::Win32::Foundation::HINSTANCE>> = None;
        #[deprecated]
        static mut __UNSAFE_PLUGIN__: Option<$type> = None;

        // エントリポイント
        // プラグインクラスのインスタンスの生成と破棄を行っています
        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub unsafe extern "system" fn DllMain(
            dll: tvtest::windows::Win32::Foundation::HINSTANCE,
            reason: tvtest::export::DllLoadReason,
            _reserved: *mut std::ffi::c_void,
        ) -> bool {
            use tvtest::export::DllLoadReason;
            use std::sync::Arc;

            match reason {
                DllLoadReason::ProcessAttach => {
                    __UNSAFE_DLL__ = Arc::new(dll).into();
                }
                DllLoadReason::ProcessDetach => {
                    __UNSAFE_DLL__ = None;
                    __UNSAFE_PLUGIN__ = None;
                }
                _ => {}
            };

            return true;
        }

        // プラグインの準拠するプラグイン仕様のバージョンを返す
        // プラグインがロードされると最初にこの関数が呼ばれ、
        // 対応していないバージョンが返された場合はすぐにアンロードされます。
        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub extern "system" fn TVTGetVersion() -> u32 {
            <$type>::get_api_version().into()
        }

        // プラグインの情報を取得する
        // TVTGetVersion の次に呼ばれるので、プラグインの情報を PluginInfo 構造体に設定します。
        // FALSE が返された場合、すぐにアンロードされます。
        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub extern "system" fn TVTGetPluginInfo(
            mut info: tvtest::plugin::PluginInfo,
        ) -> bool {
            let new_info = <$type>::get_info();
            info.kind = new_info.kind;
            info.flags = new_info.flags;
            info.name = new_info.name;
            info.copyright = new_info.copyright;
            info.description = new_info.description;

            true
        }

        // 初期化を行う
        // TVTGetPluginInfo の次に呼ばれるので、初期化処理を行います。
        // FALSE が返された場合、すぐにアンロードされます。
        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub unsafe extern "system" fn TVTInitialize(
            mut param: tvtest::plugin::PluginParam,
        ) -> bool {
            use std::sync::Arc;
            use tvtest::api::PluginApi;

            if let Some(dll) = &__UNSAFE_DLL__ {
                let param = Arc::new(param);
                let api = PluginApi {
                    dll: Arc::clone(dll),
                    param: Arc::clone(&param),
                };
                let plugin = <$type>::new(api);
                plugin.api.set_event_callback(default_event_handler);

                let result = plugin.initialize();
                __UNSAFE_PLUGIN__ = plugin.into();

                result
            } else {
                panic!("__UNSAFE_DLL__ has not initialized yet.")
            }
        }

        // 終了処理を行う
        // プラグインがアンロードされる前に呼ばれるので、終了処理を行います。
        // この関数が呼ばれるのは TVTInitialize 関数が TRUE を返した場合だけです。
        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub unsafe extern "system" fn TVTFinalize() -> bool {
            if let Some(plugin) = &__UNSAFE_PLUGIN__ {
                let result = plugin.finalize();
                __UNSAFE_PLUGIN__ = None;

                result
            } else {
                panic!("__UNSAFE_PLUGIN__ has not initialized yet.")
            }
        }

        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub unsafe extern "system" fn default_event_handler(
            event: tvtest::event::Event,
            param1: tvtest::windows::Win32::Foundation::LPARAM,
            param2: tvtest::windows::Win32::Foundation::LPARAM,
            client_data: tvtest::ClientData,
        ) -> tvtest::windows::Win32::Foundation::LRESULT {
            if let Some(plugin) = &__UNSAFE_PLUGIN__ {
                tvtest::export::handle_event(event, param1, param2, plugin)
            } else {
                panic!("__UNSAFE_PLUGIN__ has not initialized yet.")
            }
        }
    }
}

#[inline]
pub fn handle_event<T: TVTestEventHandler>(
    event: Event,
    param1: LPARAM,
    param2: LPARAM,
    handler: &T,
) -> LRESULT {
    let param1 = param1.0;
    let param2 = param2.0;

    let result: isize = match event {
        Event::PluginEnable => handler.on_plugin_enable(param1 != 0) as isize,
        Event::PluginSettings => handler.on_plugin_settings(HWND(param1)) as isize,
        Event::ChannelChange => handler.on_channel_change() as isize,
        Event::ServiceChange => handler.on_service_change() as isize,
        Event::DriverChange => handler.on_driver_change() as isize,
        Event::ServiceUpdate => handler.on_service_update() as isize,
        Event::RecordStatusChange => unsafe {
            let status = RecordStatus::from_unchecked(param1 as u32);
            handler.on_record_status_change(status) as isize
        },
        Event::FullScreenChange => handler.on_fullscreen_change(param1 != 0) as isize,
        Event::PreviewChange => handler.on_preview_change(param1 != 0) as isize,
        Event::VolumeChange => handler.on_volume_change(param1 as i32, param2 != 0) as isize,
        Event::StereoModeChange => unsafe {
            let mode = StereoMode::from_unchecked(param1);
            handler.on_stereo_mode_change(mode) as isize
        },
        Event::ColorChange => handler.on_color_change() as isize,
        Event::StandBy => handler.on_standby(param1 != 0) as isize,
        Event::Command => handler.on_command(param1 as i32) as isize,
        Event::Execute => {
            let ptr = WideStringPtr(NonNull::new(param1 as *mut u16));
            handler.on_execute(ptr) as isize
        },
        Event::Reset => handler.on_reset() as isize,
        Event::StatusReset => handler.on_status_reset() as isize,
        Event::AudioStreamChange => handler.on_audio_stream_change(param1 as i32) as isize,
        Event::SettingsChange => handler.on_settings_change() as isize,
        Event::Close => handler.on_close() as isize,
        Event::StartRecord => unsafe {
            let ptr = param1 as *const StartRecordInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_start_record(info) as isize
        },
        Event::RelayRecord => {
            let ptr = WideStringPtr(NonNull::new(param1 as *mut u16));
            handler.on_relay_record(ptr) as isize
        },
        Event::ControllerFocus => handler.on_controller_focus(HWND(param1)) as isize,
        Event::StartUpDone => {
            handler.on_startup_done();
            0
        },
        Event::ProgramGuideInitialize => handler.on_program_guide_initialize(HWND(param1)) as isize,
        Event::ProgramGuideFinalize => handler.on_program_guide_finalize(HWND(param1)) as isize,
        Event::ProgramGuideCommand => unsafe {
            let ptr = param2 as *const ProgramGuideCommandParam;
            let param = ptr.as_ref().unwrap();
            handler.on_program_guide_command(param1 as u32, param) as isize
        },
        Event::ProgramGuideInitializeMenu => unsafe {
            let ptr = param1 as *const ProgramGuideInitializeMenuInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_program_guide_initialize_menu(info) as isize
        }
        Event::ProgramGuideMenuSelected => handler.on_program_guide_menu_selected(param1 as u32) as isize,
        Event::ProgramGuideProgramDrawBackground => unsafe {
            let ptr1 = param1 as *const ProgramGuideProgramInfo;
            let program_info = ptr1.as_ref().unwrap();
            let ptr2 = param2 as *const ProgramGuideProgramDrawBackgroundInfo;
            let info = ptr2.as_ref().unwrap();
            handler.on_program_guide_program_draw_background(program_info, info) as isize
        }
        Event::ProgramGuideProgramInitializeMenu => unsafe {
            let ptr1 = param1 as *const ProgramGuideProgramInfo;
            let program_info = ptr1.as_ref().unwrap();
            let ptr2 = param2 as *const ProgramGuideProgramInitializeMenuInfo;
            let info = ptr2.as_ref().unwrap();
            handler.on_program_guide_program_initialize_menu(program_info, info) as isize
        }
        Event::ProgramGuideProgramMenuSelected => unsafe {
            let ptr = param1 as *const ProgramGuideProgramInfo;
            let program_info = ptr.as_ref().unwrap();
            handler.on_program_guide_program_menu_selected(program_info, param2 as u32) as isize
        }
        Event::FilterGraphInitialize => unsafe {
            let ptr = param1 as *const FilterGraphInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_filter_graph_initialize(info);
            0
        }
        Event::FilterGraphInitialized => unsafe {
            let ptr = param1 as *const FilterGraphInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_filter_graph_initialized(info);
            0
        }
        Event::FilterGraphFinalize => unsafe {
            let ptr = param1 as *const FilterGraphInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_filter_graph_finalize(info);
            0
        }
        Event::FilterGraphFinalized => unsafe {
            let ptr = param1 as *const FilterGraphInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_filter_graph_finalized(info);
            0
        }
        Event::DrawCommandIcon => unsafe {
            let ptr = param1 as *const DrawCommandIconInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_draw_command_icon(info) as isize
        }
        Event::StatusItemDraw => unsafe {
            let ptr = param1 as *const StatusItemDrawInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_status_item_draw(info) as isize
        }
        Event::StatusItemNotify => unsafe {
            let ptr = param1 as *const StatusItemEventInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_status_item_notify(info) as isize
        }
        Event::StatusItemMouse => unsafe {
            let ptr = param1 as *const StatusItemMouseEventInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_status_item_mouse_event(info) as isize
        }
        Event::PanelItemNotify => unsafe {
            let ptr = param1 as *const PanelItemEventInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_panel_item_notify(info) as isize
        }
        Event::FavoritesChanged => {
            handler.on_favorites_changed();
            0
        }
        Event::OneSegModeChanged => {
            handler.on_one_seg_mode_changed(param1 != 0);
            0
        }
        Event::GetVariable => unsafe {
            let ptr = param1 as *const GetVariableInfo;
            let info = ptr.as_ref().unwrap();
            handler.on_get_variable(info) as isize
        },
        Event::Trailer => {
            0
        }
    };

    LRESULT(result)
}
