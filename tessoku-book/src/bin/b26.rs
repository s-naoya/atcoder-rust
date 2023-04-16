use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {n: usize}
    let mut deleted = vec![false; n + 1];

    let nsqrt = (n as f64).sqrt() as usize;
    for i in 2..=nsqrt {
        // print!("=== i : {} -> ", i);
        if deleted[i] {
            // println!("continue");
            continue;
        }
        // println!("sosuu");
        let mut j = i * 2;
        while j <= n {
            // println!("-> {} not sosuu", j);
            deleted[j] = true;
            j += i;
        }
    }
    for i in 2..=n {
        if !deleted[i] {
            println!("{}", i);
        }
    }
}
