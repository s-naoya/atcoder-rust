use proconio::input;

fn main() {
    input! {
        nab: [u32; 3],
    }
    let mut ans: u32 = 0;
    for n in 0..nab[0]+1 {
        let ns = n.to_string();
        let mut nsum: u32 = 0;
        for ni in ns.chars() {
            nsum = nsum + ni.to_digit(10).unwrap();
        }
        if nsum >= nab[1] && nsum <= nab[2] {
            ans = ans + n;
        }
    }
    println!("{}", ans);
}
