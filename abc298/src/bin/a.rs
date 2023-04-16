use proconio::{input, marker::Chars};
fn main() {
    input! {
        _: usize,
        s: Chars
    }
    let mut flag = false;
    let mut ans = "Yes";
    for si in s {
        if si == 'o' {
            flag = true;
        }
        if si == 'x' {
            ans = "No";
            break;
        }
    }
    if ans == "Yes" && !flag {
        ans = "No";
    }
    println!("{}", ans);
}
