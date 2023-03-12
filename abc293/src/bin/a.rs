use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {mut s: Chars,}
    for i in 1..=s.len() / 2 {
        let tmp = s[2 * i - 1 - 1];
        s[2 * i - 1 - 1] = s[2 * i - 1];
        s[2 * i - 1] = tmp;
    }
    for si in s {
        print!("{}", si);
    }
}
