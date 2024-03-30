use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, a: usize, b: usize,
        d: [usize; n],
    };

    let d = d
        .into_iter()
        .map(|x| x % (a + b))
        .sorted()
        .collect::<Vec<_>>();

    let ans = (0..n).any(|i| {
        if i == 0 {
            d[n - 1] - d[0] < a
        } else {
            (d[i - 1] + a + b - d[i]) < a
        }
    });

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
