use proconio::input;
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

    let mut max_id = 0;
    let mut max_val = 0;
    for i in 1..=n {
        if max_val <= g[i].len() {
            max_id = i;
            max_val = g[i].len();
        }
    }
    println!("{}", max_id);
}
