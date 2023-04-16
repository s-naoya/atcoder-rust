use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {n: usize}

    let m = 10000;
    let mut r = 0;
    for _ in 0..n {
        input! {(t, a): (char, i32)}
        match t {
            '+' => r += a,
            '-' => r -= a,
            '*' => r *= a,
            _ => continue,
        }
        if r < 0 {
            r += m;
        }
        r %= m;
        println!("{}", r);
    }
}
