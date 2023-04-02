use proconio::{input, marker::Chars};
fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let mut before = ' ';
    for si in s {
        if before == ' ' {
            before = si;
            continue;
        } else if before != si {
            before = si;
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
