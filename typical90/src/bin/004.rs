use itertools::Itertools;
use proconio::input;
use std::vec;

fn main() {
    input! {
        hw: [usize; 2],
        a: [[usize; hw[1]]; hw[0]],
    }
    let mut ans: Vec<Vec<usize>> = vec![vec![0; hw[1]]; hw[0]];
    let mut note_h = vec![0; hw[0]];
    let mut note_w = vec![0; hw[1]];
    for i in 0..hw[0] {
        for j in 0..hw[1] {
            note_h[i] += a[i][j];
            note_w[j] += a[i][j];
        }
    }
    for i in 0..hw[0] {
        for j in 0..hw[1] {
            ans[i][j] = note_h[i] + note_w[j] - a[i][j];
        }
    }
    println!("{}", ans.iter().map(|x| x.iter().join(" ")).join("\n"));
}
// 39m26s