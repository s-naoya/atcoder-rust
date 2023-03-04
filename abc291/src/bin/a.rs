use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    for i in 0..s.len() {
        if s.get(i).unwrap().is_ascii_uppercase() {
           println!("{}", i+1);
        }
    }
}