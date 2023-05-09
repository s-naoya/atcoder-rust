use itertools::Itertools;
use proconio::input;
fn main() {
    ans2();
}
// fn ans1() {
//     input! {abcde: [usize; 5]}
//     let mut ans = 1_000_000;
//     for a in abcde.iter().permutations(5) {
//         let mut sum = 0;
//         for &b in a {
//             sum += if sum % 10 != 0 { 10 - sum % 10 } else { 0 };
//             sum += b;
//         }
//         ans = ans.min(sum);
//     }
//     println!("{}", ans);
// }
// // 23:22
fn ans2() {
    input! {abcde: [usize; 5]}
    let f = |acc: usize, x: &&usize| {
        if acc % 10 != 0 {
            acc + 10 - acc % 10 + *x
        } else {
            acc + *x
        }
    };
    let ans = abcde
        .iter()
        .permutations(5)
        .map(|x| x.iter().fold(0, f))
        .min()
        .unwrap();
    println!("{}", ans);
}
