use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut bar = vec![];
    let mut kome = vec![];
    for i in 0..n {
        if s[i] == '|' {
            bar.push(i);
        } else if s[i] == '*' {
            kome.push(i);
        }
    }
    // println!("{:?} {:?}", bar, kome);

    let ans = if bar[0] < kome[0] && kome[0] < bar[1] {
        "in"
    } else {
        "out"
    };
    println!("{}", ans);
}
