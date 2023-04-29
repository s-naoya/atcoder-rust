use std::vec;

use proconio::input;
fn main() {
    // 二次元の累積和
    input! {n: usize}
    let wh = 1500;
    let mut wa = vec![vec![0; wh + 1]; wh + 1];
    // 各座標にある点の数を計算する
    for _ in 0..n {
        input! {(x, y): (usize, usize)}
        wa[x][y] += 1;
    }
    // 二次元の累積和を計算
    // まず、y方向の累積和を計算する
    for i in 0..wh {
        for j in 0..wh {
            wa[i + 1][j + 1] += wa[i + 1][j];
        }
    }
    // 次に、x方向の累積和を計算する
    for j in 0..wh {
        for i in 0..wh {
            wa[i + 1][j + 1] += wa[i][j + 1];
        }
    }
    input! {q: usize}
    for _ in 0..q {
        input! {(a, b, c, d): (usize, usize, usize, usize)}
        // この数式で、座標(a, b)〜(c, d)間の合計値が計算できる
        let ans = wa[c][d] + wa[a - 1][b - 1] - wa[c][b - 1] - wa[a - 1][d];
        println!("{}", ans);
    }
}
