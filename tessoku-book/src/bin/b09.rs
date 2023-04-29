use proconio::input;
fn main() {
    input! {n: usize}
    let mut wa = vec![vec![0; 1502]; 1502];
    for _ in 0..n {
        input! {(a, b, c, d): (usize, usize, usize, usize)}
        wa[a][b] += 1;
        wa[c][d] += 1;
        wa[a][d] -= 1;
        wa[c][b] -= 1;
    }
    for i in 0..1502 {
        for j in 1..1502 {
            wa[i][j] += wa[i][j - 1];
        }
    }
    for i in 1..1502 {
        for j in 0..1502 {
            wa[i][j] += wa[i - 1][j];
        }
    }
    let mut ans = 0;
    for i in 0..=1500 {
        for j in 0..=1500 {
            if wa[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
