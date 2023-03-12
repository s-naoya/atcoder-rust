use proconio::{fastout, input};
use std::collections::HashSet;
fn dfs(
    w: usize,
    h: usize,
    a: &Vec<Vec<usize>>,
    route: &mut HashSet<usize>,
    cur: (usize, usize),
    ans: &mut usize,
) {
    if cur.0 >= h || cur.1 >= w {
        return;
    }
    if route.contains(&a[cur.0][cur.1]) {
        return;
    }
    if cur.0 == h - 1 && cur.1 == w - 1 {
        *ans += 1;
        return;
    }
    route.insert(a[cur.0][cur.1]);
    dfs(w, h, a, route, (cur.0 + 1, cur.1), ans);
    dfs(w, h, a, route, (cur.0, cur.1 + 1), ans);
    route.remove(&a[cur.0][cur.1]);
}

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        a: [[usize; w]; h],
    }
    let mut ans: usize = 0;
    dfs(w, h, &a, &mut HashSet::<usize>::new(), (0, 0), &mut ans);
    println!("{}", ans);
}
