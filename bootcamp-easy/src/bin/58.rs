use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize, (t, a): (i32, i32), h: [i32; n]}
    let mut ans = 0;
    let mut m = 1_000_000_000;
    for i in 0..n {
        let tmp = ((t * 1000 - h[i] * 6) - a * 1000).abs();
        if tmp < m {
            m = tmp;
            ans = i + 1;
        }
    }
    println!("{}", ans);
}
// fn ans1() {
//     input! {
//         n: usize,
//         (t, a): (f64, f64),
//         h: [isize; n]
//     }
//     let mut ans = 0;
//     let mut m = 1_000_000_000.0;
//     for i in 0..n {
//         let tmp = t - h[i] as f64 * 0.006;
//         // println!("{} -> {}", tmp, tmp - a);
//         if (tmp - a).abs() < m {
//             m = (tmp - a).abs();
//             ans = i + 1;
//         }
//     }
//     println!("{}", ans);
// }
