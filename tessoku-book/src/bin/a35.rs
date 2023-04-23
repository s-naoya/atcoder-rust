use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut dp = Vec::new();
    for i in 0..n - 1 {
        dp.push(vec![0; i + 1]);
    }
    dp.push(a.clone());

    for i in (0..n - 1).rev() {
        let is_max = match i % 2 {
            0 => true,
            _ => false,
        };
        for j in 0..=i {
            dp[i][j] = if is_max {
                std::cmp::max(dp[i + 1][j], dp[i + 1][j + 1])
            } else {
                std::cmp::min(dp[i + 1][j], dp[i + 1][j + 1])
            }
        }
    }
    println!("{}", dp[0][0]);
}
