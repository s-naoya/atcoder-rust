use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut wa = vec![(0, 0); n + 1];
    for (i, &ai) in a.iter().enumerate() {
        wa[i + 1] = wa[i].clone();
        if ai == 0 {
            wa[i + 1].0 += 1;
        } else {
            wa[i + 1].1 += 1;
        }
    }
    input! {q: usize}
    for _ in 0..q {
        input! {(l, r): (usize, usize)}
        let sum = (wa[r].0 - wa[l - 1].0, wa[r].1 - wa[l - 1].1);
        println!(
            "{}",
            if sum.0 == sum.1 {
                "draw"
            } else if sum.0 > sum.1 {
                "lose"
            } else {
                "win"
            }
        );
    }
}
