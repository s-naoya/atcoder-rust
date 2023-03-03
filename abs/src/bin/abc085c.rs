use proconio::input;

fn main() {
    input! {
        ny: [i32; 2],
    }

    for x in 0..=ny[0] {
        for y in 0..=ny[0]-x {
            let z = ny[0] - x - y;
            if 10000*x + 5000*y + 1000*z == ny[1] {
                println!("{} {} {}", x, y, z);
                return
            }
        }
    }
    println!("-1 -1 -1");
}
// 34:31.74