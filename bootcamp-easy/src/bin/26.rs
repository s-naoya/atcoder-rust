use proconio::input;
fn main() {
    ans2();
}
// fn ans1() {
//     input! {mut h: usize}
//     let mut cnt = 1_u128;
//     while h > 1 {
//         h = h / 2;
//         cnt += cnt + 1;
//     }
//     println!("{}", cnt);
// }
// 11:31

fn ans2() {
    input! {h: u128}
    println!("{}", f(h));
}
fn f(h: u128) -> u128 {
    match h {
        1 => 1,
        _ => 2 * f(h / 2) + 1,
    }
}
