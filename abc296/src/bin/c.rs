use proconio::input;
fn main() {
    input! {
        (n, x): (usize, i64),
        mut a: [i64; n],
    }
    a.sort_unstable();
    for ai in &a {
        match a.binary_search(&(ai - x)) {
            Ok(_) => {
                println!("Yes");
                return;
            }
            Err(_) => {
                continue;
            }
        }
    }
    println!("No");
}
