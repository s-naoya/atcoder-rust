use itertools::Itertools;
use proconio::input;

fn dfs(now: usize, n: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, route: &mut Vec<usize>) {
    visited[now] = true;
    // println!("{}: {:?}", now, route);
    if now == n {
        route.push(now);
        println!("{}", route.iter().join(" "));
        return;
    }
    for ni in &g[now] {
        if !visited[*ni] {
            route.push(now);
            dfs(*ni, n, g, visited, route);
            route.pop();
        }
    }
    return;
}
fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m]
    }
    let mut g = vec![Vec::<usize>::new(); n + 1];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut route = Vec::<usize>::new();
    let mut visited = vec![false; n + 1];
    dfs(1, n, &g, &mut visited, &mut route);
}
