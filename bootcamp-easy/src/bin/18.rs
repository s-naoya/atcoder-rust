use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {s: Chars}
    let mut ans = 0;
    let mut tmp = 0;
    for si in s {
        if si == 'A' || si == 'C' || si == 'G' || si == 'T' {
            tmp += 1;
        } else {
            ans = std::cmp::max(ans, tmp);
            tmp = 0;
        }
    }
    ans = std::cmp::max(ans, tmp);
    println!("{}", ans);
}
// 4:23
