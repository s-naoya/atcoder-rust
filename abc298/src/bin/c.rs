use itertools::Itertools;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut box2 = vec![vec![]; n + 1];
    let mut box3 = vec![vec![]; 200001];

    for _ in 0..q {
        input! {t: usize}
        if t == 1 {
            input! {(i, j): (usize, usize)}
            box2[j].push(i);
            box3[i].push(j);
        } else if t == 2 {
            input! {i: usize}
            // println!("{:?}", box2);
            let mut cbox2 = box2[i].clone();
            cbox2.sort_unstable();
            println!("{}", cbox2.iter().join(" "));
        } else if t == 3 {
            input! {j: usize}
            // println!("{:?}", box3);
            let mut cbox3 = box3[j].clone();
            cbox3.sort_unstable();
            cbox3.dedup();
            println!("{}", cbox3.iter().join(" "));
        }
    }
}
