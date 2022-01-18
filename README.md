# TVTestSDK-rs
TVTest SDK binding for Rust lang

## これはなに？

TVTest プラグインを Rust で開発できるようにするための TVTestSDK の Rust バインディングです。  
Win32 なオブジェクトは `windows-rs` クレートを参照します。

## 構成

- `sdk/`  
  ヘッダーファイルを移植中です。
- `bindings/`  
  rust-bindgen で作成した試作品 (動きません...)

## 現状

- クラスによるプラグインの記述には未対応
  クラスによる実装は `TVTEST_PLUGIN_CLASS_IMPLEMENT` を定義する方法です。  
  代わりに関数定義でプラグインを記述します。

- Rust の命名規則に従います  
  関数名・イベント名などが変更されています

- `#define` を移植するのが大変なので一旦無視します  
  TVTest の API バージョンは最新に準拠します
