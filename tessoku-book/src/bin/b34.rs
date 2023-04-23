use proconio::input;
fn main() {
    input! {
        (n, _, _): (usize, usize, usize),
        a: [usize; n]
    }
    let grundies = [0usize, 0, 1, 1, 2];
    let ans = a.iter().map(|&num| grundies[num % 5]).fold(0, |a, b| a ^ b);
    println!("{}", if ans != 0 { "First" } else { "Second" });
}
