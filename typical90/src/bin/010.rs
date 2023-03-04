use proconio::input;
fn main() {
    input! {
        n: usize,
        cp: [[usize; 2]; n],
        q: usize,
        lr: [[usize; 2]; q],
    }
    let (mut note_1, mut note_2) = (vec![0; n+1], vec![0; n+1]);
    for i in 1..=n {
        if cp[i-1][0] == 1 {
            note_1[i] = note_1[i-1] + cp[i-1][1];
            note_2[i] = note_2[i-1];
        } else {
            note_1[i] = note_1[i-1];
            note_2[i] = note_2[i-1] + cp[i-1][1];
        }
    }
    // println!("{:?} {:?}", note_1, note_2);
    for lri in lr {
        println!("{} {}", note_1[lri[1]]-note_1[lri[0]-1], note_2[lri[1]]-note_2[lri[0]-1]);
    }

    // for lri in lr {
    //     let (mut ans_1, mut ans_2) = (0, 0);
    //     for i in lri[0]-1..lri[1] {
    //         if *cp.get(i).unwrap().get(0).unwrap() == 1 as usize {
    //             ans_1 += cp[i][1];
    //         } else {
    //             ans_2 += cp[i][1];
    //         }
    //     }
    //     println!("{} {}", ans_1, ans_2);
    // }
}
