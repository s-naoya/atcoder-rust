use proconio::input;
fn main() {
    input! {
        (n, m): (usize, usize),
    }
    let mut g = vec![vec![]; m];
    for _ in 0..m {
        input! {(a, b, c): (usize, usize, usize)}
        g[a].push((b, c));
        g[b].push((a, c));
    }
}
