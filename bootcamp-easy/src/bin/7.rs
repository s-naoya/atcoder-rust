use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {
        mut a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n]
    }
    for bi in b {
        for i in 0..3 {
            for j in 0..3 {
                if bi == a[i][j] {
                    a[i][j] = 0;
                }
            }
        }
    }
    let mut ans = "No";
    for i in 0..3 {
        let mut ans_i = true;
        for j in 0..3 {
            if a[i][j] != 0 {
                ans_i = false;
            }
        }
        if ans_i {
            ans = "Yes";
        }
    }
    for j in 0..3 {
        let mut ans_i = true;
        for i in 0..3 {
            if a[i][j] != 0 {
                ans_i = false;
            }
        }
        if ans_i {
            ans = "Yes";
        }
    }
    {
        let mut ans_i = true;
        for j in 0..3 {
            if a[j][j] != 0 {
                ans_i = false;
            }
        }
        if ans_i {
            ans = "Yes";
        }
    }
    {
        let mut ans_i = true;
        for j in 0..3 {
            if a[j][2 - j] != 0 {
                ans_i = false;
            }
        }
        if ans_i {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
// 7:17
