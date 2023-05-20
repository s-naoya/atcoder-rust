use proconio::input;
fn main() {
    ans1();
}

fn ans1() {
    input! {n: usize, b: [usize; n-1]}
    let mut a = vec![0; n];
    a[0] = b[0];
    a[1] = b[0];
    for i in 1..n - 1 {
        a[i + 1] = b[i];
        if a[i] > b[i] {
            a[i] = b[i];
        }
    }
    println!("{}", a.iter().sum::<usize>());
}
// 10:31
