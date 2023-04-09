use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m]
    }
    let mut g = vec![Vec::<usize>::new(); n];
    for (a, b) in ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut visited = vec![false; n];
    let mut nextvisit = VecDeque::<usize>::new();
    nextvisit.push_front(0);
    while !nextvisit.is_empty() {
        let now = nextvisit.pop_front().unwrap();
        if visited[now] {
            continue;
        } else {
            visited[now] = true;
        }
        for ni in &g[now] {
            nextvisit.push_front(*ni);
        }
    }

    let mut ans = "The graph is connected.";
    for v in visited {
        if !v {
            ans = "The graph is not connected.";
            break;
        }
    }
    println!("{}", ans);
}
