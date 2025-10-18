use itertools::Itertools;
use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    };

    let ans = ac
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, t)| t)
        .rev()
        .scan(usize::MAX, |min, (i, (a, c))| {
            *min = cmp::min(*min, c);

            if *min < c { Some(None) } else { Some(Some(i)) }
        })
        .flatten()
        .sorted()
        .collect::<Vec<_>>();

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|i| i + 1).join(" "));
}
