use proconio::{input, marker::Chars};
fn main() {
    input! {s: Chars, t: Chars}
    let mut cnt_1 = vec![0; 26];
    let mut cnt_2 = vec![0; 26];
    let mut a_1 = 0;
    let mut a_2 = 0;
    for i in 0..s.len() {
        if s[i] == '@' {
            a_1 += 1;
        } else {
            cnt_1[s[i] as usize - 97] += 1;
        }
        if t[i] == '@' {
            a_2 += 1;
        } else {
            cnt_2[t[i] as usize - 97] += 1;
        }
    }
    let mut ans = "Yes";
    for i in 0..26 {
        if cnt_1[i] == cnt_2[i] {
            continue;
        } else if cnt_1[i] < cnt_2[i] {
            if i == 0 || i == 2 || i == 3 || i == 4 || i == 14 || i == 17 || i == 19 {
                a_1 -= cnt_2[i] - cnt_1[i];
            } else {
                ans = "No";
                break;
            }
        } else {
            if i == 0 || i == 2 || i == 3 || i == 4 || i == 14 || i == 17 || i == 19 {
                a_2 -= cnt_1[i] - cnt_2[i];
            } else {
                ans = "No";
                break;
            }
        }
        if a_1 < 0 || a_2 < 0 {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}
