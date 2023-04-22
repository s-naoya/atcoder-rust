use proconio::input;
fn main() {
    input! {
        (n, a, b): (usize, usize, usize)
    }
    let mut dp = vec![false; n + 1];
    for i in 0..=n {
        dp[i] = if i >= a && !dp[i - a] {
            true
        } else if i >= b && !dp[i - b] {
            true
        } else {
            false
        }
    }
    println!("{}", if dp[n] { "First" } else { "Second" });
}
