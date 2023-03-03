use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }
    let mut ans = 0;
    'abc: loop {
        for am in &mut a {
            if *am % 2 == 0 {
                *am = *am / 2;
            } else {
                break 'abc;
            }
        }
        ans = ans + 1;
    }
    println!("{}", ans);
}
