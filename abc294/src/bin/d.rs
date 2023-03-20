use proconio::input;
use std::collections::BTreeSet;
// use std::io;
fn ans_o() {
    input! {
        (_, q): (usize, usize)
    }
    let mut no1: usize = 1;
    let mut yo: BTreeSet<usize> = BTreeSet::new();
    for _ in 0..q {
        input! {
            e0: usize,
        }
        if e0 == 1 {
            yo.insert(no1);
            no1 += 1;
        } else if e0 == 2 {
            input! {e1: usize}
            yo.remove(&e1);
        } else if e0 == 3 {
            println!("{}", yo.iter().next().unwrap());
        }
    }
}
fn main() {
    ans_o();
    return;
    // let mut read = String::new();
    // io::stdin().read_line(&mut read).unwrap();
    // let read1: Vec<&str> = read.trim().split(' ').collect();
    // let _n: usize = read1[0].parse().unwrap();
    // let q: usize = read1[1].parse().unwrap();

    // let mut event = Vec::new();
    // for _ in 0..q {
    //     let mut read = String::new();
    //     io::stdin().read_line(&mut read).unwrap();
    //     let read1: Vec<&str> = read.trim().split(' ').collect();
    //     let mut e: Vec<usize> = Vec::new();
    //     e.push(read1[0].parse().unwrap());
    //     if e[0] == 2 {
    //         e.push(read1[1].parse().unwrap());
    //     }
    //     event.push(e);
    // }

    // let mut no1: usize = 1;
    // let mut yo: BTreeSet<usize> = BTreeSet::new();
    // for e in event {
    //     if e[0] == 1 {
    //         yo.insert(no1);
    //         no1 += 1;
    //     } else if e[0] == 2 {
    //         yo.remove(&e[1]);
    //     } else if e[0] == 3 {
    //         let min = yo.iter().next().unwrap();
    //         println!("{}", min);
    //     }
    // }
}
