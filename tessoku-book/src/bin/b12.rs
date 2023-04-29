use proconio::input;
fn main() {
    input! {n: f64}

    let mut left = 0.0;
    let mut right = 50.0;
    let ans = loop {
        let mid: f64 = left + (right - left) / 2.0;
        let nans = mid.powf(3.0) as f64 + mid;
        if (nans - n).abs() <= 0.001 {
            break mid as f64;
        } else if nans < n {
            left = mid;
            continue;
        } else if nans > n {
            right = mid;
            continue;
        }
    };
    println!("{}", ans);
}
