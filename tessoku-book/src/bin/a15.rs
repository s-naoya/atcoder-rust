use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut t = a.clone();
    t.sort();
    t.dedup();
    let mut b = a.clone();
    for bi in &mut b {
        let bim: usize = *bi;
        *bi = binary_search(&t, bim) + 1;
    }
    println!("{}", b.iter().join(" "));
}

fn binary_search(nums: &Vec<usize>, val: usize) -> usize {
    let mut first = 0;
    let mut last = nums.len() - 1;
    loop {
        let mid = (first + last) / 2;
        if nums[mid] == val {
            return mid;
        } else if nums[mid] < val {
            first = mid + 1;
        } else if nums[mid] > val {
            last = mid - 1;
        }
    }
}
