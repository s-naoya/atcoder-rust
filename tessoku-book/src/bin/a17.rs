use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    ans1(n, &a, &b);
}

fn ans1(n: usize, a: &Vec<usize>, b: &Vec<usize>) {
    let mut dp = vec![0; n];
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = std::cmp::min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2])
    }
    // println!("{:?}", dp);

    let mut route = vec![n];
    let mut now = n - 1;
    while now > 0 {
        // println!("{}: {:?}", now, route);
        if now < 2 {
            route.push(1);
            now = 0;
        } else if dp[now] == dp[now - 1] + a[now - 1] {
            route.push(now);
            now -= 1;
        } else if dp[now] == dp[now - 2] + b[now - 2] {
            route.push(now - 1);
            now -= 2;
        }
    }
    route.reverse();
    println!("{}\n{}", route.len(), route.iter().join(" "));
}
