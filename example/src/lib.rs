use tvtest::api::PluginApi;
use tvtest::plugin::{PluginFlag, PluginInfo, PluginType};
use tvtest::version::{DEFAULT_API_VERSION, Version};
use tvtest::wchar::ConvertToPWSTR;
use tvtest_macro::{export_plugin, TVTestPlugin};

// プラグイン構造体
#[allow(dead_code)]
pub struct ExamplePlugin {
    // TVTest の機能にアクセスするための構造体
    api: PluginApi,

    // 任意のフィールド
    foo: u32,
    bar: String,
    hoge: i32,
}

// プラグイン構造体に TVTestPlugin trait を実装する
impl TVTestPlugin for ExamplePlugin {
    // プラグイン構造体のインスタンスを生成する
    // フィールドの初期化処理のみを記述する
    // プラグインの初期化処理などは fn initialize(&self) で実装する
    fn new(api: PluginApi) -> Self {
        ExamplePlugin {
            api,
            foo: 10,
            bar: "".to_string(),
            hoge: 334
        }
    }

    // プラグインの API バージョンを返す
    // 明示的に実装しない場合、デフォルトのバージョンを返す
    fn get_api_version() -> Version {
        DEFAULT_API_VERSION
    }

    // プラグインの情報を返す
    fn get_info() -> PluginInfo {
        PluginInfo {
            types: PluginType::Normal,
            flags: PluginFlag::Normal,
            name: "Example".to_pwstr(),
            copyright: "© 2021 @SlashNephy <spica@starry.blue>".to_pwstr(),
            description: "TVTestSDK-rs のサンプルプラグイン".to_pwstr(),
        }
    }

    // 初期化を行う
    // 明示的に実装しない場合、何もしない
    // false を返すとプラグインのロードが失敗になる
    fn initialize(&self) -> bool {
        self.api.add_log("プラグインを読み込みました！".to_pwstr());

        true
    }

    // クリーンアップを行う
    // 明示的に実装しない場合、何もしない
    // false を返すとプラグインのアンロードが失敗になる
    fn finalize(&self) -> bool {
        true
    }
}

// C言語形式の関数をエクスポートする
// 生成された target/example.dll の拡張子を .tvtp に変更すると TVTest でロードできる
export_plugin!(ExamplePlugin);
