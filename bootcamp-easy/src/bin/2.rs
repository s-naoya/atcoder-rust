use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, x: [i128; n]}
    let mut ans = 1_000_000_000_000_000;
    for p in 1..=100 {
        let mut sum = 0i128;
        for &xi in &x {
            // 引き算があるため、オーバーフローの可能性を考慮し符号ありの型にする
            sum += (xi - p) * (xi - p);
        }
        ans = std::cmp::min(ans, sum);
    }
    println!("{}", ans);
}
// 08:41
