use std::mem::swap;

use proconio::input;
fn main() {
    input! {(mut a, mut b): (usize, usize)}
    let (a_ori, b_ori) = (a, b);
    if a < b {
        swap(&mut a, &mut b);
    }
    while b != 0 {
        a %= b;
        swap(&mut a, &mut b);
    }
    println!("{}", a_ori * b_ori / a);
}
