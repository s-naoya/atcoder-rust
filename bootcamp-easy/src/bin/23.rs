use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        n: usize,
        (d, x): (usize, usize),
        a: [usize; n]
    }
    // let mut ans = 0;
    // for ai in a {
    //     ans += sumc(ai, d, 1);
    // }
    let ans = a.iter().fold(0, |ai, &aj| ai + sumc(aj, d, 1));
    println!("{}", x + ans);
}
fn sumc(ai: usize, d: usize, i: usize) -> usize {
    match i * ai + 1 <= d {
        true => sumc(ai, d, i + 1),
        false => i,
    }
    // let mut i = 1;
    // let mut ans = 1;
    // while i * ai + 1 <= d {
    //     ans += 1;
    //     i += 1;
    // }
    // ans
}
// 5:23
