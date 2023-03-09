use proconio::input;
fn main() {
    input! {n: usize}
    let mut ans = 0;
    for a in 1..n {
        println!("a: {}", a);
        for b in 1..n {
            println!("b: {}", b);
            if a*b >= n {continue;}
            let cd = n - a*b;
            let fac = factorize(cd);
            // let fac: Vec<(usize, usize)> = Vec::new();
            println!("cd: {}, fac: {:?}", cd, fac);
            ans += fac.len();
        }
    }
    println!("{}", ans);
}

fn factorize(nn: usize) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();
    let mut i = 2;
    let mut n = nn;
    loop {
        if n % i != 0 {
            continue;
        }
        let mut e = 0;
        while n % i == 0 {
            e += 1;
            n /= i;
        }
        ans.push((i, e));

        if i*i > n {
            break;
        }
        i += 1;
    }
    if n != 1 {
        ans.push((n, 1));
    }
    ans
}
