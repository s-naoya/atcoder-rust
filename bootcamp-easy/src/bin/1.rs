use proconio::input;
fn main() {
    input! {(a, b): (usize, usize)}
    // b-1口増やすと考える (b-1)/(a-1)の切り上げ
    // 分子に分母-1を足すと、切り上げが可能
    let ans = (b - 1 + a - 2) / (a - 1);
    // if b == 1 {
    //     println!("0");
    //     return;
    // }
    // let mut ans = 1;
    // while a + (a - 1) * (ans - 1) < b {
    //     ans += 1;
    // }
    println!("{}", ans);
}
// 11:31
