use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {(_, a, b): (usize, usize, usize), s: Chars}
    let mut b_ord = 1;
    let mut ord = 1;
    // そのまんま実装する
    // 境界値条件に注意する 今回の場合は"満たない場合"なので<=
    for si in s {
        match si {
            'a' => {
                if ord <= a + b {
                    println!("Yes");
                    ord += 1;
                } else {
                    println!("No");
                }
            }
            'b' => {
                if ord <= a + b && b_ord <= b {
                    println!("Yes");
                    ord += 1;
                    b_ord += 1;
                } else {
                    println!("No");
                }
            }
            _ => {
                println!("No");
            }
        }
    }
}
// 6:29
