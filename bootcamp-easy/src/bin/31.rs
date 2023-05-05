use std::collections::HashSet;

use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {s: Chars}
    let mut set = HashSet::new();
    for si in s {
        if set.contains(&si) {
            println!("no");
            return;
        } else {
            set.insert(si);
        }
    }
    println!("yes");
}
// 2:17
