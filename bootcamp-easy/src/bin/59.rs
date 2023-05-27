use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {s: Chars}
    let mut ans = "AC";
    let mut cnt = 0;
    for i in 0..s.len() {
        if i == 0 {
            if s[0] != 'A' {
                ans = "WA";
                break;
            }
        } else {
            if (i >= 2 && i <= s.len() - 2) && s[i] == 'C' {
                cnt += 1;
            } else {
                if !s[i].is_ascii_lowercase() {
                    ans = "WA";
                    break;
                }
            }
        }
    }
    if cnt != 1 {
        ans = "WA";
    }
    println!("{}", ans);
}
// 12:45
