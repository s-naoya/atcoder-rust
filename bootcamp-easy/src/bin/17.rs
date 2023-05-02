use proconio::input;
fn main() {
    ans2();
}
fn ans2() {
    input! {n: usize, mut v: [usize; n]}
    v.sort_unstable();
    let ans: f64 = v
        .iter()
        .fold(v[0] as f64, |vi, &vj| (vi as f64 + vj as f64) / 2.0);
    println!("{}", ans);
}
// fn ans1() {
//     input! {n: usize, mut v: [usize; n]}
//     v.sort_unstable();
//     let mut ans = v[0] as f64;
//     for i in 1..n {
//         ans = (ans + v[i] as f64) / 2.0;
//     }
//     println!("{}", ans);
// }
// 4:06
