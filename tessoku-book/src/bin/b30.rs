use proconio::input;
fn main() {
    input! {(h, w): (usize, usize)}
    let m = 1_000_000_007;

    // 同じものを含む順列
    // (h+w-2)!/((h-1)!(w-1)!)
    let mut a = 1;
    for i in 1..=(h + w - 2) {
        a = (a * i) % m;
    }

    let mut b = 1;
    for i in 1..=(h - 1) {
        b = (b * i) % m;
    }
    for i in 1..=(w - 1) {
        b = (b * i) % m;
    }

    let ans = (a * power(b, m - 2, m)) % m;
    println!("{}", ans);
}
fn power(a: usize, b: usize, m: usize) -> usize {
    let mut ans = 1;
    let mut p = a;
    for i in 0..=30 {
        let wari = 1 << i;
        if (b / wari) % 2 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
    }
    ans
}
