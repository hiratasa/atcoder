use proconio::{input, marker::Chars};

fn main() {
    input! { s: Chars };

    let n = s.len();

    if s[0] == '<' && s[n - 1] == '>' && s[1..n - 1].iter().all(|&c| c == '=') {
        println!("Yes")
    } else {
        println!("No")
    }
}
