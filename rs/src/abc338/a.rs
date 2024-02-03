use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    if s[0].is_ascii_uppercase() && s[1..].iter().all(|&c| c.is_ascii_lowercase()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
