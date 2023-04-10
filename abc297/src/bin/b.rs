use std::collections::HashMap;

use proconio::{input, marker::Chars};

// fn ans_1() {
//     input! {
//         s: Chars
//     }
//     {
//         let (mut x, mut y) = (0, 0);
//         let mut flag = false;
//         for i in 0..8 {
//             if s[i] == 'B' && !flag {
//                 x = i + 1;
//                 flag = true;
//             } else if s[i] == 'B' && flag {
//                 y = i + 1;
//             }
//         }
//         if (x % 2 == 0 && y % 2 == 1) || (x % 2 == 1 && y % 2 == 0) {
//         } else {
//             println!("No");
//             return;
//         }
//     }
//     {
//         let mut flag = 0;
//         for i in 0..8 {
//             if s[i] == 'R' {
//                 if flag == 0 {
//                     flag = 1;
//                 } else if flag == 1 {
//                     flag = 2;
//                 }
//             } else if s[i] == 'K' {
//                 if flag == 1 {
//                 } else {
//                     println!("No");
//                     return;
//                 }
//             }
//         }
//     }
//     println!("Yes");
// }
fn ans_2() {
    input! {s: Chars}
    let mut mp: HashMap<char, Vec<usize>> = HashMap::new();
    for i in 0..8 {
        mp.entry(s[i]).or_insert(vec![]).push(i + 1);
    }
    let mut ans = "Yes";
    if mp[&'B'][0] % 2 == mp[&'B'][1] % 2 {
        ans = "No";
    }
    if mp[&'R'][0] > mp[&'K'][0] || mp[&'K'][0] > mp[&'R'][1] {
        ans = "No";
    }
    println!("{}", ans);
}
fn main() {
    // ans_1();
    ans_2();
}
