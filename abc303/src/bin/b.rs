use std::{cmp::max, cmp::min, collections::HashSet};

// use itertools::Itertools;
use proconio::input;
fn main() {
    // input! {(n, m): (usize, usize), a: [[usize; n]; m]}
    // let mut set = HashSet::new();
    // for i in 0..m {
    //     for j in 0..n - 1 {
    //         set.insert((a[i][j], a[i][j + 1]));
    //     }
    // }
    // let mut ans = 0;
    // for x in (1..=n).permutations(2) {
    //     let xx = (x[0], x[1]);
    //     let xy = (x[1], x[0]);
    //     if set.contains(&xx) {
    //     } else if set.contains(&xy) {
    //     } else {
    //         ans += 1;
    //     }
    // }
    // println!("{}", ans / 2);
    ans1();
}
fn ans1() {
    input! {(n, m): (usize, usize), a: [[usize; n]; m]}
    let mut set = HashSet::new();
    for i in 0..m {
        for j in 0..n - 1 {
            let (x, y) = (min(a[i][j], a[i][j + 1]), max(a[i][j], a[i][j + 1]));
            set.insert((x, y));
        }
    }
    let mut ans = 0;
    for i in 1..=n {
        for j in i + 1..=n {
            if !set.contains(&(i, j)) {
                ans += 1;
            }
        }
    }
    //     let xx = (x[0], x[1]);
    //     let xy = (x[1], x[0]);
    //     if set.contains(&xx) {
    //     } else if set.contains(&xy) {
    //     } else {
    //         ans += 1;
    //     }
    // }
    println!("{}", ans);
}
