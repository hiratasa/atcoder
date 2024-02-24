use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let c = if s[0] == s[1] {
        s[0]
    } else if s[0] == s[2] {
        s[0]
    } else {
        s[1]
    };

    let ans = s.iter().position(|&cc| cc != c).unwrap() + 1;

    println!("{ans}")
}
