use proconio::input;
fn main() {
    ans2();
}
// fn ans1() {
//     input! {(_, m, x): (usize, usize, usize), a: [usize; m]}
//     let mut e = 0;
//     let mut f = 0;
//     for i in 0..m {
//         if a[i] < x {
//             e += 1;
//         }
//         if a[i] > x {
//             f += 1;
//         }
//     }
//     println!("{}", std::cmp::min(e, f));
// }
// 4:24
fn ans2() {
    input! {(_, m, x): (usize, usize, usize), a: [usize; m]}
    let xi = a.binary_search(&x).err().unwrap();
    let a1 = &a[0..xi].len();
    let a2 = &a[xi..].len();
    let ans = std::cmp::min(*a1, *a2);
    println!("{}", ans);
}
