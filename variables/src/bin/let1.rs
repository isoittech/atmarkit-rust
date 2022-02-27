fn main() {
    let x = 100;
    // println!()はマクロで、1番目の引数の書式に従い、2番目以降の値を埋め込んで出力
    println!("{}", x);
    x = 200;
    println!("{}", x);

    /*
    (base) PS D:\01_IT\07_Rust\atmarkit_rust\variables> cargo run --bin let1
       Compiling variables v0.1.0 (D:\01_IT\07_Rust\atmarkit_rust\variables)
    error[E0384]: cannot assign twice to immutable variable `x`
     --> src\bin\let1.rs:5:5
      |
    2 |     let x = 100;
      |         -
      |         |
      |         first assignment to `x`
      |         help: consider making this binding mutable: `mut x`
    ...
    5 |     x = 200;
      |     ^^^^^^^ cannot assign twice to immutable variable

    For more information about this error, try `rustc --explain E0384`.
    error: could not compile `variables` due to previous error
    */

    // 最初の代入では問題ないですが、2回目の代入で「cannot assign twice to immutable variable」というエラーが発生しています。
    // immutableとは「不変の」という意味で、「不変の変数に2回目の代入はできません」と書かれています。
    // Rustの変数は、変数と言いながら、デフォルトでは値の変更ができません（不変であるとも言います）。
    // 実は、これがRustの安全性を高める仕組みの一つです。不変の変数という言葉は変ですが「とにかくそういうことはできないんだ」と思ってください。
}
