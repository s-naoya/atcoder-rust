use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {s: Chars}
    let mut ai = 0;
    let mut zi = 0;
    for i in 0..s.len() {
        if s[i] == 'A' {
            ai = i;
            break;
        }
    }
    for i in (0..s.len()).rev() {
        if s[i] == 'Z' {
            zi = i;
            break;
        }
    }
    println!("{}", zi - ai + 1);
}
// 4:19
