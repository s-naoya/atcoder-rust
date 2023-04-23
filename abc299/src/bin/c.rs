use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut max_l = 0;
    let mut start_i_tmp = 0;
    let mut is_count_now = false;
    for (i, si) in s.iter().enumerate() {
        if *si == 'o' {
            if is_count_now {
                continue;
            } else {
                is_count_now = true;
                start_i_tmp = i;
            }
        } else {
            if is_count_now {
                max_l = std::cmp::max(max_l, i - start_i_tmp);
                start_i_tmp = 0;
                is_count_now = false;
            } else {
                continue;
            }
        }
    }

    if is_count_now && start_i_tmp != 0 && s[start_i_tmp - 1] == '-' {
        max_l = std::cmp::max(max_l, n - start_i_tmp);
    }
    println!("{}", if max_l == 0 { -1 } else { max_l as i64 });
}
