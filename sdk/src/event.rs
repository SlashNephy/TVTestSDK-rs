use std::ffi::c_void;
use windows::Win32::Foundation::{LPARAM, LRESULT};

/// イベント用コールバック関数
pub type EventCallbackFunc = unsafe extern "system" fn(
    event: Event,
    param1: LPARAM,
    param2: LPARAM,
    client_data: *const c_void
) -> LRESULT;

/// イベント
/// 各イベント発生時のパラメータは CTVTestEventHadler を参照してください。
#[repr(u32)]
#[cfg_attr(test, derive(Debug))]
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
