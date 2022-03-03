fn main() {
    let a = multiplier(5, 10);
    println!("aは{}です。", a);
}

fn multiplier(x: i32, y: i32) -> i32 {
    let mut r = 0;
    while y > 0 {
        r += x;
        y -= 1;
    }
    r
}

// (base) PS D:\01_IT\07_Rust\atmarkit_rust\functions> cargo run --bin func4
//    Compiling functions v0.1.0 (D:\01_IT\07_Rust\atmarkit_rust\functions)
// error[E0384]: cannot assign to immutable argument `y`
//   --> src\bin\func4.rs:10:9
//    |
// 6  | fn multiplier(x: i32, y: i32) -> i32 {
//    |                       - help: consider making this binding mutable: `mut y`
// ...
// 10 |         y -= 1;
//    |         ^^^^^^ cannot assign to immutable argument
