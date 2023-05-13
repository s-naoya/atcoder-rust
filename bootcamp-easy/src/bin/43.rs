use std::collections::HashMap;

use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {w: Chars}
    let mut cnt = HashMap::new();
    for wi in w {
        *cnt.entry(wi).or_insert(0) += 1;
    }
    let mut ans = "Yes";
    for (_, i) in cnt {
        if i % 2 != 0 {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}
// 3:27
