use std::collections::VecDeque;

use proconio::input;
fn main() {
    input! {q: usize}
    let m = 998244353;
    let mut s = VecDeque::new();
    s.push_back(1);
    let mut r = 1;

    for _ in 0..q {
        input! {t: usize}
        if t == 1 {
            input! {x: char}
            let x = x as usize - 48;
            s.push_back(x);
            r = (r * 10 + x) % m;
        } else if t == 2 {
            let y = s.pop_front().unwrap();
            r = (r - y * 10_usize.pow(s.len() as u32) % m) % m;
        } else if t == 3 {
            println!("{}", r);
            // let mut num = 0u128;
            // let slen = s.len();
            // for i in 0..slen {
            //     num += ((s[i] as u128 - 48) * 10_u128.pow(slen as u32 - i as u32 - 1)) as u128;
            //     // print!("{} ", num);
            // }
            // // print!("{:?} -> {} --> ", s, num);
            // println!("{}", num % 998244353);
        }
    }
}
