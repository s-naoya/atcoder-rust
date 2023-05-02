use itertools::Itertools;
use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize }
    let mut a = vec![];
    for i in 0..n {
        input! {at: usize}
        a.push((at, i + 1));
    }
    a.sort_unstable();
    println!("{}", a.iter().map(|(_, i)| i).into_iter().join(" "));
}
// 8:35
