use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let x = s[3..].iter().collect::<String>().parse::<usize>().unwrap();

    if x > 0 && x < 350 && x != 316 {
        println!("Yes");
    } else {
        println!("No");
    }
}
