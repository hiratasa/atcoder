use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    };

    for row in a {
        println!(
            "{}",
            row.into_iter()
                .positions(|x| x > 0)
                .map(|i| i + 1)
                .join(" ")
        );
    }
}
