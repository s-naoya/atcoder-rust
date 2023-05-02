use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(n, k): (i128, i128)}
    let ans = std::cmp::min(n % k, (n % k - k).abs());
    println!("{}", ans);
}
// 3:15
