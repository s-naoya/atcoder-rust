use proconio::input;
fn main() {
    input! {n: String}
    println!("{}", i64::from_str_radix(&n, 2).unwrap());
}
