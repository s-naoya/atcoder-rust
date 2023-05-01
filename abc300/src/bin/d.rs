use proconio::input;
fn main() {
    input! {n: u128}
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
                sosu.push(i as u128);
            }
        }
        sosu
    };
    let mut ans = 0;
    for i in 0..sosu.len() {
        for j in i+1..sosu.len() {
            for k in j+1..sosu.len() {
                let v: u128 = sosu[i].pow(2) * sosu[j] * sosu[k].pow(2);
                if v <= n {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
