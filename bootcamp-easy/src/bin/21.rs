use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {mut x: usize}
    loop {
        if sosu2(x) {
            println!("{}", x);
            break;
        } else {
            x += 1;
        }
    }
}
// fn sosu(x: usize) -> bool {
//     let mut ans = true;
//     for i in 2..=((x as f64).sqrt() as usize) {
//         if x % i == 0 {
//             ans = false;
//             break;
//         }
//     }
//     ans
// }
// 4:59
fn sosu2(x: usize) -> bool {
    match (2..=((x as f64).sqrt() as usize))
        .into_iter()
        .find(|&i| x % i == 0)
    {
        Some(_) => false,
        None => true,
    }
}
