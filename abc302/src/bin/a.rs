use proconio::input;
fn main() {
    // input! {(a, b): (u128, u128)}
    input! {(a, b): (i64, i64)}
    // let ans = if a % b != 0 { a / b + 1 } else { a / b };
    let ans = (a + b - 1) / b;
    println!("{}", ans);
}
