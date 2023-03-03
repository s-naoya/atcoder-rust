use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }
    a.sort();
    let (mut alice, mut bob) = (0, 0);
    loop {
        alice = alice + a.pop().unwrap();
        if a.len() == 0 {
            break;
        }
        bob = bob + a.pop().unwrap();
        if a.len() == 0 {
            break;
        }
    }
    println!("{}", alice - bob);
}
// 09:02.97