use crate::plugin::*;
use crate::wchar::*;

pub mod plugin;
pub mod wchar;

#[no_mangle]
pub extern "system" fn TVTGetVersion() -> u32 {
    TVTEST_PLUGIN_VERSION
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn TVTGetPluginInfo(mut info: PluginInfo) -> bool {
    // プラグインの情報を返す
    info.types = PluginType::Normal;
    info.flags = PluginFlag::Normal;
    info.name = "サンプル".as_pwstr();
    info.copyright = "Copyright(c) 2010 Taro Yamada".as_pwstr();
    info.description = "何もしないプラグイン".as_pwstr();
    return true;	// false を返すとプラグインのロードが失敗になる
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn TVTInitialize(mut param: PluginParam) -> bool {
    // ここで初期化を行う
    // 何もしないのであればオーバーライドしなくても良い
    return true;	// false を返すとプラグインのロードが失敗になる
}

#[no_mangle]
pub extern "system" fn TVTFinalize() -> bool {
    // ここでクリーンアップを行う
    // 何もしないのであればオーバーライドしなくても良い
    return true;
}
