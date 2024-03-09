use std::iter::once;

use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize, t: usize,
        ab: [(Usize1, usize); t],
    };

    ab.into_iter()
        .scan(
            (vec![0; n], once((0, n)).collect::<FxHashMap<_, _>>()),
            |(scores, freq), (a, b)| {
                let old = scores[a];
                scores[a] += b;
                let new = scores[a];

                *freq.get_mut(&old).unwrap() -= 1;
                if freq[&old] == 0 {
                    freq.remove(&old);
                }

                *freq.entry(new).or_insert(0) += 1;

                Some(freq.len())
            },
        )
        .for_each(|ans| {
            println!("{ans}");
        });
}
