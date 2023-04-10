use proconio::input;

fn ans_1() {
    input! {
        (n, k): (usize, usize),
        mut a: [usize; n]
    }
    a.sort_unstable();
    let mut note = Vec::with_capacity(n);
    for i in 0..n {
        note.push(vec![0usize; i + 1]);
    }
    let mut next_note = note.clone();
    for i in 0..n {
        next_note[i][i] = 1;
    }

    let mut minnum = 0;
    for _ in 0..k {
        let mut numvec = vec![0; n];
        // println!("=== k = {} ===", kk);
        for i in 0..n {
            let mut num = 0;
            for j in 0..=i {
                // println!("{} = {} * {}", num, a[j], next_note[i][j]);
                num += a[j] * next_note[i][j];
            }
            numvec[i] = num;
            // println!("{} -> {}", i, num);
        }

        let mut numflag = vec![false; n];
        let mut nowminidx = 0;
        minnum = numvec[nowminidx];
        for i in 0..n {
            if numvec[nowminidx] == numvec[i] {
                numflag[i] = true;
            } else if numvec[nowminidx] > numvec[i] {
                numflag[i] = true;
                numflag[nowminidx] = false;
                nowminidx = i;
                minnum = numvec[i];
            }
        }
        for i in 0..n {
            if !numflag[i] {
                continue;
            }
            note[i] = next_note[i].clone();
            let num = numvec[i];

            let mut next_num = num;
            let mut next_num_j = 0usize;
            for k in 0..i {
                if next_num > (num + a[k]) {
                    next_num = num + a[k];
                    next_num_j = k;
                }
            }
            next_note[i][next_num_j] += 1;
        }
        // println!("{}", minnum);
    }
    println!("{}", minnum);
}
fn main() {
    ans_1();
}
