use proconio::{input, marker::Chars};
fn main() {
    ans1();
}
fn ans1() {
    input! {a: Chars, b: Chars}
    let mut ans = "EQUAL";
    if a.len() > b.len() {
        ans = "GREATER";
    } else if a.len() < b.len() {
        ans = "LESS";
    } else {
        for (&ai, &bi) in a.iter().zip(b.iter()) {
            let ain = ai as i32 - '0' as i32;
            let bin = bi as i32 - '0' as i32;
            if ain > bin {
                ans = "GREATER";
                break;
            } else if ain < bin {
                ans = "LESS";
                break;
            } else {
                continue;
            }
        }
    }
    println!("{}", ans);
}
// 5:32
