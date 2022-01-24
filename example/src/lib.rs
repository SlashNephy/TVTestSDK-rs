use enumflags2::BitFlag;
use tvtest::api::PluginApi;
use tvtest::{export_plugin, TVTestEventHandler, TVTestPlugin};
use tvtest::plugin::{PluginFlag, PluginInfo, PluginKind};
use tvtest::version::{DEFAULT_API_VERSION, Version};
use tvtest::win32::{IntoRustString, UnsafeIntoRustString, WideStringPtr};

/// **必須**
/// プラグイン構造体を定義します
/// `ExamplePlugin::new(...)` でコンストラクタを実装します
#[allow(dead_code)]
pub struct ExamplePlugin {
    /// TVTest の機能にアクセスするための構造体です
    api: PluginApi,

    /// 任意のフィールドを追加できます
    foo: u32,
    bar: String,
    hoge: i32,
}

/// **必須**
/// プラグイン構造体に `TVTestPlugin` trait を実装します
impl TVTestPlugin for ExamplePlugin {
    /// **必須**
    /// プラグイン構造体のインスタンスを生成します
    /// フィールドの初期化処理のみを記述します
    /// プラグインの初期化処理などは `fn initialize(&self) -> bool` で実装します
    fn new(api: PluginApi) -> Self {
        ExamplePlugin {
            api,
            foo: 10,
            bar: "".to_string(),
            hoge: 334
        }
    }

    /// プラグインの API バージョンを返します
    /// 明示的に実装しない場合、デフォルトのバージョンを返します
    /// 本体の API バージョンより新しい場合はロードされません
    fn get_api_version() -> Version {
        DEFAULT_API_VERSION
    }

    /// **必須**
    /// プラグインの情報を返します
    /// null 終端ワイド文字列 (`WideString` 構造体) と Rust 文字列 (`String`) との変換は内部で行われます
    fn get_info() -> PluginInfo {
        PluginInfo {
            kind: PluginKind::Normal,
            flags: PluginFlag::empty(),
            name: "Example".into(),
            copyright: "© 2021 @SlashNephy <spica@starry.blue>".into(),
            description: "TVTestSDK-rs のサンプルプラグイン".into(),
        }
    }

    /// 初期化を行います
    /// 明示的に実装しない場合、何もしません
    /// `false` を返すとプラグインのロードが失敗になります
    fn initialize(&self) -> bool {
        // ログを書き込みます
        // `self.api` には TVTest の API を呼び出すためのメソッドが多数実装されています
        self.api.add_log("プラグインを読み込みました！".to_string());

        true
    }

    /// クリーンアップを行います
    /// 明示的に実装しない場合、何もしません
    /// `false` を返すとプラグインのアンロードが失敗になります
    fn finalize(&self) -> bool {
        true
    }
}

/// **必須**
/// プラグイン構造体に `TVTestEventHandler` trait を実装します
/// 各種イベントが発生した際に呼び出されます
/// 不要な場合でも
/// `impl TVTestEventHandler for ExamplePlugin {}`
/// を記述する必要があります
impl TVTestEventHandler for ExamplePlugin {
    /// チャンネルが変更されたときに呼び出されます
    fn on_channel_change(&self) -> bool {
        // 現在のチャンネル情報を取得します
        // API の呼び出しは失敗する可能性があるため、`Option<T>` 型が返されます
        if let Some(channel) = self.api.get_current_channel_info() {
            self.api.add_log(
                // 固定長のワイド文字列 (`FixedWideString` 構造体) は `.into_string()` 関数で
                // Rust で扱える文字列に変換できます
                format!("Current Service = {:?}", channel.channel_name.into_string())
            );
        }

        true
    }

    fn on_execute(&self, command_line: WideStringPtr) -> bool {
        // ワイド文字列ポインタ (`WideStringPtr` 構造体) は `.read_string()` 関数で
        // Rust で扱える文字列に変換できます
        // ワイド文字列のポインタ操作など TVTest から渡されたポインタ先を読み取る操作は
        // 失敗する可能性があるため、`Option<T>` 型が返されます
        if let Some(cmd) = command_line.read_string() {
            self.api.add_log(format!("CommandLine = {}", cmd));
        }

        true
    }
}

// **必須**
// C言語形式の関数をエクスポートします
// 生成された target/example.dll の拡張子を .tvtp に変更すると TVTest でロードできるようになります
export_plugin!(ExamplePlugin);
