use windows::Win32::Foundation::{LPARAM, LRESULT};
use crate::plugin::PluginParam;

/// メッセージ送信用コールバック関数
pub type MessageCallbackFunc = unsafe extern "system" fn(
    param: *const PluginParam,
    message: Message,
    param1: LPARAM,
    param2: LPARAM,
) -> LRESULT;

/// メッセージ
#[repr(u32)]
#[cfg_attr(test, derive(Debug))]
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

impl PluginParam {
    #[inline]
    pub fn send_message(&self, message: Message, param1: LPARAM, param2: LPARAM) -> LRESULT {
        unsafe {
            (self.callback)(self, message, param1, param2)
        }
    }

    #[inline]
    pub fn send_message_bool(&self, message: Message, param1: LPARAM, param2: LPARAM) -> bool {
        self.send_message(message, param1, param2).0 != 0
    }
}
