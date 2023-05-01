use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize}
    // 全探索でよかった
    for i in 1..=50000 {
        if (i as f64 * 1.08).floor() as usize == n {
            println!("{}", i);
            return;
        }
    }
    // let a = (n as f64 / 1.08).round() as usize;
    // for aa in a - 5..a + 5 {
    //     if (aa as f64 * 1.08).floor() as usize == n {
    //         println!("{}", aa);
    //         return;
    //     }
    // }

    println!(":(");
}
