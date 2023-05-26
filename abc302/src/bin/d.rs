use proconio::input;
fn main() {
    input! {(n, m, d): (usize, usize, isize), mut a: [isize; n], mut b: [isize; m]}
    a.sort_unstable();
    // b.sort_unstable();
    let ans = ans1(d, &a, &b);
    println!("{}", ans);
}
fn ans1(d: isize, a: &Vec<isize>, b: &Vec<isize>) -> isize {
    let mut ans = -1;
    for &bj in b {
        let s = a.binary_search(&(bj + d));
        match s {
            Ok(n) => ans = ans.max(a[n] + bj),
            Err(n) => {
                if n >= 1 && a[n - 1] >= bj - d {
                    ans = ans.max(a[n - 1] + bj);
                }
            }
        }
    }
    return ans;
}
