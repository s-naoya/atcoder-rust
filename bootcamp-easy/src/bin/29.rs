use std::cmp::min;

use proconio::input;
fn main() {
    ans2();
}
// fn ans1() {
//     input! {s: String}
//     let mut ans = 1_000_000;
//     for i in 0..s.len() - 2 {
//         let si = &s[i..i + 3];
//         let num: i32 = si.parse().unwrap();
//         let abs = (num - 753).abs();
//         ans = min(ans, abs);
//     }
//     println!("{}", ans);
// }
// 7:40
fn ans2() {
    input! {s: String}
    let ans = (0..s.len() - 2).fold(1_000_000, |b, i| {
        min(b, (&s[i..i + 3].parse::<i32>().unwrap() - 753).abs())
    });
    println!("{}", ans);
}
