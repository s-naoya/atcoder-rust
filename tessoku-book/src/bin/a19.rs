use proconio::input;
fn main() {
    input! {
        (n, w): (usize, usize),
        wv: [(usize, i64); n],
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![-1_000_000_000_000_000; w + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=w {
            // println!("{} {} {:?}", i, j, dp[i]);
            dp[i][j] = if j < wv[i - 1].0 {
                dp[i - 1][j]
            } else {
                std::cmp::max(dp[i - 1][j], dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1)
            }
        }
    }

    let mut max = 0;
    for j in 0..=w {
        max = std::cmp::max(max, dp[n][j]);
    }
    println!("{}", max);
}
