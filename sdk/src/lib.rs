use std::ffi::c_void;
use windows::Win32::Foundation::HWND;
use crate::api::PluginApi;
use crate::command::DrawCommandIconInfo;
use crate::filter_graph::FilterGraphInfo;
use crate::panel::PanelItemEventInfo;
use crate::plugin::PluginInfo;
use crate::program_guide::{ProgramGuideCommandParam, ProgramGuideInitializeMenuInfo, ProgramGuideProgramDrawBackgroundInfo, ProgramGuideProgramInfo, ProgramGuideProgramInitializeMenuInfo};
use crate::record::{RecordStatus, StartRecordInfo};
use crate::status_item::{StatusItemDrawInfo, StatusItemEventInfo, StatusItemMouseEventInfo};
use crate::stereo_mode::StereoMode;
use crate::variable::GetVariableInfo;
use crate::version::{DEFAULT_API_VERSION, Version};
use crate::win32::WideStringPtr;

pub mod version;
pub mod message;
pub mod event;
pub mod channel;
pub mod log;
pub mod record;
pub mod program_guide;
pub mod filter_graph;
pub mod status_item;
pub mod panel;
pub mod command;
pub mod variable;
pub mod service;
pub mod tuning_space;
pub mod pan_scan;
pub mod status;
pub mod stereo_mode;
pub mod reset;
pub mod close;
pub mod stream;
pub mod arib_string;
pub mod program;

pub mod api;
pub mod plugin;
pub mod interface;
#[macro_use]
pub mod export;
pub mod win32;

#[macro_use]
pub extern crate enumflags2;
#[macro_use]
pub extern crate num_enum;
pub extern crate windows;

pub type ClientData = *mut c_void;

/// すべての TVTest プラグイン構造体が実装すべき trait
pub trait TVTestPlugin: TVTestEventHandler {
    fn new(api: PluginApi) -> Self;
    fn get_api_version() -> Version { DEFAULT_API_VERSION }
    fn get_info() -> PluginInfo;

    fn initialize(&self) -> bool { true }
    fn finalize(&self) -> bool { true }
}

#[allow(unused_variables)]
pub trait TVTestEventHandler {
    /// 有効状態が変化した
    /// 変化を拒否する場合 false を返します
    fn on_plugin_enable(&self, is_enable: bool) -> bool { false }
    /// 設定を行う
    /// プラグインのフラグに PLUGIN_FLAG_HASSETTINGS が設定されている場合に呼ばれます
    /// 設定が OK されたら true を返します
    fn on_plugin_settings(&self, owner: HWND) -> bool { false }
    /// チャンネルが変更された
    fn on_channel_change(&self) -> bool { false }
    /// サービスが変更された
    fn on_service_change(&self) -> bool { false }
    /// ドライバが変更された
    fn on_driver_change(&self) -> bool { false }
    /// サービスの構成が変化した
    fn on_service_update(&self) -> bool { false }
    /// 録画状態が変化した
    fn on_record_status_change(&self, status: RecordStatus) -> bool { false }
    /// 全画面表示状態が変化した
    fn on_fullscreen_change(&self, is_fullscreen: bool) -> bool { false }
    /// プレビュー表示状態が変化した
    fn on_preview_change(&self, is_preview: bool) -> bool { false }
    /// 音量が変化した
    fn on_volume_change(&self, volume: i32, is_mute: bool) -> bool { false }
    /// ステレオモードが変化した
    fn on_stereo_mode_change(&self, stereo_mode: StereoMode) -> bool { false }
    /// 色の設定が変化した
    fn on_color_change(&self) -> bool { false }

    /// 待機状態が変化した
    fn on_standby(&self, is_standby: bool) -> bool { false }
    /// コマンドが選択された
    fn on_command(&self, id: i32) -> bool { false }

    /// 複数起動禁止時に複数起動された
    fn on_execute(&self, command_line: WideStringPtr) -> bool { false }

    /// リセットされた
    fn on_reset(&self) -> bool { false }
    /// ステータス(MESSAGE_GETSTUATUSで取得できる内容)がリセットされた
    fn on_status_reset(&self) -> bool { false }
    /// 音声ストリームが変更された
    fn on_audio_stream_change(&self, stream: i32) -> bool { false }

    /// 設定が変更された
    fn on_settings_change(&self) -> bool { false }

    /// TVTestのウィンドウが閉じられる
    fn on_close(&self) -> bool { false }
    /// 録画が開始される
    fn on_start_record(&self, info: &StartRecordInfo) -> bool { false }
    /// 録画ファイルの切り替えが行われた
    fn on_relay_record(&self, file_name: WideStringPtr) -> bool { false }

    /// コントローラの対象の設定
    fn on_controller_focus(&self, hwnd: HWND) -> bool { false }

    /// 起動処理が終了した
    fn on_startup_done(&self) {}
    /// 番組表の初期化
    fn on_program_guide_initialize(&self, hwnd: HWND) -> bool { true }
    /// 番組表の終了
    fn on_program_guide_finalize(&self, hwnd: HWND) -> bool { true }
    /// 番組表のコマンドの実行
    fn on_program_guide_command(&self, command: u32, param: &ProgramGuideCommandParam) -> bool { false }
    /// 番組表のメニューの初期化
    fn on_program_guide_initialize_menu(&self, info: &ProgramGuideInitializeMenuInfo) -> i32 { 0 }
    /// 番組表のメニューが選択された
    fn on_program_guide_menu_selected(&self, command: u32) -> bool { false }
    /// 番組表の番組の背景描画
    fn on_program_guide_program_draw_background(&self, program_info: &ProgramGuideProgramInfo, info: &ProgramGuideProgramDrawBackgroundInfo) -> bool { false }
    /// 番組表の番組のメニュー初期化
    fn on_program_guide_program_initialize_menu(&self, program_info: &ProgramGuideProgramInfo, info: &ProgramGuideProgramInitializeMenuInfo) -> i32 { 0 }
    /// 番組表の番組のメニューが選択された
    fn on_program_guide_program_menu_selected(&self, program_info: &ProgramGuideProgramInfo, command: u32) -> bool { false }

    /// フィルタグラフの初期化
    fn on_filter_graph_initialize(&self, info: &FilterGraphInfo) {}
    /// フィルタグラフが初期化された
    fn on_filter_graph_initialized(&self, info: &FilterGraphInfo) {}
    /// フィルタグラフの終了処理
    fn on_filter_graph_finalize(&self, info: &FilterGraphInfo) {}
    /// フィルタグラフが終了処理された
    fn on_filter_graph_finalized(&self, info: &FilterGraphInfo) {}
    /// コマンドアイコンの描画
    fn on_draw_command_icon(&self, info: &DrawCommandIconInfo) -> bool { false }
    /// ステータス項目の描画
    fn on_status_item_draw(&self, info: &StatusItemDrawInfo) -> bool { false }
    /// ステータス項目の通知
    fn on_status_item_notify(&self, info: &StatusItemEventInfo) -> bool { false }
    /// ステータス項目のマウスイベント
    fn on_status_item_mouse_event(&self, info: &StatusItemMouseEventInfo) -> bool { false }
    /// パネル項目の通知
    fn on_panel_item_notify(&self, info: &PanelItemEventInfo) -> bool { false }
    /// お気に入りチャンネルが変更された
    fn on_favorites_changed(&self) {}
    /// ワンセグモードが変わった
    fn on_one_seg_mode_changed(&self, mode: bool) {}
    /// 変数を取得
    fn on_get_variable(&self, info: &GetVariableInfo) -> bool { false }
}
