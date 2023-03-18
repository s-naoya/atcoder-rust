use proconio::input;
fn main() {
    input! {
        (n, w): (usize, i64),
        mut wv: [(i64, usize); n],
    }
    wv.insert(0, (0, 0));
    let vmax = 100 * 1000;
    let mut dp: Vec<Vec<i64>> = vec![vec![1_000_000_000_000_000; vmax + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=vmax {
            dp[i][j] = if j < wv[i].1 {
                dp[i - 1][j]
            } else {
                std::cmp::min(dp[i - 1][j], dp[i - 1][j - wv[i].1] + wv[i].0)
            }
        }
        // println!("{:?}", dp[i]);
    }
    // println!("{:?}", dp[n]);
    let mut max = 0;
    for j in 0..=vmax {
        if dp[n][j] <= w {
            max = j;
        }
    }
    println!("{}", max);
}
