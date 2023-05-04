use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(n, a, b): (usize, usize, usize)}
    println!("{}", a * (n / (a + b)) + std::cmp::min(n % (a + b), a));
    // println!(
    //     "{}",
    //     a * (n / (a + b)) + if n % (a + b) >= a { a } else { n % (a + b) }
    // );
}
// 4:53
