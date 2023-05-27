use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {(h, _): (usize, usize), c: [Chars; h]}
    for i in 0..h * 2 {
        println!("{}", c[i / 2].iter().join(""));
    }
}
// 4:20
