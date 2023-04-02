use proconio::input;
fn main() {
    input! {(n, m): (u64, u64)}

    let mut x = m;
    let xmax = std::cmp::min(1_000_000_000_000, n * n);
    while x <= xmax {
        let mut ai = 1;
        // while ai <= n || ai <= ((x as f64).sqrt()) as u64 {
        while ai <= n && ai <= m {
            // while ai <= n {
            // print!("{} ", ai);
            if x % ai == 0 && x / ai <= n {
                println!("{}", x);
                return;
            } else {
                ai += 1;
            }
        }
        x += 1;
    }
    println!("-1");
}
