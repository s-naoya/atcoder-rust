use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {(a, b): (usize, usize), s: Chars}
    if s.len() != a + b + 1 {
        println!("No");
        return;
    }
    for i in 0..a {
        if s[i] == '-' {
            println!("No");
            return;
        }
    }
    if s[a] != '-' {
        println!("No");
        return;
    }
    for j in a + 1..a + b + 1 {
        if s[j] == '-' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
// 5:38
