use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(k, n): (usize, usize), a: [usize; n]}
    let mut max = k - a[n - 1] + a[0];
    for i in 1..n {
        max = std::cmp::max(max, a[i] - a[i - 1]);
    }
    println!("{}", k - max);
}
// 7:55
