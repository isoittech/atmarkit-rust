:toc:

# VSCodeにおけるRustアプリのテンプレートプロジェクト

## ■ 背景・目的
- Rustのお勉強のため
- Windowsアプリをさくっと作りたいため

## ■ 成果物
- 本プロジェクトをCloneして、VSCodeでﾁｮﾁｮｯと設定を編集したら、Windowsアプリが出来る、ようなそんな雛形的Rustプロジェクト。

## ■ 辿った軌跡

### ▼ Microsoft C++ build tools のインストール
[%hardbreaks]
https://visualstudio.microsoft.com/visual-cpp-build-tools/
から「ダウンロード」リンクをクリックして遷移したページで、自動的にDLが開始される。
それを実行して、ウィザードに従い、インストールする。


### ▼ Rustコンパイル のインストール

. 下記にアクセス +
https://rustup.rs/

. rustup‑init.exe ファイルをDLクリックして実行、インストールを開始する +
下記選択肢は1（Default）を選択する。
+
....
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
....

. PATHに `%USERPROFILE%\.cargo\bin` を指定する。


### ▼ VSCode拡張機能のインストール

. 拡張機能「Rust」 +
普通に入れる。

. 拡張機能「Rustfmt」 +
普通に入れる。

### ▼ お試しプログラム（コンソール出力ONLY）

```Rust
# src/main.rs

fn main() {
   println!("Hello World!");
}
```

```toml
# Cargo.toml
[package]
name # "hello_world"
version # "0.0.1"
```

書いたら `Shift + Ctrl + B` で実行し、 `Rust: Cargo build` を選択。

### ▼ お試しプログラムの自動生成

`cargo new HelloWorld` というコマンドで下記資材を作成してくれるらしい。

- HelloWorldフォルダ
- .git/
- .gitignore
- Rustプログラム
+
```rust
fn main() {
    println!("Hello, world!");
}
```
- 設定ファイル
+
```toml
[package]
name # "HelloWorld"
version # "0.1.0"
authors # ["*******"]
edition # "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```




## ■ 参考

* https://note.com/marupeke296/n/n5e4e4502ae21[WindowsのVS Codeに最低限のRust開発環境を構築して「Hello World」！]
* https://qiita.com/osanshouo/items/7966ecbd41bc3ce611dd[[Rust\] web-viewでGUIアプリをつくる]