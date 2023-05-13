use std::cmp::min;

use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {s: Chars}
    let (mut ans_0, mut ans_1) = (0, 0);
    for i in 0..s.len() {
        ans_0 += if i % 2 == 0 {
            if s[i] == '0' {
                0
            } else {
                1
            }
        } else {
            if s[i] == '1' {
                0
            } else {
                1
            }
        };
        ans_1 += if i % 2 == 0 {
            if s[i] == '0' {
                1
            } else {
                0
            }
        } else {
            if s[i] == '1' {
                1
            } else {
                0
            }
        }
    }
    println!("{}", min(ans_0, ans_1));
}
// 5:43
