use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut g = vec![vec![]; n + 1];
    for _ in 0..m {
        input! {(a, b, c): (usize, usize, usize)}
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut cur = vec![1_000_000_000; n + 1];
    cur[1] = 0;
    let mut q = BinaryHeap::new();
    q.push((Reverse(cur[1]), 1));
    let mut kakutei = vec![false; n + 1];

    while !q.is_empty() {
        let (_, pos) = q.pop().unwrap();
        if kakutei[pos] {
            continue;
        }
        kakutei[pos] = true;
        for (gn, gc) in &g[pos] {
            cur[*gn] = std::cmp::min(cur[*gn], cur[pos] + *gc);
            q.push((Reverse(cur[*gn]), *gn));
        }
    }

    let mut now = n;
    let mut route = vec![n];
    while now != 1 {
        for (gn, gc) in &g[now] {
            if cur[now].wrapping_sub(cur[*gn]) == *gc {
                now = *gn;
                break;
            }
        }
        route.push(now);
    }
    println!("{}", route.iter().rev().join(" "));
}
