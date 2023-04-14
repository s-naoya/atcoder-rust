use petgraph::unionfind::UnionFind;
use proconio::input;
fn main() {
    input! {
        (n, q): (usize, usize)
    }

    let mut g = UnionFind::new(n);

    for _ in 0..q {
        input! {(m, u, v): (usize, usize, usize)}
        if m == 1 {
            g.union(u - 1, v - 1);
        } else if m == 2 {
            if g.equiv(u - 1, v - 1) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
