use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [[usize; 2]; n],
    }
    let mut ruiseki = vec![0; d + 1];
    for lrn in lr {
        ruiseki[lrn[0] - 1] += 1;
        ruiseki[lrn[1]] -= 1;
    }
    let mut before = 0;
    for i in 0..d {
        println!("{}", before + ruiseki[i]);
        before += ruiseki[i];
    }
}
