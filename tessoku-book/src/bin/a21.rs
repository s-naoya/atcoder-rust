use proconio::input;
fn main() {
    input! {
        n: usize,
        mut pn: [(usize, usize); n],
    }
    pn.insert(0, (0, 0));
    pn.push((0, 0));
    let pn = pn;
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for len in (0..=(n - 2)).rev() {
        for l in 1..=(n - len) {
            let r = l + len;
            let score1 = if l <= pn[l - 1].0 && pn[l - 1].0 <= r {
                pn[l - 1].1
            } else {
                0
            };
            let score2 = if l <= pn[r + 1].0 && pn[r + 1].0 <= r {
                pn[r + 1].1
            } else {
                0
            };

            dp[l][r] = if l == 1 {
                dp[l][r + 1] + score2
            } else if r == n {
                dp[l - 1][r] + score1
            } else {
                std::cmp::max(dp[l - 1][r] + score1, dp[l][r + 1] + score2)
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = std::cmp::max(ans, dp[i][i]);
    }
    println!("{}", ans);
}
