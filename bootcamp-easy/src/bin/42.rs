use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        (n, m): (usize, usize),
    }
    let mut cnt = vec![0; m];
    for _ in 0..n {
        input! {k: usize, a: [usize; k]}
        for ai in a {
            cnt[ai - 1] += 1;
        }
    }
    let mut ans = 0;
    for c in cnt {
        if c == n {
            ans += 1;
        }
    }
    println!("{}", ans);
}
// 3;29
