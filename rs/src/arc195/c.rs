fn main() {
    input! {
        t: usize,
        cases: [(usize, usize); t],
    };

    for (r, b) in cases {
        // eprintln!("# {r} {b}");
        if r % 2 != 0 {
            println!("No");
            continue;
        }

        if r == 0 && b % 2 != 0 {
            println!("No");
            continue;
        }

        println!("Yes");

        let c = r / 2;

        let mut seen = FxHashSet::default();
        if (b + c) % 2 == 0 {
            let z = (b + c) / 2;

            (0..z)
                .map(|i| (i + 1, i + 2))
                .chain((0..z).map(|i| (z - i + 1, z - i)))
                .cycle()
                .tuple_windows()
                .take(b + c)
                .enumerate()
                .for_each(|(i, ((x0, y0), (x1, y1)))| {
                    if i < b {
                        assert!(!seen.contains(&(x0, y0)));
                        seen.insert((x0, y0));
                        println!("B {x0} {y0}");
                    } else {
                        assert!(!seen.contains(&(x0, y0)));
                        seen.insert((x0, y0));
                        println!("R {x0} {y0}");
                        if y0 < y1 {
                            let x = min(x0, x1);
                            let y = if x == x0 { y1 } else { y0 };
                            assert!(!seen.contains(&(x, y)));
                            seen.insert((x, y));
                            println!("R {x} {y}");
                        } else {
                            let x = max(x0, x1);
                            let y = if x == x0 { y1 } else { y0 };
                            assert!(!seen.contains(&(x, y)));
                            seen.insert((x, y));
                            println!("R {x} {y}");
                        }
                    }
                });
        } else if b + c == 1 {
            assert!(b == 0);
            println!("R 1 1");
            println!("R 1 2");
        } else {
            let z = (b + c) / 2;

            (0..=z)
                .map(|i| (i + 1, i + 1))
                .chain((0..z).map(|i| (z - i + 2, z - i)))
                .tuple_windows()
                .take(b + c)
                .enumerate()
                .for_each(|(i, ((x0, y0), (x1, y1)))| {
                    if i < b {
                        assert!(!seen.contains(&(x0, y0)));
                        seen.insert((x0, y0));
                        println!("B {x0} {y0}");
                    } else {
                        assert!(!seen.contains(&(x0, y0)));
                        seen.insert((x0, y0));
                        println!("R {x0} {y0}");
                        if y0 < y1 {
                            let x = min(x0, x1);
                            let y = if x == x0 { y1 } else { y0 };
                            assert!(!seen.contains(&(x, y)));
                            seen.insert((x, y));
                            println!("R {x} {y}");
                        } else {
                            let x = max(x0, x1);
                            let y = if x == x0 { y1 } else { y0 };
                            assert!(!seen.contains(&(x, y)));
                            seen.insert((x, y));
                            println!("R {x} {y}");
                        }
                    }
                });
            assert!(!seen.contains(&(3, 1)));
            seen.insert((3, 1));
            println!("R 3 1");
            assert!(!seen.contains(&(2, 1)));
            seen.insert((2, 1));
            println!("R 2 1");
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
use itertools::*;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use proconio::{
    input, input_interactive,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
