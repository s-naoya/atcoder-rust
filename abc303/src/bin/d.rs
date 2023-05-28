use proconio::{input, marker::Chars};
fn main() {
    input! {(x, y, z): (usize, usize, usize), s: Chars}
    let mut zip = vec![];
    let mut m = (0, 0); // a, A
    let mut upper = if s[0].is_ascii_uppercase() {
        true
    } else {
        false
    };
    {
        let mut now = s[0];
        let mut cnt = 0;
        for &si in &s {
            if si == now {
                cnt += 1;
            } else {
                zip.push(cnt);
                if si == 'A' {
                    m.0 += cnt
                } else {
                    m.1 += cnt
                };
                cnt = 1;
                now = si;
            }
        }
        zip.push(cnt);
        if s[s.len() - 1] == 'A' {
            m.1 += cnt
        } else {
            m.0 += cnt
        };
    }
    let mut cap = false;
    let mut ans = 0;
    for zi in zip {
        if upper {
            if cap {
                let no = x * m.1 + y * m.0;
                let push = z + y * m.1 + x * m.0;
                if no < push {
                    ans += x * zi;
                } else {
                    ans += z + y * zi;
                    cap = !cap;
                }
            } else {
                let no = y * m.1 + z * m.0;
                let push = z + x * m.1 + y * m.0;
                if no < push {
                    ans += y * zi;
                } else {
                    ans += z + x * zi;
                    cap = !cap;
                }
            }
            m.1 -= zi;
        } else {
            if cap {
                let no = x * m.1 + y * m.0;
                let push = z + y * m.1 + x * m.0;
                if no < push {
                    ans += x * zi;
                } else {
                    ans += z + y * zi;
                    cap = !cap;
                }
            } else {
                let no = y * m.1 + z * m.0;
                let push = z + x * m.1 + y * m.0;
                if no < push {
                    ans += y * zi;
                } else {
                    ans += z + x * zi;
                    cap = !cap;
                }
            }
            m.0 -= zi;
        }
        // if cap {
        //     // 大文字でCapが押されている
        //     ans += x * zi;
        // } else {
        //     let a = z + x * zi;
        //     let b = y * zi;
        //     if a < b {
        //         ans += a;
        //         cap = !cap;
        //     } else {
        //         ans += b;
        //     }
        // }
        // } else {
        //     if !cap {
        //         ans += x * zi;
        //     } else {
        //         let a = z + x * zi;
        //         let b = y * zi;
        //         if a < b {
        //             ans += a;
        //             cap = !cap;
        //         } else {
        //             ans += b;
        //         }
        //     }
        // }
        // println!("{} {}", zi, ans);
        upper = !upper;
    }
    println!("{}", ans);
}
