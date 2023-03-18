use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }
    s.insert(0, ' ');
    t.insert(0, ' ');
    let mut dp = vec![vec![0; t.len()]; s.len()];
    for i in 1..s.len() {
        for j in 0..t.len() {
            dp[i][j] = if i >= 1 && j >= 1 && s[i] == t[j] {
                max(max(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1] + 1)
            } else if i >= 1 && j >= 1 {
                max(dp[i - 1][j], dp[i][j - 1])
            } else if i >= 1 {
                dp[i - 1][j]
            } else if j >= 1 {
                dp[i][j - 1]
            } else {
                0
            }
        }
        // println!("{:?}", dp[i]);
    }
    println!("{}", dp[s.len() - 1][t.len() - 1]);
}
