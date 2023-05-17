use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {(h, w): (usize, usize), a:[Chars; h]}
    println!("{}", vec!['#'; w + 2].iter().join(""));
    for ai in a {
        print!("#");
        print!("{}", ai.iter().join(""));
        println!("#");
    }
    println!("{}", vec!['#'; w + 2].iter().join(""));
}
// 3:41
