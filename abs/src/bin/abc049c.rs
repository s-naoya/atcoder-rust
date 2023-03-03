use proconio::input;

fn main() {
    input! {s: String,}
    let mut i = 0;
    let len = s.len();
    // println!("len: {}", len);
    loop {
        // println!("i: {}", i);
        if i == len {
            println!("YES");
            return;
        }
        if i + 4 < len && &s[i..i+5] == "dream" {
            if i + 6 < len && &s[i+5..i+7] == "er" {
                if i + 7 < len && &s[i+7..i+8] == "a" { // dream
                    // println!("dream");
                    i = i + 5;
                    continue;
                }
                // dreamer
                // println!("dreamer");
                i = i + 7;
                continue;
            }
            // dream
            // println!("dream");
            i = i + 5;
            continue;
        } else if i + 4 < len && &s[i..i+5] == "erase" {
            if i + 5 < len && &s[i+5..i+6] == "r" { // eraser
                // println!("eraser");
                i = i + 6;
                continue;
            }
            // erase
            // println!("erase");
            i = i + 5;
            continue;
        } else {
            println!("NO");
            return;
        }
    }
}
// 1:37:19.27