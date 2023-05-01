use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(h, w): (u128, u128)}
    // hかwが1のとき、角は全く動く事ができないため、答えは1になる（例外：コーナーケース）
    // 制約の下限にも注意する
    let ans = if h == 1 || w == 1 { 1 } else { (h * w + 1) / 2 };
    println!("{}", ans);
}
// 10:23
