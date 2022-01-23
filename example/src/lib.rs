use tvtest::api::PluginApi;
use tvtest::event_handler::TVTestEventHandler;
use tvtest::export_plugin;
use tvtest::plugin::{PluginFlag, PluginInfo, PluginType, TVTestPlugin};
use tvtest::version::{DEFAULT_API_VERSION, Version};

// [必須]
// プラグイン構造体を定義します
// ::new(...) でコンストラクタを実装します
#[allow(dead_code)]
pub struct ExamplePlugin {
    // TVTest の機能にアクセスするための構造体です
    api: PluginApi,

    // 任意のフィールドを追加できます
    foo: u32,
    bar: String,
    hoge: i32,
}

// [必須]
// プラグイン構造体に TVTestPlugin trait を実装します
impl TVTestPlugin for ExamplePlugin {
    // [必須]
    // プラグイン構造体のインスタンスを生成します
    // フィールドの初期化処理のみを記述します
    // プラグインの初期化処理などは fn initialize(&self) で実装します
    fn new(api: PluginApi) -> Self {
        ExamplePlugin {
            api,
            foo: 10,
            bar: "".to_string(),
            hoge: 334
        }
    }

    // プラグインの API バージョンを返します
    // 明示的に実装しない場合、デフォルトのバージョンを返します
    // 本体の API バージョンより新しい場合はロードされません
    fn get_api_version() -> Version {
        DEFAULT_API_VERSION
    }

    // [必須]
    // プラグインの情報を返します
    // ワイド文字列と String との変換は自動で行われます
    fn get_info() -> PluginInfo {
        PluginInfo {
            types: PluginType::Normal,
            flags: PluginFlag::Normal,
            name: "Example".to_string(),
            copyright: "© 2021 @SlashNephy <spica@starry.blue>".to_string(),
            description: "TVTestSDK-rs のサンプルプラグイン".to_string(),
        }
    }

    // 初期化を行います
    // 明示的に実装しない場合、何もしません
    // false を返すとプラグインのロードが失敗になります
    fn initialize(&self) -> bool {
        // ログを書き込みます
        // self.api には TVTest の API を呼び出すためのメソッドが実装されています
        self.api.add_log("プラグインを読み込みました！".to_string());

        true
    }

    // クリーンアップを行います
    // 明示的に実装しない場合、何もしません
    // false を返すとプラグインのアンロードが失敗になります
    fn finalize(&self) -> bool {
        true
    }
}

// [必須]
// プラグイン構造体に TVTestEventHandler trait を実装します
// 各種イベントが発生した際に呼び出されます
// 明示的に実装しない場合、何もしません
impl TVTestEventHandler for ExamplePlugin {
    fn on_channel_change(&self) -> bool {
        self.api.add_log("channel changed.".to_string());
        true
    }
}

// [必須]
// C言語形式の関数をエクスポートします
// 生成された target/example.dll の拡張子を .tvtp に変更すると TVTest でロードできるようになります
export_plugin!(ExamplePlugin);
