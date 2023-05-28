use proconio::{input, marker::Chars};
fn main() {
    input! {_: usize, s: Chars, t: Chars}
    let mut ans = "Yes";
    for i in 0..s.len() {
        if s[i] == t[i]
            || (s[i] == '1' && t[i] == 'l')
            || (s[i] == 'l' && t[i] == '1')
            || (s[i] == '0' && t[i] == 'o')
            || (s[i] == 'o' && t[i] == '0')
        {
            continue;
        } else {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}
