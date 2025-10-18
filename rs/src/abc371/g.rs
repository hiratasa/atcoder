fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        a: [usize; n],
    };

    let cycles = (0..n)
        .scan(vec![false; n], |used, i| {
            if used[i] {
                Some(None)
            } else {
                Some(Some(
                    iterate(i, |&j| p[j])
                        .skip(1)
                        .take_while(|&j| j != i)
                        .chain(once(i))
                        .inspect(|&i| used[i] = true)
                        .collect::<Vec<_>>(),
                ))
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let cidxs = cycles
        .iter()
        .enumerate()
        .fold(vec![(0, 0); n], |mut cidxs, (idx, cycle)| {
            cycle.iter().enumerate().for_each(|(j, &i)| {
                cidxs[i] = (idx, j);
            });
            cidxs
        });

    let ans = (0..n)
        .scan(
            (vec![None; cidxs.len()], FxHashMap::default()),
            |(assigns, conds): &mut (Vec<Option<usize>>, FxHashMap<usize, (usize, usize)>), i| {
                if let Some(l) = assigns[cidxs[i].0] {
                    Some(a[cycles[cidxs[i].0][(cidxs[i].1 + l) % cycles[cidxs[i].0].len()]])
                } else {
                    let cidx = cidxs[i].0;
                    let idx_in_cycle = cidxs[i].1;
                    let cycle = &cycles[cidx];
                    let k = cycle.len();
                    let factors = (2..)
                        .scan(k, |k, i| {
                            if *k == 1 {
                                None
                            } else if i * i > *k {
                                Some((replace(k, 1), 1))
                            } else {
                                let q;
                                (q, *k) = iterate(*k, |&z| z / i)
                                    .enumerate()
                                    .find(|&(_, z)| z % i > 0)
                                    .unwrap();

                                Some((i, q))
                            }
                        })
                        .filter(|&(_, q)| q > 0)
                        .collect::<Vec<_>>();

                    let rm = factors
                        .iter()
                        .filter_map(|&(p, q)| {
                            let (qq, r) = *conds.get(&p)?;

                            if qq <= q {
                                Some((r, p.pow(qq as u32)))
                            } else {
                                let pp = p.pow(q as u32);
                                Some((r % pp, pp))
                            }
                        })
                        .collect::<Vec<_>>();
                    let (c, l) = crt(&rm);

                    let i = (0..)
                        .map(|i| i * l + c)
                        .take_while(|&i| i < k)
                        .min_by_key(|&i| a[cycle[(idx_in_cycle + i) % k]])
                        .unwrap();
                    assigns[cidx] = Some(i);

                    factors.iter().for_each(|&(p, q)| {
                        if let Some(&(qq, _)) = conds.get(&p) {
                            if qq <= q {
                                conds.insert(p, (q, i % p.pow(q as u32)));
                            }
                        } else {
                            conds.insert(p, (q, i % p.pow(q as u32)));
                        }
                    });

                    Some(a[cycle[(idx_in_cycle + i) % k]])
                }
            },
        )
        .collect::<Vec<_>>();

    println!("{}", ans.iter().join(" "));
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

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (_zero, g, _u, v) = std::iter::successors(Some((a, b, 1, 0)), |&(a, b, u, v)| {
        if a == 0 {
            None
        } else {
            Some((b % a, a, -u * (b / a) + v, u))
        }
    })
    .last()
    .unwrap();

    (v, (g - a * v) / b, g)
}

// 中国剰余定理
fn crt(rm: &[(usize, usize)]) -> (usize, usize) {
    rm.iter()
        .copied()
        .fold((0usize, 1usize), |(r0, m0), (r1, m1)| {
            let (x, y, g) = extgcd(m0 as i64, m1 as i64);

            let c = r1 as i64 - r0 as i64;
            assert!(c % g == 0);

            let l = m0 / g as usize * m1;

            let xx = ((c / g) * x).rem_euclid(m1 as i64 / g) as usize;

            (xx * m0 + r0, l)
        })
}
