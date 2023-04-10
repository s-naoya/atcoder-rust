use std::mem::swap;

use proconio::input;

// fn ans_1() {
//     input! {
//         (mut a, mut b): (u64, u64)
//     }
//     let mut ans = 0u64;
//     // let mut i = 0;
//     while a != b {
//         if a > b {
//             let mut cnt = (a - b) / b;
//             if a - b < b {
//                 cnt += 1;
//             }
//             // println!("{}", cnt);
//             a = a - b * cnt;
//             ans += cnt;
//         } else if a < b {
//             let mut cnt = (b - a) / a;
//             if b - a < a {
//                 cnt += 1;
//             }
//             // println!("{}", cnt);
//             b = b - a * cnt;
//             ans += cnt;
//         }
//         // println!("{} {} {}", a, b, ans);
//         // i += 1;
//         // if i > 10 {
//         //     break;
//         // }
//     }
//     println!("{}", ans);
// }

fn ans_2() {
    input! {(mut a, mut b): (u64, u64)}
    if a < b {
        swap(&mut a, &mut b);
    }
    let mut ans = 0u64;
    while b > 0 {
        ans += a / b;
        a %= b;
        swap(&mut a, &mut b);
    }
    println!("{}", ans - 1);
}
fn main() {
    // ans_1();
    ans_2();
}
