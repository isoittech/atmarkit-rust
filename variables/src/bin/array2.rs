fn main() {
    let a = [1, 2, 3, 4, 5];
    let i = 6;
    let x = a[i];
    let y = a[1];
    let z = a[2];
    println!("x={}, y={}, z={}", x, y, z);
}

/*
(base) PS D:\01_IT\07_Rust\atmarkit_rust\variables> cargo run --bin array2
   Compiling variables v0.1.0 (D:\01_IT\07_Rust\atmarkit_rust\variables)
error: this operation will panic at runtime
 --> src\bin\array2.rs:4:13
  |
4 |     let x = a[i];
  |             ^^^^ index out of bounds: the length is 5 but the index is 6
  |
  = note: `#[deny(unconditional_panic)]` on by default
*/

// コンパイル時にインデックスの値が範囲外だったため、実行せずにエラーとしています。
// しかし、変数iの値が実行時でないと決まらない場合には、
// 実行時にインデックスのチェックが実施されます。もし、
// インデックスが範囲外であれば、Rustはpanicという形でエラーを返し、
// プログラムを終了させる。
