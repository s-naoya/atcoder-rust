use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, t: [usize; n], m: usize}
    let tsum: usize = t.iter().sum();
    for _ in 0..m {
        input! {(p, x): (usize, usize)}
        let a = tsum - t[p - 1] + x;
        println!("{}", a);
    }
}
// 5:28
