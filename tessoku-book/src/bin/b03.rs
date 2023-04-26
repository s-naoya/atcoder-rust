use proconio::input;
fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort_unstable();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i == j || j == k || i == k {
                    continue;
                }
                if a[i] + a[j] + a[k] == 1000 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
