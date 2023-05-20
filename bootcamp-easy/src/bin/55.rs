use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        (n, d): (usize, usize),
        x: [[f64; d]; n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut z = 0.0;
            for k in 0..d {
                z += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
            }
            z = z.sqrt();
            if z == z.floor() {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
