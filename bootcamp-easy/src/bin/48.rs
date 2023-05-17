use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, h: [usize; n]}
    let mut ans = 0;
    let mut now = 0;
    for i in 0..n - 1 {
        if h[i] >= h[i + 1] {
            now += 1;
        } else {
            ans = ans.max(now);
            now = 0;
        }
    }
    ans = ans.max(now);
    println!("{}", ans);
}
// 3:30
