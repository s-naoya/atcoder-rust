use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let ans = a.iter().fold(0, |acc, x| acc ^ x);
    println!("{}", if ans == 0 { "Second" } else { "First" });
}
