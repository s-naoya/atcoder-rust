use proconio::input;

fn check(a: &Vec<usize>, t: usize) -> usize {
    let mut ans = 0;
    for ai in a {
        ans += t / ai;
    }
    return ans;
}

fn binary_search(a: &Vec<usize>, k: usize) -> usize {
    let mut first = 0;
    let mut last = 1000000000;
    while last - first > 1 {
        let mid = first + (last - first) / 2;
        if check(&a, mid) < k {
            first = mid;
        } else {
            last = mid;
        }
    }
    return last;
}

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }
    println!("{}", binary_search(&a, k));
}
