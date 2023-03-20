use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = Vec::new();
    for ai in a {
        if ai % 2 == 0 {
            ans.push(ai);
        }
    }
    println!("{}", ans.iter().join(" "));
}
