use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {(a, b): (usize, usize)}
    let mut ans = 0;
    for i in a..=b {
        let is = i.to_string().chars().collect::<Vec<char>>();
        let mut ok = true;
        for j in 0..is.len() {
            if is[j] != is[is.len() - j - 1] {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
// 6:02
