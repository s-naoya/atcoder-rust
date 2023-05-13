use itertools::Itertools;
use proconio::input;
fn main() {
    input! {n: usize, mut a: [i32; n]}
    let mut i = 0;
    while i < a.len() - 1 {
        // println!("{}: {}", i, a.iter().join(" "));
        if a[i] - a[i + 1] > 1 {
            for j in (a[i + 1] + 1..a[i]).rev() {
                i += 1;
                a.insert(i, j);
                // println!("{}, {}: {}", i, j, a.iter().join(" "));
            }
            // i = 0;
        }
        if a[i] - a[i + 1] < -1 {
            for j in a[i] + 1..a[i + 1] {
                i += 1;
                a.insert(i, j);
                // println!("{}, {}: {}", i, j, a.iter().join(" "));
            }
            // i = 0;
        }
        i += 1;
    }
    println!("{}", a.iter().join(" "));
}
