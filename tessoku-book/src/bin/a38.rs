use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        (d, n): (usize, usize),
        lrh: [(usize, usize, usize); n],
    }
    let mut day = vec![24; d];
    for lrhi in lrh {
        for i in lrhi.0-1..lrhi.1 {
            if day[i] > lrhi.2 {
                day[i] = lrhi.2;
            }
        }
    }
    let ans: usize = day.iter().sum();
    println!("{}", ans);
}
