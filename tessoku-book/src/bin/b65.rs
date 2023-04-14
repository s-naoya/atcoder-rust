use itertools::Itertools;
use proconio::input;

fn dfs(g: &Vec<Vec<usize>>, dp: &mut Vec<i32>, joushi: usize, buka: usize) -> i32 {
    // println!(
    //     "=== joushi: {}, buka: {}, g[buka]: {:?}",
    //     joushi, buka, g[buka]
    // );
    let mut max = -1;
    for buka_no_buka in &g[buka] {
        if *buka_no_buka == joushi {
            continue;
        }
        max = std::cmp::max(max, dfs(g, dp, buka, *buka_no_buka));
        // println!("-> {} max: {}", buka, max);
    }
    dp[buka] = max + 1;
    // println!("--> dp[{}]: {}", buka, dp[buka]);
    return dp[buka];
}

fn main() {
    input! {(n, t): (usize, usize)}
    let mut g = vec![vec![]; n + 1];
    for _ in 0..n - 1 {
        input! {(a, b): (usize, usize)}
        g[a].push(b);
        g[b].push(a);
    }

    let mut dp = vec![0; n + 1];
    dfs(&g, &mut dp, t, t);
    dp.remove(0);
    println!("{}", dp.iter().join(" "));
}
