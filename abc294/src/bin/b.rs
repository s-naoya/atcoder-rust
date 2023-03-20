use proconio::input;
fn main() {
    // ans_othre();
    // return;
    input! {
        (h, w): (usize, usize),
        a: [[usize; w]; h],
    }
    let alpha: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for ai in a {
        for aij in ai {
            if aij == 0 {
                print!(".");
            } else {
                print!("{}", alpha[aij - 1]);
            }
        }
        println!("");
    }
}

// fn ans_othre() {
//     input! {
//         (h, w): (usize, usize),
//         a: [[u32; w]; h],
//     }
//     for ai in a {
//         for aij in ai {
//             if aij == 0 {
//                 print!(".");
//             } else {
//                 print!("{}", std::char::from_u32('A' as u32 + aij - 1).unwrap());
//             }
//         }
//         println!("");
//     }
// }
