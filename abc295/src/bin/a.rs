use proconio::input;
// use proconio::marker::String;
fn main() {
    input! {
        n: usize,
        w: [String; n]
    }
    for wi in w {
        if wi == "and" || wi == "not" || wi == "that" || wi == "the" || wi == "you" {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
