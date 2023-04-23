use proconio::*;
use std::io::{self, *};
fn main() {
    let mut stdin = source::line::LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {n: usize}
    let mut left = 1;
    let mut right = n;
    for _ in 0..20 {
        let mid = left + (right - left) / 2;
        println!("? {}", mid);
        input! {num: usize}
        if num == 0 {
            left = mid;
        } else {
            right = mid;
        }
        if left + 1 == right {
            println!("! {}", left);
            return;
        }
        // }
    }
    println!("! 1");
}
