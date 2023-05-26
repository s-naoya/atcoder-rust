use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {(n, m): (usize, usize), s: [Chars; n]}
    let mut it = vec![];
    for i in 0..n {
        it.push(i);
    }
    let mut ans = "No";
    for i in it.iter().permutations(n) {
        let mut nno = true;
        for n in 0..n - 1 {
            let mut no = 0;
            for j in 0..m {
                if s[*i[n]][j] == s[*i[n + 1]][j] {
                    continue;
                } else {
                    no += 1;
                }
            }
            if no != 1 {
                nno = false;
                break;
            }
        }
        if nno {
            ans = "Yes";
            break;
        }
    }
    println!("{}", ans);
}
