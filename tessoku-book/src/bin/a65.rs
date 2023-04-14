use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n-1]
    }
    let mut g = vec![vec![]; n + 1];
    for (i, ai) in a.iter().enumerate() {
        g[*ai].push(i + 2);
    }
    let mut dp = vec![0; n + 1];
    for i in (1..=n).rev() {
        dp[i] = g[i].len();
        for j in &g[i] {
            dp[i] += dp[*j];
        }
    }

    dp.remove(0);
    println!("{}", dp.iter().join(" "));
}
