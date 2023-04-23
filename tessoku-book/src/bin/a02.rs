use proconio::input;
fn main() {
    input! {
        (n, x): (usize, usize),
        a: [usize; n]
    }
    println!(
        "{}",
        if a.iter().any(|&ai| ai == x) {
            "Yes"
        } else {
            "No"
        }
    );
}
