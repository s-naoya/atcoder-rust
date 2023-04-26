use proconio::input;
fn main() {
    input! {mut n: usize}
    for x in (0..10).rev() {
        let wari = 1 << x;
        print!("{}", (n / wari) % 2);
    }
    println!("");
}
