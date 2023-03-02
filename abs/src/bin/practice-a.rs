use proconio::marker::Chars;
use proconio::input;
use std::iter::FromIterator;

fn main() {
    input! {
        a: i32,
        bc: [i32; 2],
        s: Chars,
    }
    println!("{} {}", a+bc[0]+bc[1], String::from_iter(s));
}
