use itertools::Itertools;
use std::vec;

use proconio::input;
fn main() {
    input! {
        (n, s): (usize, usize),
        a: [usize; n],
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
    // println!("{:?}", dp);
    if !dp[n][s] {
        println!("-1");
        return;
    }
    let mut nums = Vec::new();
    let mut j = s;
    for i in (1..=n).rev() {
        // println!("{} {} {:?}", i, j, nums);
        if j >= a[i - 1] && dp[i - 1][j - a[i - 1]] {
            nums.push(i);
            j -= a[i - 1];
            continue;
        }
        if dp[i - 1][j] {
            continue;
        }
    }
    nums.reverse();
    println!("{}\n{}", nums.len(), nums.iter().join(" "));
}
