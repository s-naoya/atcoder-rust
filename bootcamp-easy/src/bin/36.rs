use proconio::input;
fn main() {
    ans2();
}
// fn ans1() {
//     input! {(n, m): (usize, i64)}
//     let mut ok = vec![0i64; n + 2];
//     for _ in 0..m {
//         input! {(l, r): (usize, usize)}
//         ok[l] += 1;
//         ok[r + 1] -= 1;
//     }
//     let mut ans = 0;
//     for i in 1..n + 1 {
//         ok[i] += ok[i - 1];
//         if ok[i] == m {
//             ans += 1;
//         }
//     }
//     println!("{}", ans);
// }
fn ans2() {
    input! {(_, m): (usize, i64)}
    let mut l_max = 0;
    let mut r_max = 1_000_000;
    for _ in 0..m {
        input! {(l, r): (usize, usize)}
        l_max = l_max.max(l);
        r_max = r_max.min(r);
    }
    let ans = if l_max > r_max { 0 } else { r_max - l_max + 1 };
    println!("{}", ans);
}
