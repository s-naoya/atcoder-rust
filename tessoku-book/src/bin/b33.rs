use proconio::input;
fn main() {
    input! {
        (n, _, _): (usize, usize, usize),
        ab: [(usize, usize); n]
    }
    let ans = ab.iter().fold(0, |acc, (a, b)| acc ^ (*a - 1) ^ (*b - 1));
    println!("{}", if ans == 0 { "Second" } else { "First" });
}
