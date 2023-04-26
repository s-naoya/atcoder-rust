use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {(n, q): (usize, usize), a: [usize; n], lr: [(usize, usize); q]}
    let mut ruisekiwa = vec![0; n + 1];
    for (i, ai) in a.iter().enumerate() {
        ruisekiwa[i + 1] = *ai + ruisekiwa[i];
    }
    for (l, r) in lr {
        println!("{}", ruisekiwa[r] - ruisekiwa[l - 1]);
    }
}
