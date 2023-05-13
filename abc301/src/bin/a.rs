use proconio::{input, marker::Chars};
fn main() {
    input! {(n, s): (usize, Chars)}
    let (mut win_1, mut win_2) = (0, 0);
    let mut b = 0;
    for i in 0..n {
        if s[i] == 'T' {
            win_1 += 1;
            if b == 0 && win_1 >= n / 2 {
                b = 1;
            }
        } else {
            win_2 += 1;
            if b == 0 && win_2 >= n / 2 {
                b = 2;
            }
        }
    }
    let ans = if win_1 > win_2 {
        "T"
    } else if win_2 > win_1 {
        "A"
    } else {
        if b == 1 {
            "T"
        } else {
            "A"
        }
    };
    println!("{}", ans);
}
