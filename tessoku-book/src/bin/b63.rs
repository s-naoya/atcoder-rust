use std::{collections::VecDeque, vec};

// use itertools::enumerate;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        (rr, cc): (usize, usize),
        (sy, sx): (usize, usize),
        (gy, gx): (usize, usize),
        c: [Chars; rr]
    }
    let mut dist = vec![vec![-1; cc]; rr];
    let mut que = VecDeque::new();
    dist[sy - 1][sx - 1] = 0;
    que.push_back((sx - 1, sy - 1));
    let hou: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for h in hou.iter() {
            let to = ((pos.0 as i32 + h.0) as usize, (pos.1 as i32 + h.1) as usize);
            let posdist = dist[pos.0][pos.1] + 1;
            match dist.get_mut(to.0) {
                Some(distx) => match distx.get_mut(to.1) {
                    Some(x) => {
                        if c[to.0][to.1] == '.' && *x == -1 {
                            *x = posdist;
                            que.push_back(to);
                        }
                    }
                    None => continue,
                },
                None => continue,
            }
        }
    }
    // for (_, di) in enumerate(dist.iter()) {
    //     for (_, d) in enumerate(di.iter()) {
    //         if *d == -1 {
    //             print!("  #");
    //         } else {
    //             print!("{:>3}", *d);
    //         }
    //     }
    //     println!("");
    // }
    println!("{}", dist[gy - 1][gx - 1]);
}
