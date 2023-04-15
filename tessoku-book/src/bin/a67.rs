use petgraph::unionfind::UnionFind;
use proconio::input;
fn main() {
    input! {
        (n, m): (usize, usize),
        mut abc: [(usize, usize, usize); m]
    }
    abc.sort_unstable_by(|(_, _, c1), (_, _, c2)| c1.cmp(c2));

    let mut uf = UnionFind::<usize>::new(n);
    let mut ans = 0usize;
    for (a, b, c) in abc {
        if uf.equiv(a - 1, b - 1) {
            continue;
        } else {
            uf.union(a - 1, b - 1);
            ans += c;
        }
    }
    println!("{}", ans);
}
