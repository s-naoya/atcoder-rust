use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        (n, m): (usize, usize),
        ab: [[usize; 2]; m]
    }
    let mut ans = vec![0; n];
    for abi in ab {
        for a in abi {
            ans[a - 1] += 1;
        }
    }
    for ansi in ans {
        println!("{}", ansi);
    }
}
// 3:33
