use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m]
    }
    let mut g = vec![Vec::<usize>::new(); n + 1];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    for i in 1..=n {
        g[i].sort_unstable();
        println!("{}: {{{}}}", i, g[i].iter().join(", "));
    }
}
