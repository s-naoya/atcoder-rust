use std::{collections::VecDeque, vec};

use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    }
    let mut g = vec![Vec::new(); n + 1];
    for _ in 1..=m {
        input! {(a, b): (usize, usize)}
        g[a].push(b);
        g[b].push(a);
    }

    let mut dist = vec![-1; n + 1];
    let mut que = VecDeque::new();
    que.push_back(1);
    dist[1] = 0;
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for to in &g[pos] {
            if dist[*to] == -1 {
                dist[*to] = dist[pos] + 1;
                que.push_back(*to);
            }
        }
    }

    for i in 1..=n {
        println!("{}", dist[i]);
    }
}
