fn main() {
    input! {
        t: usize,
    };

    // let mut rng = SmallRng::seed_from_u64(42);

    for _ in 0..t {
        input! {
            n: usize, k: usize,
            a: [usize; n],
        };

        // let n = rng.gen_range(3..=10);
        // let k = rng.gen_range(0..1 << n);
        // let a = (0..n).map(|_| rng.gen_range(0..1 << n)).collect::<Vec<_>>();

        // eprintln!("{n} {k}\n{}", a.iter().join(" "));

        if k == 0 {
            println!("Yes");
            println!("2");
            println!("1 1");
            continue;
        }

        let (basis, basis_bits, zero_bits) = make_basis(&a);

        let (bits, kk) = izip!(basis, basis_bits).fold((0, k), |(bits, kk), (b, b_bit)| {
            let c = (b + 1).next_power_of_two() >> 1;
            if kk & c != 0 {
                (bits ^ b_bit, kk ^ b)
            } else {
                (bits, kk)
            }
        });

        if kk != 0 {
            println!("No");
            continue;
        }

        let Some(mut target) = once(0).chain(zero_bits).map(|b| bits ^ b).find(|&b| {
            !(0..n).all(|i| (b & (1 << i) != 0) == (i % 2 == 0))
                && !(0..n).all(|i| (b & (1 << i) != 0) == (i % 2 == 1))
        }) else {
            println!("No");
            continue;
        };

        let mut a = a;
        let mut ops = vec![];
        let i0 = if let Some(i0) =
            (0..n - 2).find(|&i| target & (1 << i) != 0 && target & (1 << (i + 1)) != 0)
        {
            if target & (1 << (i0 + 2)) != 0 {
                operate(&mut a, i0 + 1, &mut ops);
                target ^= 1 << (i0 + 2);
            }
            operate(&mut a, i0, &mut ops);
            target ^= 1 << (i0 + 1);

            i0 + 1
        } else if let Some(i0) =
            (0..n - 1).find(|&i| target & (1 << i) == 0 && target & (1 << (i + 1)) == 0)
        {
            i0
        } else {
            assert!(target & (1 << (n - 3)) == 0);
            assert!(target & (1 << (n - 2)) != 0);
            assert!(target & (1 << (n - 1)) != 0);
            operate(&mut a, n - 2, &mut ops);
            target ^= 1 << (n - 2);

            n - 3
        };

        assert!(target & (1 << i0) == 0);
        assert!(target & (1 << (i0 + 1)) == 0);
        for i in i0..n {
            if target & (1 << i) == 0 {
                continue;
            }

            move_to_left2(&mut a, i, &mut ops);
            target ^= 1 << i;
            target ^= 1 << (i - 2);
        }

        assert!(target & (1 << (n - 2)) == 0);
        assert!(target & (1 << (n - 1)) == 0);

        for i in (1..n - 2).rev() {
            if target & (1 << i) == 0 {
                continue;
            }

            if target & (1 << (i - 1)) == 0 {
                move_to_left(&mut a, i, &mut ops);
                target ^= 1 << i;
                target ^= 1 << (i - 1);
            } else {
                operate(&mut a, i - 1, &mut ops);
                target ^= 1 << i;
            }
        }

        assert_eq!(target, 1);
        assert_eq!(a[0], k);

        println!("Yes");
        println!("{}", ops.len());
        println!("{}", ops.iter().map(|&x| x + 1).join(" "));
    }
}

fn make_basis(a: &[usize]) -> (Vec<usize>, Vec<u64>, Vec<u64>) {
    let mut a = a.to_vec();
    let mut bits = (0..a.len()).map(|i| 1 << i).collect::<Vec<_>>();
    let mut st = 0;
    for i in (0..60).rev() {
        if let Some(p) = a[st..].iter().position(|&x| x & (1 << i) != 0) {
            let p = st + p;
            a.swap(st, p);
            bits.swap(st, p);
            for j in st + 1..a.len() {
                if a[j] & (1 << i) != 0 {
                    a[j] ^= a[st];
                    bits[j] ^= bits[st];
                }
            }
            st += 1;
        }
    }

    (a[..st].to_vec(), bits[..st].to_vec(), bits[st..].to_vec())
}

// 二つ左に移動する
fn move_to_left2(a: &mut [usize], i: usize, ops: &mut Vec<usize>) {
    assert!(i > 1);

    operate(a, i - 2, ops);
    operate(a, i - 1, ops);
    operate(a, i - 2, ops);
}

// 右に二つ余剰があるときに、一つ左に移動する
fn move_to_left(a: &mut [usize], i: usize, ops: &mut Vec<usize>) {
    assert!(i > 0);
    assert!(i + 2 < a.len());

    operate(a, i + 1, ops);
    operate(a, i, ops);
    operate(a, i + 1, ops);
    operate(a, i - 1, ops);
    operate(a, i, ops);
    operate(a, i - 1, ops);
}

fn operate(a: &mut [usize], i: usize, ops: &mut Vec<usize>) {
    assert!(i + 1 < a.len());

    ops.push(i);
    a[i + 1] ^= a[i];
    a[i] = a[i + 1];
}

#[allow(unused_imports)]
use std::{
    cmp::{max, min, Ordering, Reverse},
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
use rand::{rngs::SmallRng, Rng, SeedableRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
