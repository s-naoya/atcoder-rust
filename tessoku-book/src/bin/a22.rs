use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize; n-1],
        mut b: [usize; n-1]
    }
    a.insert(0, 0);
    b.insert(0, 0);
    // 遷移できないiからdp[a[i]]に遷移することを防ぐため、初期値を小さい値にする
    let mut dp = vec![std::i32::MIN; n + 1];
    dp[1] = 0;
    for i in 1..n {
        dp[a[i]] = std::cmp::max(dp[a[i]], dp[i] + 100);
        dp[b[i]] = std::cmp::max(dp[b[i]], dp[i] + 150);
    }
    println!("{}", dp[n]);
}
