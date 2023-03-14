use proconio::input;
fn lower_bound(nums: &Vec<usize>, val: usize) -> Result<usize, &str> {
    let mut first: i32 = -1;
    let mut last: i32 = nums.len() as i32;
    while last - first > 1 {
        let mid: i32 = first + (last - first) / 2;
        if nums.get(mid as usize).unwrap() < &val {
            first = mid;
        } else {
            last = mid;
        }
    }
    return Ok(last as usize);
}
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }
    a.sort();
    for xi in x {
        let ans = lower_bound(&a, xi).unwrap();
        println!("{}", ans);
        //     let mut kmin = 0;
        //     let mut kmax = n - 1;
        //     let mut cnt = 0;
        //     println!("xi = {} ----", xi);
        //     loop {
        //         cnt += 1;
        //         let k = (kmin + kmax) / 2;
        //         let ak = match a.get(k) {
        //             Some(num) => num,
        //             None => panic!("None {} {} {}", kmin, kmax, k),
        //         };
        //         println!(
        //             "cnt: {}, kmin: {}, kmax: {}, k: {} a[k]: {}",
        //             cnt, kmin, kmax, k, a[k]
        //         );
        //         if a[k] == xi {
        //             println!("{}", k);
        //             break;
        //         } else if kmin > kmax {
        //             println!("{}", k + 1);
        //             break;
        //         } else if a[k] < xi {
        //             kmin = k + 1;
        //         } else if a[k] > xi {
        //             kmax = k - 1;
        //         }
        //         if cnt >= 10 {
        //             break;
        //         }
        //     }
    }
}
