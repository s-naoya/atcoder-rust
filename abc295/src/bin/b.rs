use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        (r, c): (usize, usize),
        b: [Chars; r],
    }
    let mut ans = b.clone();
    for i in 0..r {
        for j in 0..c {
            if b[i][j] == '.' || b[i][j] == '#' {
                continue;
            } else {
                let num = b[i][j] as i32 - 48;
                ans[i][j] = '.';
                for ni in 0..r {
                    for nj in 0..c {
                        let ai = ((i as i32 - ni as i32) as i32).abs();
                        let bi = ((j as i32 - nj as i32) as i32).abs();
                        if (ai + bi) as i32 <= num {
                            ans[ni][nj] = '.';
                        }
                    }
                }
            }
        }
    }
    for i in 0..r {
        println!("{}", ans[i].iter().join(""));
    }
}
