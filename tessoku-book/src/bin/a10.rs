use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
    }
    let mut max_left = vec![0; n];
    let mut max_right = vec![0; n];
    for i in 0..n {
        let right = n - 1;
        if i == 0 {
            max_left[i] = a[i];
            max_right[right - i] = a[right]
        } else {
            max_left[i] = std::cmp::max(max_left[i - 1], a[i]);
            max_right[right - i] = std::cmp::max(max_right[right - i + 1], a[right - i]);
        }
    }
    for _ in 0..d {
        input! {(l, r): (Usize1, Usize1)}
        println!("{}", std::cmp::max(max_left[l - 1], max_right[r + 1]));
    }
}
