use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, s: Chars}
    let mut x = 0;
    let mut x_max = x;
    for i in 0..n {
        if s[i] == 'I' {
            x += 1;
        } else if s[i] == 'D' {
            x -= 1;
        }
        x_max = std::cmp::max(x_max, x);
    }
    println!("{}", x_max);
}
// 3:54
