use proconio::{fastout, input};
fn main() {
    ans1();
}
#[fastout]
fn ans1() {
    input! {(n, k, q): (usize, i32, i32)}
    let mut ok = vec![0; n + 1];
    for _ in 0..q {
        input! {a: usize}
        ok[a] += 1;
    }
    for i in 1..=n {
        if ok[i] > q - k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
// 9:23
