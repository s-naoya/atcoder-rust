use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, mut d: [usize; n]}
    d.sort_unstable();
    let ans = d[d.len() / 2] - d[d.len() / 2 - 1];
    println!("{}", ans);
}
// 3:25
