use proconio::{input, marker::Chars};
fn main() {
    input! {
        (h,w): (usize, usize),
        a: [Chars; h],
        b: [Chars; h]
    }
    for s in 0..h {
        for t in 0..w {
            let mut ok = true;
            for i in 0..h {
                for j in 0..w {
                    // A を縦方向にs回、横方向にtシフトするとA[i][j]はA[(i−s)mod H][(j−t)mod W] に移る
                    // h, wを足さないと、i>sの時i-sの計算でオーバーフローする
                    if a[(h + i - s) % h][(w + j - t) % w] != b[i][j] {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
    return;
}
// fn main() {
//     input! {
//         (h, w): (usize, usize),
//         a: [Chars; h],
//         b: [Chars; h]
//     }
//     for i in 0..h {
//         let mut ai = a[i].clone();
//         for j in 0..w {
//             if ai == b[0] {
//                 let ans = check(&a, &b, h, j, i);
//                 if ans {
//                     println!("Yes");
//                     return;
//                 }
//             } else {
//                 ai = yoko_shift(&ai);
//             }
//         }
//     }
//     println!("No");
// }
// fn check(
//     a: &Vec<Vec<char>>,
//     b: &Vec<Vec<char>>,
//     h: usize,
//     cnt_yoko: usize,
//     cnt_tate: usize,
// ) -> bool {
//     let mut ac = a.clone();
//     let tate_i = |i: usize, len: usize| -> usize {
//         if i >= len {
//             i - len
//         } else {
//             i
//         }
//     };
//     for itmp in 0..h {
//         let i = tate_i(itmp + cnt_tate, h);
//         for _ in 0..cnt_yoko {
//             ac[i] = yoko_shift(&ac[i])
//         }
//         if ac[i] != b[itmp] {
//             return false;
//         }
//     }
//     return true;
// }
// fn yoko_shift(v: &Vec<char>) -> Vec<char> {
//     let mut vc = v.clone();
//     vc.push(v[0]);
//     vc.remove(0);
//     return vc;
// }
