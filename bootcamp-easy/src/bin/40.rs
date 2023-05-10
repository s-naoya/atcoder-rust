use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(a, b): (i64, i64)}
    if a <= 0 && b >= 0 {
        println!("Zero");
    } else if a > 0 && b > 0 {
        println!("Positive");
    } else {
        if (a - b) % 2 == 0 {
            println!("Negative");
        } else {
            println!("Positive");
        }
    }
}
// 5:38
