use proconio::input;

fn main() {
    input! {
        nq: [usize; 2],
        events: [(usize, usize); nq[1]],
    }
    let mut status = vec![0; nq[0]];
    for event in events {
        if event.0 == 1 {
            status[event.1-1] += 1;
        } else if event.0 == 2 {
            status[event.1-1] += 2;
        } else if event.0 == 3 {
            if status[event.1-1] >= 2 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }

}
