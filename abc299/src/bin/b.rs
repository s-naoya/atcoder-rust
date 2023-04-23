use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        (n, t): (usize, usize),
        c: [usize; n],
        r: [usize; n]
    }
    let mut cr = HashMap::new();
    for i in 0..n {
        cr.entry(c[i]).or_insert(vec![]).push((r[i], i));
    }
    if cr.contains_key(&t) {
        let (_, i) = cr[&t].iter().max().unwrap();
        println!("{}", *i + 1);
    } else {
        let (_, i) = cr[&c[0]].iter().max().unwrap();
        println!("{}", *i + 1);
    }
}
