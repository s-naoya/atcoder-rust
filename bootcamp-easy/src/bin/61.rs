use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {mut s: Chars, t: Chars}
    for _ in 0..s.len() {
        s.insert(0, s[s.len() - 1]);
        s.pop();
        let mut ans = true;
        for j in 0..s.len() {
            if s[j] != t[j] {
                ans = false;
                break;
            }
        }
        if ans {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
// 6:37
