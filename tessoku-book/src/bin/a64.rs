use std::{cmp::Reverse, collections::BinaryHeap};

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
    let mut kakutei = vec![false; n + 1];
    q.push((Reverse(0), 1));

    while !q.is_empty() {
        // println!("====");
        let (_, pos) = q.pop().unwrap();
        // print!("POP {} -> ", pos);
        if kakutei[pos] {
            // println!("NG");
            continue;
        }
        // println!("OK");
        kakutei[pos] = true;
        // println!("{:?}", g);
        for (gn, gc) in &g[pos] {
            if kakutei[*gn] {
                continue;
            }
            // print!("{}: {} or {} + {} -> ", *gn, cur[*gn], cur[pos], *gc);
            cur[*gn] = std::cmp::min(cur[*gn], cur[pos].clone() + *gc);
            q.push((Reverse(cur[*gn].clone()), *gn));
            // println!("{}", cur[*gn])
        }
    }
    for i in 1..=n {
        if cur[i] == 1_000_000_000 {
            println!("-1");
        } else {
            println!("{}", cur[i]);
        }
    }
}
