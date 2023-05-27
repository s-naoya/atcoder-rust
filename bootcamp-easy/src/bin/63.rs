use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, a: [usize; n]}
    let mut ans = 0;
    let mut cnt = 1;
    for i in 0..n {
        if a[i] == cnt {
            cnt += 1;
        } else {
            ans += 1;
        }
    }
    if ans == n {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
// 5:17
