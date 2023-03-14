use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }
    let mut ab: Vec<usize> = Vec::new();
    let mut cd: Vec<usize> = Vec::new();
    for ai in &a {
        for bi in &b {
            ab.push(ai + bi);
        }
    }
    for ci in &c {
        for di in &d {
            cd.push(ci + di);
        }
    }
    cd.sort();
    for abi in ab {
        match cd.binary_search(&(k - abi)) {
            Ok(_) => {
                println!("Yes");
                return;
            }
            Err(_) => continue,
        }
    }
    println!("No");
}
