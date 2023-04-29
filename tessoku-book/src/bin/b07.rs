use proconio::{fastout, input};
#[fastout]
fn main() {
    // 一次元の差分累積和
    input! {t: usize, n: usize}
    let mut wa = vec![0; t + 1];
    // 一つ前の時間との差分を計算
    for _ in 0..n {
        input! {(l, r): (usize, usize)}
        wa[l] += 1;
        wa[r] -= 1;
    }
    // 時刻tの時に働いている従業員は、差分の累積和で計算できる
    // sum(wa[0]~wa[t])
    let mut sum = 0;
    for i in 0..t {
        sum += wa[i];
        println! {"{}", sum};
    }
}
