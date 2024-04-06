use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    println!(
        "{}",
        (1..=n).map(|i| if i % 3 == 0 { 'x' } else { 'o' }).join("")
    );
}
