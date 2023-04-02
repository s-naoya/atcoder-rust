use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: [Chars; 8],
    }
    let ii = ['8', '7', '6', '5', '4', '3', '2', '1'];
    let jj = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for i in (0..8).rev() {
        for j in 0..8 {
            if s[i][j] == '*' {
                println!("{}{}", jj[j], ii[i]);
            }
        }
    }
}
