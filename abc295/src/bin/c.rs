use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    // a.sort();
    a.sort_unstable();
    let mut ans = 0;
    let mut i = 0;
    while i < n - 1 {
        if a[i] == a[i + 1] {
            ans += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    println!("{}", ans);
}
