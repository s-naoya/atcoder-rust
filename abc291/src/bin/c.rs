use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let mut road: HashMap<(i32, i32), i32> = HashMap::new();
    road.insert((0, 0), 0);
    let mut before = (0, 0);
    for si in s {
        let next = match si {
            'R' => (before.0 + 1, before.1),
            'L' => (before.0 - 1, before.1),
            'U' => (before.0, before.1 + 1),
            'D' => (before.0, before.1 - 1),
            _   => panic!("error"),
        };
        if road.contains_key(&next) {
            println!("Yes");
            return;
        } else {
            road.insert(next, 0);
        }
        before = next;
    }
    println!("No");
}
