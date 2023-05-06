#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::iter::*;
#[allow(unused_imports)]
use std::mem::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use std::usize;

#[allow(unused_imports)]
use bitset_fixed::BitSet;
#[allow(unused_imports)]
use itertools::{chain, iproduct, iterate, izip, Itertools};
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use rustc_hash::FxHashMap;
#[allow(unused_imports)]
use rustc_hash::FxHashSet;

// vec with some initial value
#[allow(unused_macros)]
macro_rules! vvec {
    ($($x:expr),+; $y:expr; $n:expr) => {{
        let mut v = vec![$y; $n];

        let mut it = v.iter_mut();
        $(
            *it.next().unwrap() = $x;
        )+

        v
    }}
}

#[allow(unused_macros)]
macro_rules! it {
    ($x:expr) => {
        once($x)
    };
    ($first:expr,$($x:expr),+) => {
        chain(
            once($first),
            it!($($x),+)
        )
    }
}

#[allow(unused_macros)]
macro_rules! bitset {
    ($n:expr, $x:expr) => {{
        let mut bs = BitSet::new($n);
        bs.buffer_mut()[0] = $x as u64;
        bs
    }};
}

#[allow(unused_macros)]
macro_rules! pushed {
    ($c:expr, $x:expr) => {{
        let mut c = $c;
        c.push($x);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! inserted {
    ($c:expr, $($x:expr),*) => {{
        let mut c = $c;
        c.insert($($x),*);
        c
    }};
}

#[allow(unused_macros)]
macro_rules! read_tuple {
    ($($t:ty),+) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut it = line.trim()
            .split_whitespace();

        ($(
            it.next().unwrap().parse::<$t>().ok().unwrap()
        ),+)
    }}
}

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_str() -> Vec<char> {
    read::<String>().chars().collect()
}

#[allow(dead_code)]
fn read_row<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim()
        .split_whitespace()
        .map(|s| s.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_col<T: FromStr>(n: usize) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}

#[allow(dead_code)]
fn read_mat<T: FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_row()).collect()
}

#[allow(dead_code)]
fn read_vec<R, F: FnMut() -> R>(n: usize, mut f: F) -> Vec<R> {
    (0..n).map(|_| f()).collect()
}

trait IterCopyExt<'a, T>: IntoIterator<Item = &'a T> + Sized
where
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

impl<'a, T, I> IterCopyExt<'a, T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
}

fn z_algorithm<T: std::cmp::Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    let mut z = vec![0; n];
    z[0] = n;

    let mut l = 0;
    let mut r = 0;
    for i in 1..n {
        // assert!(s[l..r] == s[0..r - l]);
        if i < r && z[i - l] < r - i {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i..r] (=s[i-l..r-l]) = s[0..r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = std::cmp::max(i, r);
            while r < n && s[r] == s[r - l] {
                r += 1;
            }
            z[i] = r - l;
        }
    }

    z
}

fn conv(c: char) -> u8 {
    match c {
        'O' | 'D' => 0,
        'I' => 1,
        'Z' => 2,
        'E' => 3,
        'h' => 4,
        's' => 5,
        'q' => 6,
        'L' => 7,
        'B' => 8,
        'G' => 9,
        _ => unreachable!(),
    }
}

// u[..l0] + (b0?s:"") と u[..l1] + (b1:s:"") を比較して大きいほうを残す
// zは s+u に対するz_algorithmの結果
fn merge(
    (l0, b0, lens0): (usize, bool, &BitSet),
    (l1, b1, lens1): (usize, bool, &BitSet),
    s: &[u8],
    u: &[u8],
    z: &[usize],
    masks: &[BitSet],
) -> (usize, bool, BitSet) {
    assert!(l0 <= u.len(), "{} {} {:?}", l0, b0, lens0);
    assert!(l1 <= u.len(), "{} {} {:?}", l1, b1, lens1);

    // どっちかはbがtrue
    assert!(b0 || b1);

    let is_expected_order = {
        if b0 {
            if b1 {
                l0 < l1
            } else {
                false
            }
        } else {
            if b1 {
                true
            } else {
                unreachable!()
            }
        }
    };

    let (l0, b0, lens0, l1, b1, lens1) = if is_expected_order {
        (l0, b0, lens0, l1, b1, lens1)
    } else {
        (l1, b1, lens1, l0, b0, lens0)
    };

    let sl = s.len();

    enum DiffPos {
        Less(usize),
        Greater(usize),
        Contained,
        Contains,
    }
    use DiffPos::*;

    let r = if b0 {
        assert!(b1);
        assert!(l0 <= l1);
        // u[..l0] + s と u[..l1] + s の比較 (l0<=l1)
        // => s と u[l0..l1] + s の比較
        let idx = z[sl + l0];
        if idx < sl {
            if idx < l1 - l0 {
                if s[idx] < u[l0 + idx] {
                    Less(l0 + idx)
                } else {
                    Greater(l0 + idx)
                }
            } else {
                // s[l1-l0..] と s の比較
                let idx2 = z[l1 - l0];
                if idx2 < sl - (l1 - l0) {
                    if s[l1 - l0 + idx2] < s[idx2] {
                        Less(l1 + idx2)
                    } else {
                        Greater(l1 + idx2)
                    }
                } else {
                    Contained
                }
            }
        } else {
            Contained
        }
    } else {
        assert!(b1);
        // u[..l0] と u[..l1] + s の比較
        if l0 <= l1 {
            Contained
        } else {
            // => u[l1..l0] と s の比較
            let idx = z[sl + l1];
            if idx < l0 - l1 {
                if idx < sl {
                    if u[l1 + idx] < s[idx] {
                        Less(l1 + idx)
                    } else {
                        Greater(l1 + idx)
                    }
                } else {
                    Contains
                }
            } else if l0 - l1 <= sl {
                Contained
            } else {
                Contains
            }
        }
    };

    match r {
        Less(idx) => (l1, b1, (lens0 & &masks[idx + 1]) | lens1),
        Greater(idx) => (l0, b0, (lens1 & &masks[idx + 1]) | lens0),
        Contained => (l1, b1, lens0 | lens1),
        Contains => (l0, b0, lens0 | lens1),
    }
}

// 辞書順最大を求める
// check(i, l): i番目未満の文字列まででl文字はokか
fn calc<F: Fn(usize, usize) -> bool>(t: &[Vec<u8>], d: usize, check: F) -> Vec<u8> {
    let masks = once(bitset!(d + 1, 0))
        .chain((0..=d).scan(bitset!(d + 1, 0), |bs, _| {
            *bs <<= 1;
            bs.set(0, true);
            Some(bs.clone())
        }))
        .collect::<Vec<_>>();
    let (ret, _) = t.iter().enumerate().fold(
        (vec![], bitset!(d + 1, 1)),
        |(u, lens): (Vec<u8>, BitSet), (i, s)| {
            assert!(lens[u.len()]);
            assert!(check(i, u.len()));

            let z = z_algorithm(&chain(s.citer(), u.citer()).collect::<Vec<_>>());

            let (l2, lens2) = {
                let mut lens2 = lens.clone();
                let mut tmp = 0;
                for j in 0..=d {
                    if lens2[j] {
                        if !check(i + 1, j) {
                            lens2.set(j, false);
                        } else {
                            tmp = j;
                        }
                    }
                }
                (tmp, lens2)
            };

            let (l0, b0, lens0) = (0..=d)
                .filter(|&l| lens[l] && l + s.len() <= d && check(i + 1, l + s.len()))
                .map(|l| {
                    let mut lens1 = bitset!(d + 1, 0);
                    lens1.set(l + s.len(), true);
                    (l, true, lens1)
                })
                .fold((l2, false, lens2), |(l0, b0, lens0), (l1, b1, lens1)| {
                    merge((l0, b0, &lens0), (l1, b1, &lens1), &s, &u, &z, &masks)
                });

            assert!(l0 <= u.len());

            if b0 {
                assert!(lens0[l0 + s.len()]);
                (chain(u[..l0].citer(), s.citer()).collect(), lens0)
            } else {
                assert!(lens0[l0]);
                let mut u = u;
                u.resize_with(l0, || unreachable!());
                (u, lens0)
            }
        },
    );

    ret
}

fn main() {
    let d = read::<usize>();
    let n = read::<usize>();
    let w = read_vec(n, || read_str());

    let t = w
        .iter()
        .map(|s| s.citer().rev().map(|c| conv(c)).collect::<Vec<_>>())
        .sorted_by(|s0, s1| {
            izip!(s0.citer().chain(s1.citer()), s1.citer().chain(s0.citer()))
                .skip_while(|(d0, d1)| d0 == d1)
                .next()
                .map_or(Ordering::Equal, |(d0, d1)| d0.cmp(&d1).reverse())
        })
        .collect::<Vec<_>>();

    let all_zero = t.iter().all(|s| s[0] == 0);

    if all_zero {
        let mut ans = calc(&t, d, |_i, _l| true);

        assert!(ans[0] == 0);
        while ans.len() > 1 && *ans.last().unwrap() == 0 {
            ans.pop();
        }
        if ans.len() == 1 {
            println!("0");
        } else {
            println!("0.{}", ans.citer().skip(1).join(""));
        }
    } else {
        let dp_rev = once(bitset!(d + 1, 1))
            .chain(
                t.iter()
                    .rev()
                    .map(|s| s.len())
                    .scan(bitset!(d + 1, 1), |bs, l| {
                        bs.shl_or(l);
                        Some(bs.clone())
                    }),
            )
            .collect::<Vec<_>>();

        let ansl = (0..n)
            .filter(|&i| t[i][0] != 0)
            .map(|i| {
                t[i].len()
                    + (0..=d - t[i].len())
                        .rev()
                        .find(|&l| dp_rev[n - 1 - i][l])
                        .unwrap()
            })
            .max()
            .unwrap();

        let ans = calc(&t, ansl, |i, l| dp_rev[n - i][ansl - l]);

        assert!(ans[0] != 0);
        assert!(ans.len() == ansl);

        println!("{}", ans.citer().join(""));
    }
}
