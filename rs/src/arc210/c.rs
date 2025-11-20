fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize, m: usize,
            mut a: [usize; n],
        };
        let n = n + 20;
        a.resize(n, 0);
        let mut b = vec![0; n];
        let mut d = vec![0; n];
        let mut c = 0;
        for i in 0..n {
            b[i] = a[i] + c;
            d[i] = c;
            c = b[i] / 10;
            b[i] %= 10;
        }
        assert_eq!(c, 0);

        let mut is_leading_zeros = true;
        let mut l = 0;
        for i in (0..n).rev() {
            l = l * 10 + b[i];
            assert!(l / m < 10);
            if l / m > 0 {
                is_leading_zeros = false;
            }
            if !is_leading_zeros {
                print!("{}", l / m);
            }
            let k = l / m * m;

            let e = l.saturating_sub(d[i]);
            if k <= e {
                l = min(d[i], l);
            } else {
                l -= k;
            }
        }
        if is_leading_zeros {
            print!("0");
        }

        println!();
    }
}

#[allow(unused_imports)]
use std::{
    cmp::{Ordering, Reverse, max, min},
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
    iter::{once, once_with, repeat, repeat_n, repeat_with, successors},
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
