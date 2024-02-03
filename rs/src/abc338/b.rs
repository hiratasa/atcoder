use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let ans = (b'a'..=b'z')
        .rev()
        .max_by_key(|&c| s.iter().copied().filter(|&d| d == c as char).count())
        .unwrap() as char;

    println!("{ans}");
}
