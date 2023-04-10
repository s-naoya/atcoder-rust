use proconio::input;
fn main() {
    input! {
        (n, d): (usize, i64),
        t: [i64; n]
    }
    let mut ans = -1;
    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            ans = t[i];
            break;
        }
    }
    println!("{}", ans);
}
