use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    const M: usize = 100000000;

    let b = a.into_iter().map(|x| x % M).sorted().collect::<Vec<_>>();

    let s = (n - 1) * b.iter().sum::<usize>();
    let t = b
        .iter()
        .copied()
        .enumerate()
        .scan(n, |i, (j, x)| {
            while *i > 0 && b[*i - 1] + x >= M {
                *i -= 1;
            }

            Some(n - std::cmp::max(*i, j + 1))
        })
        .sum::<usize>();

    let ans = s - t * M;

    println!("{ans}");
}
