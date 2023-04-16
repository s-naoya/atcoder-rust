use proconio::input;
fn main() {
    input! {q: usize}
    for _ in 0..q {
        input! {x: u32}
        let mut ans = true;
        let xsqrt = (x as f64).sqrt() as u32;
        for i in 2..=xsqrt {
            if x % i == 0 {
                ans = false;
                break;
            }
        }
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
