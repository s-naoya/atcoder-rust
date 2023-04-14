use petgraph::unionfind::UnionFind;
use proconio::input;
fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
        q: usize,
    }
    let mut query = vec![];
    let mut isab = vec![true; m];
    for _ in 0..q {
        input! {t: usize}
        if t == 1 {
            input! {x: usize}
            isab[x - 1] = false;
            query.push((1, ab[x - 1].0, ab[x - 1].1));
        } else if t == 2 {
            input! {(u, v): (usize, usize)}
            query.push((2, u, v));
        }
    }

    let mut uf = UnionFind::<usize>::new(n);
    for (i, (a, b)) in ab.iter().enumerate() {
        if isab[i] {
            uf.union(*a - 1, *b - 1);
        }
    }

    let mut ans = vec![];
    for (t, a, b) in query.iter().rev() {
        if *t == 1 {
            uf.union(*a - 1, *b - 1);
        } else if *t == 2 {
            let ans_tmp = if uf.equiv(*a - 1, *b - 1) {
                "Yes"
            } else {
                "No"
            };
            ans.push(ans_tmp);
        }
    }

    for a in ans.iter().rev() {
        println!("{}", a);
    }
}
