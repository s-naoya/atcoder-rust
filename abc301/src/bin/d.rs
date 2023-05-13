use proconio::{input, marker::Chars};
fn main() {
    ans2();
}
fn ans2() {
    input! {s: Chars, n: u128}
    let f = |x: &Vec<char>| u128::from_str_radix(&x.iter().collect::<String>(), 2).unwrap();
    let mut sc = s.clone();
    for i in 0..s.len() {
        if sc[i] == '?' {
            sc[i] = '0';
        }
    }
    let sn = f(&sc);
    if sn > n {
        println!("-1");
        return;
    }

    for i in (0..s.len()).rev() {
        if s[i] == '?' {
            let mut sc_c = sc.clone();
            sc_c[i] = '1';
            let sn = f(&sc_c);
            if sn < n {
                sc = sc_c;
            }
        }
    }
    let sn = f(&sc);
    println!("{}", sn);
}

// fn ans1() {
//     input! {s: Chars, n: u128}
//     let q = s.iter().filter(|&&x| x == '?').count();
//     for bit in (0..(1 << q)).rev() {
//         let mut sc = s.clone();
//         for i in 0..q {
//             let num = (bit / (1 << i)) % 2;
//             // println!("{} -> ", num);
//             // let cnt = 0;
//             for j in (0..s.len()).rev() {
//                 // println!("i {} j {} sj {} cnt {} ", i, j, sc[j], cnt);
//                 if sc[j] == '?' {
//                     // if i == cnt {
//                     sc[j] = std::char::from_digit(num as u32, 10).unwrap();
//                     // println!("{:?}", sc);
//                     break;
//                     // } else {
//                     //     cnt += 1;
//                     // }
//                 }
//             }
//         }
//         // println!("sc: {:?}", sc);
//         let sc = u128::from_str_radix(&sc.into_iter().collect::<String>(), 2).unwrap();
//         if sc <= n {
//             println!("{}", sc);
//             return;
//         }
//     }
//     println!("-1");
// }
