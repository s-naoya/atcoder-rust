use std::collections::HashMap;

use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: u128}
    let mut memo = HashMap::new();
    memo.insert(0, 2);
    memo.insert(1, 1);
    println!("{}", l(n, &mut memo));
}
fn l(i: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    match memo.contains_key(&i) {
        true => *memo.get(&i).unwrap(),
        false => {
            let a = l(i - 1, memo) + l(i - 2, memo);
            memo.insert(i, a);
            a
        }
    }
}
// 6:42
