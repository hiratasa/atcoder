fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            mut a: [usize; n],
            mut b: [usize; n],
        };

        let na = a.iter().filter(|&&x| x == 1).count();
        let nb = b.iter().filter(|&&x| x == 1).count();

        if na != nb {
            println!("No");
            continue;
        }

        if na > n / 2 {
            a.iter_mut().for_each(|x| *x = 1 - *x);
            b.iter_mut().for_each(|x| *x = 1 - *x);
        }

        let pos_a = a.iter().positions(|&x| x == 1).collect::<Vec<_>>();
        let pos_b = b.iter().positions(|&x| x == 1).collect::<Vec<_>>();

        let sa = pos_a.iter().sum::<usize>();
        let sb = pos_b.iter().sum::<usize>();
        if sa != sb {
            println!("No");
            continue;
        }

        let mut lesses = izip!(pos_a.iter().copied(), pos_b.iter().copied())
            .filter(|&(i, j)| i < j)
            .collect::<Vec<_>>();
        let mut greaters = izip!(pos_a.iter().copied(), pos_b.iter().copied())
            .filter(|&(i, j)| i > j)
            .rev()
            .collect::<Vec<_>>();

        let mut ops = vec![];
        while let Some((i, j)) = lesses.pop() {
            let (k, l) = greaters.pop().unwrap();

            let d0 = j - i;
            let d1 = k - l;

            match d0.cmp(&d1) {
                Ordering::Less => {
                    ops.push((i, j, k - d0, k));
                    greaters.push((k - d0, l));
                }
                Ordering::Equal => {
                    ops.push((i, j, l, k));
                }
                Ordering::Greater => {
                    ops.push((i, i + d1, l, k));
                    lesses.push((i + d1, j));
                }
            }
        }

        println!("Yes");
        println!("{}", ops.len());
        for (i, j, k, l) in ops {
            let (i, j, k, l) = if i < k { (i, j, k, l) } else { (k, l, i, j) };

            println!("{} {} {} {}", i + 1, j + 1, k + 1, l + 1);
        }
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_with, successors},
    mem::{replace, swap, take},
};

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{Itertools, chain, iproduct, iterate, izip};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
