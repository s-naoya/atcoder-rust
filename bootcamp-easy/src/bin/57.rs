use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(a, b): (isize, isize)}
    let mut ans = -1;
    for i in 1..=2000 {
        if ((i * 8) / 100) == a && ((i * 10) / 100) == b {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
// 6:15
