use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        x: [[usize; w]; h],
        q: usize,
        abcd: [[usize; 4]; q],
    }
    let mut z = vec![vec![0; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            z[i][j] = z[i][j - 1] + x[i - 1][j - 1];
        }
    }
    for j in 1..=w {
        for i in 1..=h {
            z[i][j] += z[i - 1][j];
        }
    }
    for abcdk in abcd {
        println!(
            "{}",
            z[abcdk[2]][abcdk[3]] + z[abcdk[0] - 1][abcdk[1] - 1]
                - z[abcdk[2]][abcdk[1] - 1]
                - z[abcdk[0] - 1][abcdk[3]]
        );
    }
}
