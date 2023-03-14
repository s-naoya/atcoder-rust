use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        a: [usize; n],
    }
    let mut imax = a.len() - 1;
    let mut imin = 0;
    loop {
        let i = (imax + imin) / 2;
        if a[i] == x {
            println!("{}", i + 1);
            break;
        } else if a[i] < x {
            imin = i + 1;
        } else if a[i] > x {
            imax = i - 1;
        }
    }
}
// 二分探索
