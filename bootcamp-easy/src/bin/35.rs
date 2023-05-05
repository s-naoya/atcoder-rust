use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {mut s: Chars}
    s.sort_unstable();
    s.dedup();
    let alpha = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    // println!("{:?}", s);
    // 境界値をちゃんと計算する
    for i in 0..26 {
        if i == s.len() || (i < s.len() && s[i] != alpha[i]) {
            println!("{}", alpha[i]);
            return;
        }
    }
    println!("None");
}
// 12:10
