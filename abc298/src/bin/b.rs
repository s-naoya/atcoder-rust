use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n]
    }
    let mut ans = "No";
    let mut aa = a;
    for _ in 0..4 {
        let mut now_ans = true;
        for i in 0..n {
            for j in 0..n {
                if aa[i][j] == 1 && aa[i][j] != b[i][j] {
                    now_ans = false;
                }
            }
        }
        if now_ans {
            ans = "Yes";
            break;
        } else {
            let mut aaa = vec![vec![0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    aaa[i][j] = aa[n - 1 - j][i];
                }
            }
            aa = aaa;
        }
    }
    println!("{}", ans);
}
