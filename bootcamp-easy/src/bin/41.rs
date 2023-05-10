use std::cmp::min;

use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, p: [usize; n]}
    let mut a = vec![1_000_000_000; n];
    a[0] = p[0];
    for i in 1..n {
        a[i] = min(a[i - 1], p[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        if p[i] <= a[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
// 7:16
