use proconio::input;

fn main() {
    input! {
        ab: [i32; 2]
    }
    if ab[0]*ab[1] % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
