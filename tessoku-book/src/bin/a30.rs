use proconio::input;
fn main() {
    input! {(n, r): (usize, usize)}
    let m = 1_000_000_007;

    let mut b = 1;
    for i in 1..=r {
        b = (b * i) % m;
    }
    for i in 1..=(n - r) {
        b = (b * i) % m;
    }

    let mut a = 1;
    for i in 1..=n {
        a = (a * i) % m;
    }

    let ans = {
        let mut p = b;
        let mut ans = 1;
        for i in 0..30 {
            let wari = 1 << i;
            if ((m - 2) / wari) % 2 == 1 {
                ans = (ans * p) % m;
            }
            p = (p * p) % m;
        }
        ans
    };
    let ans = (ans * a) % m;
    println!("{}", ans);
}

fn power(a: u64, b: u64, m: u64) -> u64 {
    let mut ans = 1;
    for i in 1..=b {
        ans = (ans * a) % m;
    }
    ans
}
