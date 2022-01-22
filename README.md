# TVTestSDK-rs
TVTest SDK binding for Rust lang

## これはなに？

TVTest プラグインを Rust で開発できるようにするための TVTestSDK の Rust バインディングです。  
Win32 なオブジェクトは `windows-rs` クレートを参照します。

## 構成

- `sdk/`  
  ヘッダーファイルを移植中です。
- `sdk-macro/`
  Cの関数エクスポート用のマクロ定義があります。
- `example/`
  サンプルプラグイン実装です。
- `bindings/`  
  rust-bindgen で作成した試作品 (動きません...)

## 現状

- クラスによるプラグインの記述に対応 (example 以下)

- Rust の命名規則に従います  
  関数名・イベント名などが変更されています

- `#define` を移植するのが大変なので一旦無視します  
  TVTest の API バージョンは最新に準拠します
