use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    for si in s {
        if si.to_digit(10).unwrap() == 1 {
            ans = ans + 1;
        }
    }
    println!("{}", ans);
}
