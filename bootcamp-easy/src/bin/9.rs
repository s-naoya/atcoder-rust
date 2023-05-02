use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n]
    }
    let mut ans = 0;
    for xi in x {
        ans += 2 * std::cmp::min(xi, k - xi);
    }
    println!("{}", ans);
}
// 6:30
