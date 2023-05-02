use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(a, b): (u32, u32)}
    // 紙に数式を書いて計算する
    let ab = 10_u32.pow(b.to_string().len() as u32) * a + b;
    let mut ans = "No";
    for i in 0..=1000 {
        if ab == i * i {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
// 20:07
