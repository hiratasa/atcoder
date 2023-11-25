use std::{collections::BTreeMap, iter::once};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        queries: [(Usize1, usize); q],
    };

    let freq = a.iter().copied().fold(BTreeMap::default(), |mut map, x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });
    let mi = a.iter().copied().min().unwrap();
    let ma = a.iter().copied().max().unwrap();
    let nonexist = once((0, mi))
        .chain(
            a.iter()
                .copied()
                .sorted()
                .dedup()
                .tuple_windows()
                .map(|(x, y)| (x + 1, y)),
        )
        .filter(|&(x, y)| x < y)
        .chain(once((ma + 1, usize::MAX)))
        .collect::<BTreeMap<_, _>>();

    let insert = |set: &mut BTreeMap<usize, usize>, x: usize| {
        let l = if let Some((&l, &r)) = set.range(..=x).next_back() {
            if r < x {
                set.insert(x, x + 1);
                x
            } else if r == x {
                set.remove(&l);
                set.insert(l, x + 1);
                l
            } else {
                return;
            }
        } else {
            set.insert(x, x + 1);
            x
        };

        if let Some(&r) = set.get(&(x + 1)) {
            set.remove(&(x + 1));
            set.insert(l, r);
        } else {
            set.insert(l, x + 1);
        }
    };

    let remove = |set: &mut BTreeMap<usize, usize>, x: usize| {
        let (&l, &r) = set.range(..=x).next_back().unwrap();
        assert!(x < r);

        set.remove(&l);

        if l < x {
            set.insert(l, x);
        }

        if x + 1 < r {
            set.insert(x + 1, r);
        }
    };

    queries
        .iter()
        .scan((a, freq, nonexist), |(a, freq, nonexist), &(i, x)| {
            let x0 = a[i];

            *freq.get_mut(&x0).unwrap() -= 1;
            if freq[&x0] == 0 {
                insert(nonexist, x0);
            }

            a[i] = x;
            if freq.get(&x).copied().unwrap_or(0) == 0 {
                remove(nonexist, x);
            }
            *freq.entry(x).or_insert(0) += 1;

            Some(*nonexist.keys().next().unwrap())
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}
