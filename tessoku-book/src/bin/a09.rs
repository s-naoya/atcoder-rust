use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        abcd: [(usize, usize, usize, usize); n]
    }
    let mut wa = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in abcd {
        wa[a - 1][b - 1] += 1;
        wa[a - 1][d] -= 1;
        wa[c][b - 1] -= 1;
        wa[c][d] += 1;
    }
    for i in 0..h {
        for j in 1..w {
            wa[i][j] += wa[i][j - 1];
        }
    }
    for i in 1..h {
        for j in 0..w {
            wa[i][j] += wa[i - 1][j];
        }
    }
    for i in 0..h {
        wa[i].pop();
        println!("{}", wa[i].iter().join(" "));
    }
}
