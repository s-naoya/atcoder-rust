use proconio::input;
fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        a: [usize; n]
    }
    let mut grundy = vec![0; 100_001];
    for i in 0..=100_000 {
        let mut transit = [false; 3];
        if i >= x {
            transit[grundy[i - x]] = true;
        }
        if i >= y {
            transit[grundy[i - y]] = true;
        }
        grundy[i] = if !transit[0] {
            0
        } else if !transit[1] {
            1
        } else {
            2
        }
    }
    let ans = a.iter().map(|&ai| grundy[ai]).fold(0, |acc, ai| acc ^ ai);
    println!("{}", if ans == 0 { "Second" } else { "First" });
}
