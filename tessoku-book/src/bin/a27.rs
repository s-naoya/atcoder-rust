use proconio::input;
fn main() {
    input! {(mut a, mut b): (usize, usize)}
    if a < b {
        std::mem::swap(&mut a, &mut b)
    }
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    println!("{}", a);
}
