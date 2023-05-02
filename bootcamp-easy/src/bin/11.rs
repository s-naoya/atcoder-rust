use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {n: usize}
    let mut ans = 0;
    let mut cnta = 0;
    for i in 1..=n {
        let mut cnt = 0;
        let mut ia = i;
        while ia % 2 == 0 {
            cnt += 1;
            ia /= 2;
        }
        if cnt >= cnta {
            ans = i;
            cnta = cnt;
        }
    }
    println!("{}", ans);
}
// 10:05
