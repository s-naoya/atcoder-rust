// use std::collections::BTreeMap;

use itertools::Itertools;
// use proconio::{input, marker::Chars};
use proconio::input;
fn main() {
    ans2();
}
// fn ans1() {
//     input! {n: usize}
//     let mut rest = BTreeMap::new();
//     for i in 0..n {
//         input! {s: Chars, p: usize}
//         rest.entry(s).or_insert(vec![]).push((p, i + 1));
//     }
//     for (_, p) in &rest {
//         let mut pp = p.clone();
//         pp.sort_unstable_by(|a, b| b.0.cmp(&a.0));
//         for (_, j) in pp {
//             println!("{}", j);
//         }
//     }
// }
// 7:47
fn ans2() {
    input! {n: usize}
    let mut v = vec![];
    for i in 0..n {
        input! {s: String, p: usize}
        v.push((s, p, i));
    }
    v.sort_by(
        |(a, ap, _), (b, bp, _)| {
            if a != b {
                a.cmp(&b)
            } else {
                bp.cmp(&ap)
            }
        },
    );
    println!("{}", v.iter().map(|x| x.2 + 1).into_iter().join("\n"));
}
