use std::collections::HashMap;

use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {s: Chars}
    let mut cnt = HashMap::new();
    cnt.insert('N', 0);
    cnt.insert('W', 0);
    cnt.insert('S', 0);
    cnt.insert('E', 0);
    for si in s {
        *cnt.entry(si).or_insert(0) += 1;
    }
    if (cnt[&'N'] == 0 && cnt[&'S'] > 0)
        || (cnt[&'N'] > 0 && cnt[&'S'] == 0)
        || (cnt[&'W'] == 0 && cnt[&'E'] > 0)
        || (cnt[&'W'] > 0 && cnt[&'E'] == 0)
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
// 8:11
