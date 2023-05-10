use proconio::input;
fn main() {
    ans1();
}

fn ans1() {
    input! {n: usize, a: [usize; n]}
    let mut at = a.clone();
    at.sort_unstable();
    for i in 0..n {
        let x = if a[i] == *at.last().unwrap() {
            at[at.len() - 2]
        } else {
            *at.last().unwrap()
        };
        println!("{}", x);
    }
}
// 11:32
