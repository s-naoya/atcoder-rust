use proconio::input;
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; k]
    }
    let mut dp = vec![false; n + 1];
    for i in 0..=n {
        for aj in &a {
            if i >= *aj && !dp[i - *aj] {
                dp[i] = true;
                break;
            }
        }
    }
    println!("{}", if dp[n] { "First" } else { "Second" });
}
