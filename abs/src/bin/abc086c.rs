use proconio::input;

fn main() {
    input! {
        n: i32,
        t: [[i32; 3]; n],
    }
    let mut ti1= vec![0, 0, 0];
    for ti in t {
        let kyori = (ti[1]-ti1[1]).abs() + (ti[2]-ti1[2]).abs();
        let kyori_t = kyori - (ti[0]-ti1[0]);
        if kyori_t == 0 {
            ti1 = ti.clone();
            continue;
        } else if kyori < ti[0] - ti1[0] && kyori_t % 2 == 0 {
            ti1 = ti.clone();
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
