use proconio::input;
fn main() {
    input! {n: usize}
    let sosu = {
        let max = 300_000;
        let mut deleted = vec![false; max];
        let mut sosu = Vec::new();
        for i in 2..=((max as f64).sqrt() as usize) {
            if deleted[i] {
                continue;
            }
            for j in ((2 * i)..max).step_by(i) {
                deleted[j] = true;
            }
        }
        for i in 2..max {
            if !deleted[i] {
                sosu.push(i);
            }
        }
        sosu
    };
    println!("{} {:?}", n, sosu);
}
