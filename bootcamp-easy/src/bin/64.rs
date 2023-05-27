use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {mut o: Chars, mut e: Chars}
    let mut pass = vec![];
    for _ in 0..o.len() {
        if !o.is_empty() {
            pass.push(o[0]);
            o.remove(0);
        }
        if !e.is_empty() {
            pass.push(e[0]);
            e.remove(0);
        }
    }
    println!("{}", pass.iter().join(""));
}
// 4:24
