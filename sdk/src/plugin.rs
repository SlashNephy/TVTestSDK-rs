use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::copy;
use windows::Win32::Foundation::{HWND, PWSTR};
use windows::Win32::Globalization::lstrlenW;

// プラグインのバージョン
macro_rules! tvtest_plugin_version {
    ($major: expr, $minor: expr, $rev: expr) => {
        (($major) << 24) | (($minor) << 12) | ($rev)
    };
}

#[inline]
pub fn tvtest_plugin_version(major: u32, minor: u32, rev: u32) -> u32 {
    tvtest_plugin_version!(major, minor, rev)
}

pub const TVTEST_PLUGIN_VERSION: u32 = tvtest_plugin_version!(0, 0, 14);

/*
// エクスポート関数定義用
#define TVTEST_EXPORT(type) extern "C" __declspec(dllexport) type WINAPI

#ifdef offsetof
#define TVTEST_OFFSETOF offsetof
#else
#define TVTEST_OFFSETOF(type, member) \
	((size_t)((BYTE*)&((type*)0)->member-(BYTE*)(type*)0))
#endif

#ifdef interface
#define TVTEST_COM_INTERFACE interface
#else
#define TVTEST_COM_INTERFACE struct
#endif
 */

// プラグインの種類
#[repr(u32)]
pub enum PluginType {
    Normal  // 普通
}

// プラグインのフラグ
#[repr(u32)]
pub enum PluginFlag {
    Normal = 0x00000000,
    HasSettings = 0x00000001,       // 設定ダイアログがある
    EnableDefault = 0x00000002,     // デフォルトで有効
    DisableOnStart = 0x00000004,    // 起動時は必ず無効
    NoUnload = 0x00000008,          // 終了時以外アンロード不可
    NoEnabledDisabled = 0x00000010, // 有効/無効の区別がない, メニューに表示されず、Event::PluginEnable が送られません
}

// プラグインの情報
#[repr(C)]
pub struct PluginInfo
{
    // 種類 (PluginType)
    pub types: PluginType,
    // フラグ (PluginFlag)
    pub flags: PluginFlag,
    // プラグイン名
    pub name: PWSTR,
    // 著作権情報
    pub copyright: PWSTR,
    // 説明文
    pub description: PWSTR,
}

// メッセージ送信用コールバック関数
pub type MessageCallbackFunc = unsafe extern "stdcall" fn(
    param: *mut PluginParam,
    message: Message,
    param1: isize,
    param2: isize,
) -> isize;

// プラグインパラメータ
#[repr(C)]
pub struct PluginParam
{
    // コールバック関数
    pub callback: MessageCallbackFunc,
    // メインウィンドウのハンドル
    pub hwnd_app: HWND,
    // プラグイン側で好きに使えるデータ
    pub client_data: *mut c_void,
    // TVTest側で使用するデータ。アクセス禁止
    internal_data: *mut c_void,
}

/*
// エクスポート関数
typedef DWORD (WINAPI *GetVersionFunc)();
typedef BOOL (WINAPI *GetPluginInfoFunc)(PluginInfo *pInfo);
typedef BOOL (WINAPI *InitializeFunc)(PluginParam *pParam);
typedef BOOL (WINAPI *FinalizeFunc)();
 */

// メッセージ
#[repr(u32)]
pub enum Message {
    GetVersion,                  // プログラムのバージョンを取得
    QueryMessage,                // メッセージに対応しているか問い合わせる
    MemoryAlloc,                 // メモリ確保
    SetEventCallback,            // イベントハンドル用コールバックの設定
    GetCurrentChannelInfo,       // 現在のチャンネルの情報を取得
    SetChannel,                  // チャンネルを設定
    GetService,                  // サービスを取得
    SetService,                  // サービスを設定
    GetTuningSpaceName,          // チューニング空間名を取得
    GetChannelInfo,              // チャンネルの情報を取得
    GetServiceInfo,              // サービスの情報を取得
    GetDriverName,               // BonDriverのファイル名を取得
    SetDriverName,               // BonDriverを設定
    StartRecord,                 // 録画の開始
    StopRecord,                  // 録画の停止
    PauseRecord,                 // 録画の一時停止/再開
    GetRecord,                   // 録画設定の取得
    ModifyRecord,                // 録画設定の変更
    GetZoom,                     // 表示倍率の取得
    SetZoom,                     // 表示倍率の設定
    GetPanScan,                  // パンスキャンの設定を取得
    SetPanScan,                  // パンスキャンを設定
    GetStatus,                   // ステータスを取得
    GetRecordStatus,             // 録画ステータスを取得
    GetVideoInfo,                // 映像の情報を取得
    GetVolume,                   // 音量を取得
    SetVolume,                   // 音量を設定
    GetStereoMode,               // ステレオモードを取得
    SetStereoMode,               // ステレオモードを設定
    GetFullScreen,               // 全画面表示の状態を取得
    SetFullScreen,               // 全画面表示の状態を設定
    GetPreview,                  // 再生が有効か取得
    SetPreview,                  // 再生の有効状態を設定
    GetStandby,                  // 待機状態であるか取得
    SetStandby,                  // 待機状態を設定
    GetAlwaysOnTop,              // 常に最前面表示であるか取得
    SetAlwaysOnTop,              // 常に最前面表示を設定
    CaptureImage,                // 画像をキャプチャする
    SaveImage,                   // 画像を保存する
    Reset,                       // リセットを行う
    Close,                       // ウィンドウを閉じる
    SetStreamCallback,           // ストリームコールバックを設定
    EnablePlugin,                // プラグインの有効状態を設定
    GetColor,                    // 色の設定を取得
    DecodeARIBString,            // ARIB文字列のデコード
    GetCurrentProgramInfo,       // 現在の番組の情報を取得

    QueryEvent,                  // イベントに対応しているか取得
    GetTuningSpace,              // 現在のチューニング空間を取得
    GetTuningSpaceInfo,          // チューニング空間の情報を取得
    SetNextChannel,              // チャンネルを次に設定する

    GetAudioStream,              // 音声ストリームを取得
    SetAudioStream,              // 音声ストリームを設定
    IsPluginEnabled,             // プラグインの有効状態を取得
    RegisterCommand,             // コマンドの登録
    AddLog,                      // ログを記録
    ResetStatus,                 // ステータスを初期化
    SetAudioCallback,            // 音声のコールバック関数を設定
    DoCommand,                   // コマンドの実行
    Removed1,                    // (機能削除)
    Removed2,                    // (機能削除)
    GetHostInfo,                 // ホストプログラムの情報を取得
    GetSetting,                  // 設定の取得
    GetDriverFullPathName,       // BonDriverのフルパスを取得
    GetLogo,                     // ロゴの取得
    GetAvailableLogoType,        // 利用可能なロゴの取得
    RelayRecord,                 // 録画ファイルの切り替え
    SilentMode,                  // サイレントモードの取得/設定
    SetWindowMessageCallback,    // ウィンドウメッセージコールバックの設定
    RegisterController,          // コントローラの登録
    OnControllerButtonDown,      // コントローラのボタンが押されたのを通知
    GetControllerSettings,       // コントローラの設定を取得
    GetEPGEventInfo,             // 番組情報を取得
    FreeEPGEventInfo,            // 番組情報を解放
    GetEPGEventList,             // 番組のリストを取得
    FreeEPGEventList,            // 番組のリストを解放
    EnumDriver,                  // BonDriverの列挙
    GetDriverTuningSpaceList,    // BonDriverのチューニング空間のリストの取得
    FreeDriverTuningSpaceList,   // BonDriverのチューニング空間のリストの解放
    EnableProgramGuideEvent,     // 番組表のイベントの有効/無効を設定する
    RegisterProgramGuideCommand, // 番組表のコマンドを登録
    GetStyleValue,               // スタイル値を取得
    ThemeDrawBackground,         // テーマの背景を描画
    ThemeDrawText,               // テーマの文字列を描画
    ThemeDrawIcon,               // テーマのアイコンを描画
    GetEPGCaptureStatus,         // EPG取得状況を取得
    GetAppCommandInfo,           // コマンドの情報を取得
    GetAppCommandCount,          // コマンドの数を取得
    GetVideoStreamCount,         // 映像ストリームの数を取得
    GetVideoStream,              // 映像ストリームを取得
    SetVideoStream,              // 映像ストリームを設定
    GetLog,                      // ログを取得
    GetLogCount,                 // ログの数を取得
    RegisterPluginCommand,       // プラグインのコマンドを登録
    SetPluginCommandState,       // プラグインのコマンドの状態を設定
    PluginCommandNotify,         // プラグインのコマンドの通知
    RegisterPluginIcon,          // プラグインのアイコンを登録
    RegisterStatusItem,          // ステータス項目を登録
    SetStatusItem,               // ステータス項目の設定
    GetStatusItemInfo,           // ステータス項目の情報を取得
    StatusItemNotify,            // ステータス項目の通知
    RegisterTSProcessor,         // TSプロセッサの登録
    RegisterPanelItem,           // パネル項目を登録
    SetPanelItem,                // パネル項目の設定
    GetPanelItemInfo,            // パネル項目の情報を取得
    SelectChannel,               // チャンネルを選択する
    GetFavoriteList,             // お気に入りチャンネルを取得
    FreeFavoriteList,            // お気に入りチャンネルを解放
    GetOneSegMode,               // ワンセグモードを取得
    SetOneSegMode,               // ワンセグモードを設定
    GetDPI,                      // DPIを取得
    GetFont,                     // フォントを取得
    ShowDialog,                  // ダイアログを表示
    ConvertTime,                 // 日時を変換
    SetVideoStreamCallback,      // 映像ストリームのコールバック関数を設定
    GetVarStringContext,         // 変数文字列のコンテキストを取得
    FreeVarStringContext,        // 変数文字列のコンテキストを解放
    FormatVarString,             // 変数文字列を使って文字列をフォーマット
    RegisterVariable,            // 変数を登録
    Trailer
}

// イベント用コールバック関数
pub type EventCallbackFunc = unsafe extern "stdcall" fn(
    event: Event,
    param1: isize,
    param2: isize,
    client_data: *mut c_void
) -> isize;

// イベント
// 各イベント発生時のパラメータは CTVTestEventHadler を参照してください。
#[repr(u32)]
pub enum Event {
    PluginEnable,         // 有効状態が変化した
    PluginSettings,       // 設定を行う
    ChannelChange,        // チャンネルが変更された
    ServiceChange,        // サービスが変更された
    DriverChange,         // ドライバが変更された
    ServiceUpdate,        // サービスの構成が変化した
    RecordStatusChange,   // 録画状態が変化した
    FullScreenChange,     // 全画面表示状態が変化した
    PreviewChange,        // プレビュー表示状態が変化した
    VolumeChange,         // 音量が変化した
    StereoModeChange,     // ステレオモードが変化した
    ColorChange,          // 色の設定が変化した

    StandBy,              // 待機状態が変化した
    Command,              // コマンドが選択された

    Execute,              // 複数起動禁止時に複数起動された

    Reset,                // リセットされた
    StatusReset,          // ステータスがリセットされた
    AudioStreamChange,    // 音声ストリームが変更された

    SettingsChange,       // 設定が変更された

    Close,                // TVTestのウィンドウが閉じられる
    StartRecord,          // 録画が開始される
    RelayRecord,          // 録画ファイルが切り替えられた

    ControllerFocus,      // コントローラの対象を設定

    StartUpDone,          // 起動時の処理が終わった

    // 番組表関係のイベントは、Message:EnableProgramGuideEvent を呼んで有効にしないと通知されません
    ProgramGuideInitialize,             // 番組表の初期化
    ProgramGuideFinalize,               // 番組表の終了
    ProgramGuideCommand,                // 番組表のコマンド実行
    ProgramGuideInitializeMenu,         // 番組表のメニューの設定
    ProgramGuideMenuSelected,           // 番組表のメニューが選択された
    ProgramGuideProgramDrawBackground,  // 番組表の番組の背景を描画
    ProgramGuideProgramInitializeMenu,  // 番組表の番組のメニューの設定
    ProgramGuideProgramMenuSelected,    // 番組表の番組のメニューが選択された

    FilterGraphInitialize,              // フィルタグラフの初期化開始
    FilterGraphInitialized,             // フィルタグラフの初期化終了
    FilterGraphFinalize,                // フィルタグラフの終了処理開始
    FilterGraphFinalized,               // フィルタグラフの終了処理終了
    DrawCommandIcon,                    // コマンドアイコンの描画
    StatusItemDraw,                     // ステータス項目を描画
    StatusItemNotify,                   // ステータス項目の通知
    StatusItemMouse,                    // ステータス項目のマウス操作
    PanelItemNotify,                    // パネル項目の通知
    FavoritesChanged,                   // お気に入りチャンネルが変更された
    OneSegModeChanged,                  // ワンセグモードが変わった
    GetVariable,                        // 変数の取得

    Trailer
}

pub struct Version {
    pub major: u8,
    pub minor: u16,
    pub build: u16
}

impl Version {
    // バージョン番号を u32 にまとめる
    #[inline]
    pub fn make(major: u8, minor: u16, build: u16) -> u32 {
        (major << 24) as u32 | (minor << 12) as u32 | build as u32
    }

    // 上位8ビットがメジャーバージョン
    #[inline]
    pub fn get_major(version: u32) -> u8 {
        (version >> 24) as u8
    }

    // 次の12ビットがマイナーバージョン
    #[inline]
    pub fn get_minor(version: u32) -> u16 {
        ((version & 0x00FFF000) >> 12) as u16
    }

    // 下位12ビットがビルドナンバー
    #[inline]
    pub fn get_build(version: u32) -> u16 {
        (version & 0x00000FFF) as u16
    }
}

impl PluginParam {
    // プログラム(TVTest)のバージョンを取得する
    #[inline]
    pub fn get_version(&mut self) -> Version {
        let version = unsafe {
            (self.callback)(self, Message::GetVersion, 0, 0)
        } as u32;
        let major = Version::get_major(version);
        let minor = Version::get_minor(version);
        let build = Version::get_build(version);

        Version {
            major, minor, build
        }
    }

    // 指定されたメッセージに対応しているか問い合わせる
    #[inline]
    pub fn query_message(&mut self, message: Message) -> bool {
        let result = unsafe {
            (self.callback)(self, Message::QueryMessage, message as isize, 0)
        };

        result != 0
    }

    // メモリ再確保
    // pData が nullptr で新しい領域を確保
    // Size が0で領域を解放
    #[inline]
    pub fn memory_realloc(&mut self, data: *mut c_void, size: isize) -> *mut c_void {
        unsafe {
            let result = (self.callback)(self, Message::MemoryAlloc, data as isize, size);

            result as *mut c_void
        }
    }

    // メモリ確保
    #[inline]
    pub fn memory_alloc(&mut self, size: isize) -> *mut c_void {
        unsafe {
            let ptr: *mut c_void = std::ptr::null_mut();
            let result = (self.callback)(self, Message::MemoryAlloc, ptr as isize, size);

            result as *mut c_void
        }
    }

    // メモリ開放
    #[inline]
    pub fn memory_free(&mut self, data: *mut c_void) {
        unsafe {
            (self.callback)(self, Message::MemoryAlloc, data as isize, 0)
        };
    }

    // 文字列複製
    #[inline]
    pub unsafe fn string_duplicate(&mut self, string: PWSTR) -> PWSTR {
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
    #[inline]
    pub fn set_event_callback(&mut self, callback: EventCallbackFunc, client_data: *mut c_void) -> bool {
        let result = unsafe {
            (self.callback)(self, Message::SetEventCallback, callback as isize, client_data as isize)
        };

        result != 0
    }
}

#[repr(C)]
pub struct ChannelInfo {
    pub size: u32,                        // 構造体のサイズ
    pub space: i32,                       // チューニング空間(BonDriverのインデックス)
    pub channel: i32,                     // チャンネル(BonDriverのインデックス)
    pub remote_control_key_id: i32,       // リモコンID
    pub network_id: u16,                  // ネットワークID
    pub transport_stream_id: u16,         // トランスポートストリームID
    pub network_name: [u16; 32],          // ネットワーク名
    pub transport_stream_name: [u16; 32], // トランスポートストリーム名
    pub channel_name: [u16; 64],          // チャンネル名
    pub physical_channel: i32,            // 物理チャンネル番号(あまり信用できない)。不明の場合は0
    pub service_index: u16,               // サービスのインデックス(現在は意味を無くしているので使わない)
    pub service_id: u16,                  // サービスID
    // サービスはチャンネルファイルで設定されているものが取得される
    // サービスはユーザーが切り替えられるので、実際に視聴中のサービスがこれであるとは限らない
    // 実際に視聴中のサービスは MESSAGE_GETSERVICE で取得できる
    pub flags: ChannelFlag                // 各種フラグ
}

impl Default for ChannelInfo {
    fn default() -> ChannelInfo {
        ChannelInfo {
            size: size_of::<ChannelInfo>() as u32,
            space: 0,
            channel: 0,
            remote_control_key_id: 0,
            network_id: 0,
            transport_stream_id: 0,
            network_name: [0; 32],
            transport_stream_name: [0; 32],
            channel_name: [0; 64],
            physical_channel: 0,
            service_index: 0,
            service_id: 0,
            flags: ChannelFlag::Normal
        }
    }
}

#[repr(u32)]
pub enum ChannelFlag {
    Normal = 0x00000000,
    Disabled = 0x00000001  // 無効にされている
}

/*
#if TVTEST_PLUGIN_VERSION >= TVTEST_PLUGIN_VERSION_(0, 0, 1)
enum {
	CHANNELINFO_SIZE_V1 = TVTEST_OFFSETOF(ChannelInfo, PhysicalChannel)
#if TVTEST_PLUGIN_VERSION >= TVTEST_PLUGIN_VERSION_(0, 0, 12)
	, CHANNELINFO_SIZE_V2 = TVTEST_OFFSETOF(ChannelInfo, Flags)
#endif
};
#endif
 */

impl PluginParam {
    // 現在のチャンネルの情報を取得する
    #[inline]
    pub fn get_current_channel_info(&mut self) -> Option<ChannelInfo> {
        let info = ChannelInfo::default();
        let result = unsafe {
            let ptr = &info as *const _;
            (self.callback)(self, Message::GetCurrentChannelInfo, ptr as isize, 0)
        };

        if result == 0 {
            Some(info)
        } else {
            None
        }
    }

    // ログを記録する
    // 設定のログの項目に表示されます。
    #[inline]
    pub fn add_log<T: Into<Option<LogType>>>(&mut self, text: PWSTR, log_type: T) -> bool {
        let result = unsafe {
            let ptr = text.0 as *const u16;
            let log_type = match log_type.into() {
                Some(t) => t as isize,
                None => 0
            };

            (self.callback)(self, Message::AddLog, ptr as isize, log_type)
        };

        result != 0
    }
}

// ログの種類
pub enum LogType {
    Information, // 情報
    Warning,     // 警告
    Error,       // エラー
}
