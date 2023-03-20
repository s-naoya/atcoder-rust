use proconio::input;
fn main() {
    input! {
        n: usize,
        mut pn: [(usize, usize); n],
    }
    pn.insert(0, (0, 0));
    let pn = pn;
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for l in 1..=n {
        for r in (1..=n).rev() {
            let score1 = if l <= pn[l - 1].0 && pn[l - 1].0 <= r {
                pn[l - 1].1
            } else {
                0
            };
            // let score2 = if l <= pn[r+1].0 && pn[r+1].0
        }
    }
}
