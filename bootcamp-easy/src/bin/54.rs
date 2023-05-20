use std::collections::HashSet;

use proconio::{input, marker::Chars};
fn main() {
    ans1();
}

fn ans1() {
    input! {n: usize, w: [Chars; n]}
    let mut tango = HashSet::new();
    let mut ans = "Yes";
    let mut before = w[0].clone();
    // before.reverse();
    tango.insert(w[0].iter().collect::<String>());
    for i in 1..n {
        if before.last() != w[i].first() {
            ans = "No";
            break;
        } else {
            before = w[i].clone();
        }
        let now = w[i].iter().collect::<String>();
        if !tango.contains(&now) {
            tango.insert(now.clone());
        } else {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}
// 18:18
