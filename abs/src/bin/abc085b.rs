use proconio::input;

fn main() {
    input! {
        n: i32,
        mut d: [i32; n],
    }
    d.sort();
    let mut ans = 0;
    let mut before = 0;
    for di in d {
        if di != before {
            ans += 1;
        }
        before = di;
    }

    println!("{}", ans);
}
// 10:44.37