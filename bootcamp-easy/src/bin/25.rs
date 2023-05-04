use itertools::Itertools;
use proconio::input;
fn main() {
    ans2();
}
// fn ans1() {
//     input! {n: usize, p: [usize; n], q: [usize; n]}
//     let (mut a, mut b) = (0, 0);
//     for (i, r) in (1..=n).permutations(n).enumerate() {
//         if r == p {
//             a = i as i64 + 1;
//         }
//         if r == q {
//             b = i as i64 + 1;
//         }
//     }
//     println!("{}", (a - b).abs());
// }
// 8:51
fn ans2() {
    input! {n: usize, p: [usize; n], q: [usize; n]}
    let a = (1..=n)
        .permutations(n)
        .enumerate()
        .find(|(_, r)| *r == p)
        .unwrap();
    let b = (1..=n)
        .permutations(n)
        .enumerate()
        .find(|(_, r)| *r == q)
        .unwrap();
    println!("{}", (a.0 as i64 - b.0 as i64).abs());
}
