use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }
    let mut ans = 0;
    for ai in 0..a+1 {
        for bi in 0..b+1 {
            for ci in 0..c+1 {
                if 500*ai + 100*bi + 50*ci == x {
                    ans = ans + 1;
                }
            }
        }
    }

    println!("{}", ans);
}
