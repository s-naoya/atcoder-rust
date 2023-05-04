use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        (aa, bb, m): (usize, usize, usize),
        a: [usize; aa],
        b: [usize; bb],
        xyc: [(usize, usize, usize); m]
    }
    // let mut min = 1_000_000;
    // for (x, y, c) in xyc {
    //     min = std::cmp::min(min, a[x - 1] + b[y - 1] - c);
    // }
    // let min = std::cmp::min(min, a.iter().min().unwrap() + b.iter().min().unwrap());
    let min = xyc
        .iter()
        .fold(1_000_000, |m, &(x, y, c)| {
            std::cmp::min(m, a[x - 1] + b[y - 1] - c)
        })
        .min(a.iter().min().unwrap() + b.iter().min().unwrap());
    println!("{}", min);
}
// 7:33
