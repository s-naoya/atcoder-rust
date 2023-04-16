use proconio::input;
fn main() {
    input! {(a, b): (usize, usize)}
    let m = 1_000_000_007;

    let mut p = a;
    let mut ans = 1;
    for i in 0..64 {
        let wari = 1 << i;
        if (b / wari) % 2 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
    }
    println!("{}", ans);
}
