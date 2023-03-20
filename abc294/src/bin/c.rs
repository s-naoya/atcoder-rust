use itertools::Itertools;
use proconio::input;
fn main() {
    // ans_o();
    // return;
    input! {
        (n, m): (usize, usize),
        a: [usize; n],
        b: [usize; m],
    }
    let mut c = Vec::new();
    let mut aa = a.clone();
    let mut bb = b.clone();
    c.append(&mut aa);
    c.append(&mut bb);
    c.sort();

    let mut ans_a = Vec::with_capacity(n);
    let mut ans_b = Vec::with_capacity(m);
    let (mut i, mut j) = (0, 0);
    for k in 0..(n + m) {
        if i < n && c[k] == a[i] {
            ans_a.push(k + 1);
            i += 1;
        } else if j < m && c[k] == b[j] {
            ans_b.push(k + 1);
            j += 1;
        }
    }
    println!("{}", ans_a.iter().join(" "));
    println!("{}", ans_b.iter().join(" "));
}

// fn ans_o() {
//     input! {
//         (n, m): (usize, usize),
//         a: [usize; n],
//         b: [usize; m],
//     }
//     let mut c: Vec<usize> = Vec::new();
//     c.extend(&a);
//     c.extend(&b);
//     c.sort();

//     let mut ans_a = Vec::with_capacity(n);
//     let mut ans_b = Vec::with_capacity(m);
//     let (mut i, mut j) = (0, 0);
//     for k in 0..(n + m) {
//         if i < n && c[k] == a[i] {
//             ans_a.push(k + 1);
//             i += 1;
//         } else if j < m && c[k] == b[j] {
//             ans_b.push(k + 1);
//             j += 1;
//         }
//     }
//     println!(
//         "{}",
//         ans_a
//             .iter()
//             .map(|x| x.to_string())
//             .collect::<Vec<_>>()
//             .join(" ")
//     );
//     println!(
//         "{}",
//         ans_b
//             .iter()
//             .map(|x| x.to_string())
//             .collect::<Vec<_>>()
//             .join(" ")
//     );
// }
