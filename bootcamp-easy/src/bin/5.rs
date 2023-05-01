use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        (n, m, c): (usize, usize, i32),
        b: [i32; m],
        a: [[i32; m]; n]
    }
    let mut ans = 0;
    for i in 0..n {
        let mut sum = c;
        for j in 0..m {
            sum += a[i][j] * b[j];
        }
        if sum > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
// 2:52
