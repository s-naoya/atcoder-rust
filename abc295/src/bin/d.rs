use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn ans_1() {
    // s.len*0〜9の累積和 -> 全て偶数であればOKなので、mod 2 -> 0〜9のmod 2が等しければ、嬉しい区間
    input! {s: Chars}
    let mut x = [0usize; 10];
    let mut mp = HashMap::new();
    mp.insert(x.clone(), 1);
    for si in s.iter() {
        x[si.to_digit(10).unwrap() as usize] ^= 1;
        *mp.entry(x.clone()).or_insert(0) += 1;
    }

    let mut ans: usize = 0;
    for (_, num) in mp.iter() {
        ans += num * (num - 1) / 2;
    }
    println!("{}", ans);
}
fn main() {
    ans_1();
}
