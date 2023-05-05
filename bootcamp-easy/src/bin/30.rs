use proconio::input;
fn main() {
    input! {(x1, y1, x2, y2): (i32, i32, i32, i32)}
    let (dx, dy) = (x2 - x1, y2 - y1);
    // 原点以外の点を中心とした回転の公式を用いる
    // x3, y3 -> 点2を中心に点1から90度回転
    let (x3, y3) = (x2 - dy, y2 + dx);
    // x4, y4 -> 点1を中心に点2から90度回転
    let (x4, y4) = (x1 - dy, y1 + dx);
    println!("{} {} {} {}", x3, y3, x4, y4);
}
