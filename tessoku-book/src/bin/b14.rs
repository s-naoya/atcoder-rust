use proconio::input;
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }
    let a1 = Vec::from(&a[0..n / 2]);
    let a2 = Vec::from(&a[n / 2..n]);
    let mut b1 = Vec::new();
    let mut b2 = Vec::new();
    // bit全探索
    // 0..1 << a1.len() = 1を左にn/2ビットシフトした数-1まで = 2^n-1
    // ex n/2 = 3 のとき、0..0b1000 = 8-1
    for bit in 0..(1 << a1.len()) {
        let mut sum = 0;
        // bitを2進数としたときのの各桁を取り出す
        for i in 0..n {
            let num = (bit / (1 << i)) % 2; // 1 or 0
            if num == 1 {
                // 1のとき、処理を行う
                sum += a1[i];
            }
        }
        b1.push(sum);
    }
    for bit in 0..(1 << a2.len()) {
        let mut sum = 0;
        // bitを2進数としたときのの各桁を取り出す
        for i in 0..n {
            let num = (bit / (1 << i)) % 2; // 1 or 0
            if num == 1 {
                sum += a2[i];
            }
        }
        b2.push(sum);
    }
    b2.sort();
    for b1i in b1 {
        match b2.binary_search(&(k - b1i)) {
            Ok(_) => {
                println!("Yes");
                return;
            }
            Err(_) => continue,
        }
    }
    println!("No");
}
