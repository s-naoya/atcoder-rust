use proconio::{fastout, input};
fn main() {
    ans1();
}
#[fastout]
fn ans1() {
    input! {(a, b, k): (isize, isize, isize)}
    // let finish = if a + k < b { a + k } else { b };
    // for i in a..finish {
    //     println!("{}", i);
    // }
    // let start = if finish < b - k { b - k + 1 } else { finish };
    // for i in start..=b {
    //     println!("{}", i);
    // }
    for i in a..=b {
        if i < a + k || i > b - k {
            println!("{}", i);
        }
    }
}
// 16:16
