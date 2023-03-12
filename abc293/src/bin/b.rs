use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cnt = vec![false; n];
    for i in 0..n {
        let ai = a[i];
        if !cnt[i] {
            cnt[ai - 1] = true;
        }
    }
    let mut ans = Vec::new();
    for i in 0..n {
        if !cnt[i] {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
