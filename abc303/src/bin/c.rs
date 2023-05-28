use std::collections::HashSet;

use proconio::{input, marker::Chars};
fn main() {
    input! {(_, m, mut h, k): (isize, isize, isize, isize), s: Chars}
    let mut item = HashSet::new();
    for _ in 0..m {
        input! {(x, y): (isize, isize)}
        item.insert((x, y));
    }
    let mut now = (0, 0);
    for si in s {
        h -= 1;
        match si {
            'R' => now.0 += 1,
            'L' => now.0 -= 1,
            'U' => now.1 += 1,
            'D' => now.1 -= 1,
            _ => continue,
        }
        if h < 0 {
            println!("No");
            return;
        } else {
            if item.contains(&now) && h < k {
                h = k;
                item.remove(&now);
            }
        }
    }
    println!("Yes");
}
