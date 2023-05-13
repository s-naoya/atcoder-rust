use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(a, b, _, k): (i128, i128, i128, i128)}
    if (a - b).abs() > 1_000_000_000_000_000_000 {
        println!("Unfair");
        return;
    }
    let ans = if k % 2 == 0 { a - b } else { b - a };
    println!("{}", ans);
}
// 19:57
