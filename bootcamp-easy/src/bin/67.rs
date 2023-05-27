use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, s: [Chars; n]}
    let mut cnt = BTreeMap::new();
    for si in s {
        *cnt.entry(si).or_insert(0) += 1;
    }
    let mut m = 0;
    for (_, &c1) in &cnt {
        m = m.max(c1);
    }
    for (c0, &c1) in &cnt {
        if c1 == m {
            println!("{}", c0.iter().join(""));
        }
    }
}
// 13:43
