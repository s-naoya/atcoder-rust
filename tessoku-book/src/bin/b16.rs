use proconio::input;
fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    let mut dp = vec![0; n];
    dp[1] = num::abs(h[0] - h[1]);
    for i in 2..n {
        dp[i] = std::cmp::min(
            dp[i - 1] + num::abs(h[i - 1] - h[i]),
            dp[i - 2] + num::abs(h[i - 2] - h[i]),
        );
    }
    println!("{}", dp.last().unwrap());
}
