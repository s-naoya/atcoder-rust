use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(n, mut x): (usize, i64), mut a: [i64; n]}
    a.sort_unstable();
    let mut ans = 0;
    for i in 0..n {
        if x - a[i] >= 0 {
            if x - a[i] > 0 && i == n - 1 {
                break;
            }
            x -= a[i];
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
// 14:48
