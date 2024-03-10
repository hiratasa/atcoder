use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    };

    println!(
        "{}",
        s.chars()
            .scan(false, |x, c| {
                if c == '|' {
                    *x = !*x;
                    Some(None)
                } else if *x {
                    Some(None)
                } else {
                    Some(Some(c))
                }
            })
            .flatten()
            .join("")
    );
}
