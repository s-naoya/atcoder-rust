use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; 5*n],
    }
    x.sort();
    for _ in 0..n {
        x.pop();
    }
    x.reverse();
    for _ in 0..n {
        x.pop();
    }
    let mut ans = 0.0;
    for xi in &x {
        ans += *xi as f64;
    }
    println!("{}", ans/(x.len() as f64));
}
