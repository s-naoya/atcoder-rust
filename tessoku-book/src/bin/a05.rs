use proconio::input;
fn main() {
    input! {(n, k): (i32, i32)}
    let mut ans = 0;
    for i in 1..=n as i32 {
        for j in 1..=n as i32 {
            let l = k - i - j;
            if 1 <= l && l <= n {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
