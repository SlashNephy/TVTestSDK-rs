use tvtest::api::PluginApi;
use tvtest::plugin::PluginInfo;
use tvtest::version::{DEFAULT_API_VERSION, Version};

// すべての TVTest プラグイン構造体が実装すべき trait
pub trait TVTestPlugin {
    fn new(api: PluginApi) -> Self;
    fn get_api_version() -> Version { DEFAULT_API_VERSION }
    fn get_info() -> PluginInfo;

    fn initialize(&self) -> bool { true }
    fn finalize(&self) -> bool { true }
}

// TVTEST_PLUGIN_CLASS_IMPLEMENT を再現するマクロ
#[macro_export]
macro_rules! export_plugin {
    ($type: path) => {
        static mut __UNSAFE_DLL__: Option<std::sync::Arc<windows::Win32::Foundation::HINSTANCE>> = None;
        static mut __UNSAFE_PLUGIN__: Option<std::sync::Arc<$type>> = None;

        // エントリポイント
        // プラグインクラスのインスタンスの生成と破棄を行っています
        #[no_mangle]
        pub unsafe extern "system" fn DllMain(dll: windows::Win32::Foundation::HINSTANCE, reason: DllLoadReason, _reserved: *mut std::ffi::c_void) -> bool {
            match reason {
                DllLoadReason::ProcessAttach => {
                    __UNSAFE_DLL__ = std::sync::Arc::new(dll).into();
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
        pub extern "system" fn TVTGetVersion() -> u32 {
            <$type>::get_api_version().into()
        }

        // プラグインの情報を取得する
        // TVTGetVersion の次に呼ばれるので、プラグインの情報を PluginInfo 構造体に設定します。
        // FALSE が返された場合、すぐにアンロードされます。
        #[no_mangle]
        pub extern "system" fn TVTGetPluginInfo(mut info: tvtest::plugin::PluginInfo) -> bool {
            let new_info = <$type>::get_info();
            info.types = new_info.types;
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
        pub unsafe extern "system" fn TVTInitialize(param: tvtest::plugin::PluginParam) -> bool {
            if let Some(dll) = &__UNSAFE_DLL__ {
                let param = std::sync::Arc::new(param);
                let api = tvtest::api::PluginApi {
                    dll: std::sync::Arc::clone(dll),
                    param: std::sync::Arc::clone(&param),
                };
                let plugin = <$type>::new(api);

                let result = plugin.initialize();
                __UNSAFE_PLUGIN__ = std::sync::Arc::new(plugin).into();

                result
            } else {
                panic!("_DLL has not initialized yet.")
            }
        }

        // 終了処理を行う
        // プラグインがアンロードされる前に呼ばれるので、終了処理を行います。
        // この関数が呼ばれるのは TVTInitialize 関数が TRUE を返した場合だけです。
        #[no_mangle]
        pub unsafe extern "system" fn TVTFinalize() -> bool {
            if let Some(plugin) = &__UNSAFE_PLUGIN__ {
                let result = plugin.as_ref().finalize();
                __UNSAFE_PLUGIN__ = None;

                result
            } else {
                panic!("PLUGIN has not initialized yet.")
            }
        }

        #[repr(u32)]
        pub enum DllLoadReason {
            ProcessAttach = 1,
            ThreadAttach = 2,
            ThreadDetach = 3,
            ProcessDetach = 0,
        }
    }
}
