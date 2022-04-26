# TVTestSDK-rs
TVTest SDK binding for Rust lang

## これはなに？

TVTest プラグインを Rust で開発できるようにするための TVTestSDK の Rust バインディングです。  
Win32 なオブジェクトは `windows-rs` クレートを参照します。

## 構成

- `sdk/`  
  ヘッダーファイルを移植中です。
- `example-sdk/`
  サンプル実装のプラグインです。
- `bondriver/`  
  ヘッダーファイルを移植中です。
- `example-bondriver/`
  サンプル実装の BonDriver です。
- `bindings/`  
  rust-bindgen で作成した試作品 (動きません...)

## 現状

- クラスによるプラグインの記述に対応 (example 以下)

- \*(const|mut)
- i32 <-> u32

- Rust の命名規則に従います  
  関数名・イベント名などが変更されています

- `#define` を移植するのが大変なので一旦無視します  
  TVTest の API バージョンは最新に準拠します
