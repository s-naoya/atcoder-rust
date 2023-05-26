use proconio::{input, marker::Chars};
fn main() {
    input! {(h, w): (usize, usize), s: [Chars; h]}
    let ans = ans2(h, w, &s);
    for i in ans {
        println!("{} {}", i.0, i.1);
    }
}
fn ans2(h: usize, w: usize, s: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    // シークワーズ
    // 基準文字s_{si, sj}から全方向=8方向に5文字探索し、条件に一致しているかを全探索する
    // di_v, dj_v -> v方向に探索する時、iとjはどの方向に増減するか
    // eg) 左上方向に探索する時(v=0)、5文字探索する間iとjはともに-1されていく
    let di: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dj: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    let n = ['s', 'n', 'u', 'k', 'e'];
    for si in 0..h {
        for sj in 0..w {
            for v in 0..8 {
                let mut i = si;
                let mut j = sj;
                for k in 0..5 {
                    if i >= h || j >= w {
                        break;
                    }
                    if s[i][j] != n[k] {
                        break;
                    }
                    if k == 4 {
                        i = si;
                        j = sj;
                        let mut ans = vec![];
                        for _ in 0..5 {
                            ans.push((i + 1, j + 1));
                            if ((i as i32) + di[v]) >= 0 {
                                i = (i as i32 + di[v]) as usize
                            } else {
                                break;
                            }
                            if ((j as i32) + dj[v]) >= 0 {
                                j = (j as i32 + dj[v]) as usize
                            } else {
                                break;
                            }
                        }
                        return ans;
                    }
                    if ((i as i32) + di[v]) >= 0 {
                        i = (i as i32 + di[v]) as usize
                    } else {
                        break;
                    }
                    if ((j as i32) + dj[v]) >= 0 {
                        j = (j as i32 + dj[v]) as usize
                    } else {
                        break;
                    }
                }
            }
        }
    }
    return Vec::new();
}
// fn ans1(h: usize, w: usize, s: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
//     let n = ['s', 'n', 'u', 'k', 'e'];
//     let mut ans = vec![];

//     for i in 0..h {
//         for j in 0..w - 4 {
//             for k in 0..5 {
//                 if s[i][j + k] == n[k] {
//                     ans.push((i + 1, j + k + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }
//     for i in 0..h {
//         for j in (4..w).rev() {
//             for k in 0..5 {
//                 if s[i][j - k] == n[k] {
//                     ans.push((i + 1, j - k + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }
//     for j in 0..w {
//         for i in 0..h - 4 {
//             for k in 0..5 {
//                 if s[i + k][j] == n[k] {
//                     ans.push((i + k + 1, j + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }
//     for j in 0..w {
//         for i in (4..h).rev() {
//             for k in 0..5 {
//                 if s[i - k][j] == n[k] {
//                     ans.push((i - k + 1, j + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }
//     for i in 0..h - 4 {
//         for j in 0..w - 4 {
//             for k in 0..5 {
//                 if s[i + k][j + k] == n[k] {
//                     ans.push((i + k + 1, j + k + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }
//     for i in (4..h).rev() {
//         for j in (4..w).rev() {
//             for k in 0..5 {
//                 if s[i - k][j - k] == n[k] {
//                     ans.push((i - k + 1, j - k + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }

//     for i in 0..h - 4 {
//         for j in (4..w).rev() {
//             for k in 0..5 {
//                 if s[i + k][j - k] == n[k] {
//                     ans.push((i + k + 1, j - k + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }
//     for i in (4..h).rev() {
//         for j in 0..w - 4 {
//             for k in 0..5 {
//                 if s[i - k][j + k] == n[k] {
//                     ans.push((i - k + 1, j + k + 1));
//                 } else {
//                     ans.clear();
//                     break;
//                 }
//             }
//             if ans.len() == 5 {
//                 return ans;
//             }
//         }
//     }
//     return ans;
// }
