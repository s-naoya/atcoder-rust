use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(mut a, mut b, mut c): (usize, usize, usize)}
    if a == b && b == c && a == c && a % 2 == 0 {
        println!("-1");
        return;
    }
    let mut cnt = 0;
    while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
        let (at, bt, ct) = (a, b, c);
        a = bt / 2 + ct / 2;
        b = at / 2 + ct / 2;
        c = bt / 2 + ct / 2;
        cnt += 1;
    }
    println!("{}", cnt);
}
// 8:40
