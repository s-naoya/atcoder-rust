use proconio::input;
fn main() {
    ans1();
}
fn ans1() {
    input! {s: usize}
    let f = |n: usize| {
        if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        }
    };
    let mut a = Vec::with_capacity(1000001); // HashSetを使ったほうが検索が速い
    a.push(s);
    for i in 1..=1000000 {
        a.push(f(a[i - 1]));
        for j in 0..a.len() - 1 {
            if a[j] == a[a.len() - 1] {
                println!("{}", i + 1);
                return;
            }
        }
    }
}
