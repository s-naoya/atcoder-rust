use proconio::input;
fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        c: [usize; n]
    }
    let ab = a + b;
    for i in 0..n {
        if c[i] == ab {
            println!("{}", i + 1);
            break;
        }
    }
}
