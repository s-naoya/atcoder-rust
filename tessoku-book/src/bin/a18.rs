use proconio::input;
fn main() {
    input! {
        (n, s): (usize, usize),
        mut a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n + 1];

    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            if j < a[i - 1] {
                if dp[i - 1][j] {
                    dp[i][j] = true;
                }
            } else {
                if dp[i - 1][j] || dp[i - 1][j - a[i - 1]] {
                    dp[i][j] = true;
                }
            }
        }
    }
    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No");
    }
}
