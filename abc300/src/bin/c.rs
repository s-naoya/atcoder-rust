use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        (h, w): (usize, usize),
        c: [Chars; h]
    }
    let mut ans = vec![0; std::cmp::min(h, w)];
    for i in 2..h {
        for j in 2..w {
            let mut n = 0;
            while check(&c, i - 1, j - 1, n + 1) {
                n += 1;
                if i + n >= h || j + n >= w {
                    break;
                }
            }
            if n != 0 {
                ans[n - 1] += 1;
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}

fn check(c: &Vec<Vec<char>>, a: usize, b: usize, n: usize) -> bool {
    if c[a][b] != '#' {
        return false;
    }
    for d in 1..=n {
        if c[a + d][b + d] != '#'
            || c[a + d][b - d] != '#'
            || c[a - d][b + d] != '#'
            || c[a - d][b - d] != '#'
        {
            return false;
        }
    }

    return true;
}
