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
        static mut __UNSAFE_DLL__: Option<std::sync::Arc<windows::Win32::Foundation::HINSTANCE>> = None;
        #[deprecated]
        static mut __UNSAFE_PLUGIN__: Option<std::sync::Arc<$type>> = None;

        // エントリポイント
        // プラグインクラスのインスタンスの生成と破棄を行っています
        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub unsafe extern "system" fn DllMain(dll: windows::Win32::Foundation::HINSTANCE, reason: tvtest::export::DllLoadReason, _reserved: *mut std::ffi::c_void) -> bool {
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
        pub extern "system" fn TVTGetPluginInfo(mut info: tvtest::plugin::PluginInfo) -> bool {
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
        pub unsafe extern "system" fn TVTInitialize(mut param: tvtest::plugin::PluginParam) -> bool {
            use std::sync::Arc;
            use tvtest::api::PluginApi;

            if let Some(dll) = &__UNSAFE_DLL__ {
                let param = Arc::new(param);
                let api = PluginApi {
                    dll: Arc::clone(dll),
                    param: Arc::clone(&param),
                };
                let plugin = <$type>::new(api);
                plugin.api.set_event_callback(handle_event);

                let result = plugin.initialize();
                __UNSAFE_PLUGIN__ = Arc::new(plugin).into();

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
                let result = plugin.as_ref().finalize();
                __UNSAFE_PLUGIN__ = None;

                result
            } else {
                panic!("__UNSAFE_PLUGIN__ has not initialized yet.")
            }
        }

        #[no_mangle]
        #[deprecated]
        #[allow(deprecated)]
        pub unsafe extern "system" fn handle_event(
            event: tvtest::event::Event,
            param1: windows::Win32::Foundation::LPARAM,
            param2: windows::Win32::Foundation::LPARAM,
            _client_data: *const std::ffi::c_void
        ) -> windows::Win32::Foundation::LRESULT {
            use windows::Win32::Foundation::{HWND, LRESULT};
            use tvtest::command::DrawCommandIconInfo;
            use tvtest::event::Event;
            use tvtest::filter_graph::FilterGraphInfo;
            use tvtest::panel::PanelItemEventInfo;
            use tvtest::program_guide::{ProgramGuideCommandParam, ProgramGuideInitializeMenuInfo, ProgramGuideProgramDrawBackgroundInfo, ProgramGuideProgramInfo, ProgramGuideProgramInitializeMenuInfo};
            use tvtest::record::StartRecordInfo;
            use tvtest::status::{StatusItemDrawInfo, StatusItemEventInfo, StatusItemMouseEventInfo};
            use tvtest::variable::GetVariableInfo;
            use tvtest::win32::WideStringPtr;

            if let Some(plugin) = &__UNSAFE_PLUGIN__ {
                let param1 = param1.0;
                let param2 = param2.0;

                let result: isize = match event {
                    Event::PluginEnable => plugin.on_plugin_enable(param1 != 0) as isize,
                    Event::PluginSettings => plugin.on_plugin_settings(HWND(param1)) as isize,
                    Event::ChannelChange => plugin.on_channel_change() as isize,
                    Event::ServiceChange => plugin.on_service_change() as isize,
                    Event::DriverChange => plugin.on_driver_change() as isize,
                    Event::ServiceUpdate => plugin.on_service_update() as isize,
                    Event::RecordStatusChange => plugin.on_record_status_change(param1 as i32) as isize,
                    Event::FullScreenChange => plugin.on_fullscreen_change(param1 != 0) as isize,
                    Event::PreviewChange => plugin.on_preview_change(param1 != 0) as isize,
                    Event::VolumeChange => plugin.on_volume_change(param1 as i32, param2 != 0) as isize,
                    Event::StereoModeChange => plugin.on_stereo_mode_change(param1 as i32) as isize,
                    Event::ColorChange => plugin.on_color_change() as isize,
                    Event::StandBy => plugin.on_standby(param1 != 0) as isize,
                    Event::Command => plugin.on_command(param1 as i32) as isize,
                    Event::Execute => {
                        let ptr = WideStringPtr(param1 as *mut u16);
                        plugin.on_execute(ptr) as isize
                    },
                    Event::Reset => plugin.on_reset() as isize,
                    Event::StatusReset => plugin.on_status_reset() as isize,
                    Event::AudioStreamChange => plugin.on_audio_stream_change(param1 as i32) as isize,
                    Event::SettingsChange => plugin.on_settings_change() as isize,
                    Event::Close => plugin.on_close() as isize,
                    Event::StartRecord => unsafe {
                        let info = &*(param1 as *const StartRecordInfo);
                        plugin.on_start_record(info) as isize
                    },
                    Event::RelayRecord => {
                        let ptr = WideStringPtr(param1 as *mut u16);
                        plugin.on_relay_record(ptr) as isize
                    },
                    Event::ControllerFocus => plugin.on_controller_focus(HWND(param1)) as isize,
                    Event::StartUpDone => {
                        plugin.on_startup_done();
                        0
                    },
                    Event::ProgramGuideInitialize => plugin.on_program_guide_initialize(HWND(param1)) as isize,
                    Event::ProgramGuideFinalize => plugin.on_program_guide_finalize(HWND(param1)) as isize,
                    Event::ProgramGuideCommand => unsafe {
                        let param = &*(param2 as *const ProgramGuideCommandParam);
                        plugin.on_program_guide_command(param1 as u32, param) as isize
                    },
                    Event::ProgramGuideInitializeMenu => unsafe {
                        let info = &*(param1 as *const ProgramGuideInitializeMenuInfo);
                        plugin.on_program_guide_initialize_menu(info) as isize
                    }
                    Event::ProgramGuideMenuSelected => plugin.on_program_guide_menu_selected(param1 as u32) as isize,
                    Event::ProgramGuideProgramDrawBackground => unsafe {
                        let program_info = &*(param1 as *const ProgramGuideProgramInfo);
                        let info = &*(param2 as *const ProgramGuideProgramDrawBackgroundInfo);
                        plugin.on_program_guide_program_draw_background(program_info, info) as isize
                    }
                    Event::ProgramGuideProgramInitializeMenu => unsafe {
                        let program_info = &*(param1 as *const ProgramGuideProgramInfo);
                        let info = &*(param2 as *const ProgramGuideProgramInitializeMenuInfo);
                        plugin.on_program_guide_program_initialize_menu(program_info, info) as isize
                    }
                    Event::ProgramGuideProgramMenuSelected => unsafe {
                        let program_info = &*(param1 as *const ProgramGuideProgramInfo);
                        plugin.on_program_guide_program_menu_selected(program_info, param2 as u32) as isize
                    }
                    Event::FilterGraphInitialize => unsafe {
                        let info = &*(param1 as *const FilterGraphInfo);
                        plugin.on_filter_graph_initialize(info);
                        0
                    }
                    Event::FilterGraphInitialized => unsafe {
                        let info = &*(param1 as *const FilterGraphInfo);
                        plugin.on_filter_graph_initialized(info);
                        0
                    }
                    Event::FilterGraphFinalize => unsafe {
                        let info = &*(param1 as *const FilterGraphInfo);
                        plugin.on_filter_graph_finalize(info);
                        0
                    }
                    Event::FilterGraphFinalized => unsafe {
                        let info = &*(param1 as *const FilterGraphInfo);
                        plugin.on_filter_graph_finalized(info);
                        0
                    }
                    Event::DrawCommandIcon => unsafe {
                        let info = &*(param1 as *const DrawCommandIconInfo);
                        plugin.on_draw_command_icon(info) as isize
                    }
                    Event::StatusItemDraw => unsafe {
                        let info = &*(param1 as *const StatusItemDrawInfo);
                        plugin.on_status_item_draw(info) as isize
                    }
                    Event::StatusItemNotify => unsafe {
                        let info = &*(param1 as *const StatusItemEventInfo);
                        plugin.on_status_item_notify(info) as isize
                    }
                    Event::StatusItemMouse => unsafe {
                        let info = &*(param1 as *const StatusItemMouseEventInfo);
                        plugin.on_status_item_mouse_event(info) as isize
                    }
                    Event::PanelItemNotify => unsafe {
                        let info = &*(param1 as *const PanelItemEventInfo);
                        plugin.on_panel_item_notify(info) as isize
                    }
                    Event::FavoritesChanged => {
                        plugin.on_favorites_changed();
                        0
                    }
                    Event::OneSegModeChanged => {
                        plugin.on_one_seg_mode_changed(param1 != 0);
                        0
                    }
                    Event::GetVariable => unsafe {
                        let info = &*(param1 as *const GetVariableInfo);
                        plugin.on_get_variable(info) as isize
                    },
                    Event::Trailer => {
                        0
                    }
                };

                LRESULT(result)
            } else {
                panic!("__UNSAFE_PLUGIN__ has not initialized yet.")
            }
        }
    }
}
