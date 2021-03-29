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
use itertools::{chain, iproduct, iterate, izip, unfold, Itertools};
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

#[allow(dead_code)]
fn lower_bound<T, F>(mut begin: T, mut end: T, epsilon: T, mut f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: FnMut(T) -> std::cmp::Ordering,
{
    let two = T::try_from(2).ok().unwrap();
    while end - begin >= epsilon {
        let mid = begin + (end - begin) / two;
        match f(mid) {
            std::cmp::Ordering::Less => {
                begin = mid + epsilon;
            }
            _ => {
                end = mid;
            }
        }
    }
    begin
}
#[allow(dead_code)]
fn lower_bound_int<T, F>(begin: T, end: T, f: F) -> T
where
    T: std::marker::Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + std::cmp::PartialOrd<T>
        + std::convert::TryFrom<i32>,
    F: FnMut(T) -> std::cmp::Ordering,
{
    lower_bound(begin, end, T::try_from(1).ok().unwrap(), f)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

const B: i32 = 10000;

#[allow(dead_code)]
impl Rectangle {
    fn new(left: i32, top: i32, right: i32, bottom: i32) -> Rectangle {
        Rectangle {
            left,
            top,
            right,
            bottom,
        }
    }

    fn area(&self) -> i32 {
        (self.right - self.left) * (self.bottom - self.top)
    }

    fn width(&self) -> i32 {
        self.right - self.left
    }

    fn height(&self) -> i32 {
        self.bottom - self.top
    }

    fn valid(&self) -> bool {
        0 <= self.left
            && self.left < self.right
            && self.right <= B
            && 0 <= self.top
            && self.top < self.bottom
            && self.bottom <= B
    }

    fn include(&self, x: i32, y: i32) -> bool {
        // (x + 0.5, y + 0.5) を含むか
        self.left <= x && x < self.right && self.top <= y && y < self.bottom
    }

    fn overlap_with(&self, rect: &Rectangle) -> bool {
        (rect.right > self.left && self.right > rect.left)
            && (rect.bottom > self.top && self.bottom > rect.top)
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.left, self.top, self.right, self.bottom
        )
    }
}

fn find_overlap(
    ans: &[(usize, Rectangle)],
    i: usize,
    idx0: usize,
    idx1: usize,
    rect: &Rectangle,
) -> Option<usize> {
    ans[idx0..idx1]
        .iter()
        .enumerate()
        .map(|(idx, t)| (idx0 + idx, t))
        .find(|(_idx, (j, rect0))| *j != i && rect0.overlap_with(rect))
        .map(|(idx, _)| idx)
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
}

impl Dir {
    fn from_usize(idx: usize) -> Dir {
        match idx {
            0 => Dir::Left,
            1 => Dir::Top,
            2 => Dir::Right,
            3 => Dir::Bottom,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Shape {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[allow(dead_code)]
impl Shape {
    fn get(&self, dir: Dir) -> i32 {
        match dir {
            Dir::Left => self.left,
            Dir::Top => self.top,
            Dir::Right => self.right,
            Dir::Bottom => self.bottom,
        }
    }

    fn set(&mut self, dir: Dir, val: i32) -> i32 {
        match dir {
            Dir::Left => replace(&mut self.left, val),
            Dir::Top => replace(&mut self.top, val),
            Dir::Right => replace(&mut self.right, val),
            Dir::Bottom => replace(&mut self.bottom, val),
        }
    }

    fn is_zero(&self) -> bool {
        self.left == 0 && self.top == 0 && self.right == 0 && self.bottom == 0
    }
}

#[derive(Debug, Clone)]
struct EachParameter {
    i: usize,
    x: i32,
    y: i32,
    r: i32,
    shape: Shape,
    extend_order: [Dir; 4],
}

#[derive(Debug, Clone)]
struct Parameter {
    params: Vec<EachParameter>,
}

fn extend_rect(param: &EachParameter, rect0: &Rectangle, ans: &[(usize, Rectangle)]) -> Rectangle {
    let i = param.i;
    let r = param.r;
    let n = ans.len();

    let extend_left = |rect1: Rectangle| {
        let mut j0 = 0;
        let left = lower_bound_int(0, rect1.left, |left| {
            let rect = Rectangle { left, ..rect1 };
            if rect.area() > r {
                Ordering::Less
            } else if let Some(j) = find_overlap(&ans, i, j0, n, &rect) {
                // rectangles between [j0, j) have no overlap after now.
                j0 = j;
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        Rectangle { left, ..rect1 }
    };

    let extend_right = |rect1: Rectangle| {
        let mut j0 = 0;
        let right = lower_bound_int(rect1.right + 1, B + 1, |right| {
            let rect = Rectangle { right, ..rect1 };
            if rect.area() > r {
                Ordering::Greater
            } else if let Some(j) = find_overlap(&ans, i, j0, n, &rect) {
                // rectangles between [j0, j) have no overlap after now.
                j0 = j;
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }) - 1;
        Rectangle { right, ..rect1 }
    };

    let extend_top = |rect1: Rectangle| {
        let mut j0 = 0;
        let top = lower_bound_int(0, rect1.top, |top| {
            let rect = Rectangle { top, ..rect1 };
            if rect.area() > r {
                Ordering::Less
            } else if let Some(j) = find_overlap(&ans, i, j0, n, &rect) {
                // rectangles between [j0, j) have no overlap after now.
                j0 = j;
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        Rectangle { top, ..rect1 }
    };

    let extend_bottom = |rect1: Rectangle| {
        let mut j0 = 0;
        let bottom = lower_bound_int(rect1.bottom + 1, B + 1, |bottom| {
            let rect = Rectangle { bottom, ..rect1 };
            if rect.area() > r {
                Ordering::Greater
            } else if let Some(j) = find_overlap(&ans, i, j0, n, &rect) {
                // rectangles between [j0, j) have no overlap after now.
                j0 = j;
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }) - 1;
        Rectangle { bottom, ..rect1 }
    };

    // [Dir::Left, Dir::Top, Dir::Right, Dir::Bottom]
    //     .citer()
    //     .sorted_by_key(|&dir| Reverse(shape.get(dir)))
    //     .fold(rect0.clone(), |rect, dir| match dir {
    //         Dir::Left => extend_left(rect),
    //         Dir::Top => extend_top(rect),
    //         Dir::Right => extend_right(rect),
    //         Dir::Bottom => extend_bottom(rect),
    //     })
    param
        .extend_order
        .citer()
        .fold(rect0.clone(), |rect, dir| match dir {
            Dir::Left => extend_left(rect),
            Dir::Top => extend_top(rect),
            Dir::Right => extend_right(rect),
            Dir::Bottom => extend_bottom(rect),
        })
}

fn generate_rect(param: &EachParameter, ans: &[(usize, Rectangle)]) -> Rectangle {
    let i = param.i;
    let x = param.x;
    let y = param.y;
    let r = param.r;
    let shape = &param.shape;

    let n = ans.len();
    let mut j0 = 0;
    let z = lower_bound_int(1, B + 1, |z| {
        let rect = Rectangle::new(
            x - z * shape.left,
            y - z * shape.top,
            x + 1 + z * shape.right,
            y + 1 + z * shape.bottom,
        );
        if !rect.valid() || rect.area() > r {
            Ordering::Greater
        } else if let Some(j) = find_overlap(&ans, i, j0, n, &rect) {
            // rectangles between [j0, j) have no overlap after now.
            j0 = j;
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }) - 1;

    assert!(z >= 0);

    let rect0 = Rectangle::new(
        x - z * shape.left,
        y - z * shape.top,
        x + 1 + z * shape.right,
        y + 1 + z * shape.bottom,
    );

    extend_rect(param, &rect0, ans)
}

fn generate_rects(_n: usize, param: &Parameter) -> Vec<(usize, Rectangle)> {
    let ans0 = param
        .params
        .iter()
        .map(|p| (p.i, Rectangle::new(p.x, p.y, p.x + 1, p.y + 1)))
        .collect();

    param
        .params
        .iter()
        .enumerate()
        .fold(ans0, |mut ans, (idx, p)| {
            assert!(ans[idx].0 == p.i);
            ans[idx].1 = generate_rect(p, &ans);

            ans
        })
}

fn resume_generate_rects(
    _n: usize,
    param: &Parameter,
    idx: usize,
    // prev_ans should be reordered by current param
    prev_ans: &Vec<(usize, Rectangle)>,
) -> Option<Vec<(usize, Rectangle)>> {
    let ans0 =
        param.params[idx..]
            .iter()
            .enumerate()
            .fold(prev_ans.clone(), |mut ans, (idx1, p)| {
                assert!(ans[idx + idx1].0 == p.i);
                ans[idx + idx1].1 = Rectangle::new(p.x, p.y, p.x + 1, p.y + 1);
                ans
            });

    param.params[idx..]
        .iter()
        .enumerate()
        .try_fold(ans0, |mut ans, (idx1, p)| {
            let idx1 = idx + idx1;
            if idx1 == idx {
                ans[idx1].1 = generate_rect(p, &ans);
                if ans[idx1].1 == prev_ans[idx1].1 {
                    return None;
                }
            } else if find_overlap(&ans, p.i, idx, idx1, &prev_ans[idx1].1).is_none() {
                // This is incorrect.
                ans[idx1].1 = extend_rect(p, &prev_ans[idx1].1, &ans);
            } else {
                ans[idx1].1 = generate_rect(p, &ans);
            }

            Some(ans)
        })
}

fn resume_generate_rects_for_swap_order(
    _n: usize,
    param: &Parameter,
    idx0: usize,
    idx1: usize,
    // prev_ans should be reordered by current param
    prev_ans: &Vec<(usize, Rectangle)>,
) -> Option<Vec<(usize, Rectangle)>> {
    assert!(idx0 <= idx1);

    let ans0 =
        param.params[idx0..]
            .iter()
            .enumerate()
            .fold(prev_ans.clone(), |mut ans, (ii, p)| {
                assert!(ans[idx0 + ii].0 == p.i);
                ans[idx0 + ii].1 = Rectangle::new(p.x, p.y, p.x + 1, p.y + 1);
                ans
            });

    param.params[idx0..]
        .iter()
        .enumerate()
        .try_fold((ans0, idx0), |(mut ans, first_changed_idx), (ii, p)| {
            let ii = idx0 + ii;
            // ??
            /* if ii == idx0 {
                ans[ii].1 = generate_rect(p, &ans);
            } else */
            if find_overlap(&ans, p.i, first_changed_idx, ii, &prev_ans[ii].1).is_none() {
                // This is incorrect.
                ans[ii].1 = extend_rect(p, &prev_ans[ii].1, &ans);
            } else {
                ans[ii].1 = generate_rect(p, &ans);
            }

            let same = ans[ii].1 == prev_ans[ii].1;

            if first_changed_idx == ii && same {
                if ii == idx1 {
                    None
                } else {
                    Some((ans, ii + 1))
                }
            } else {
                Some((ans, min(first_changed_idx, ii)))
            }
        })
        .map(|t| t.0)
}

fn check_validity(_xyr: &[(i32, i32, i32)], ans: &[(usize, Rectangle)]) -> bool {
    ans.citer().all(|(_, rect)| rect.valid())
        && ans.citer().all(|(i, rect)| {
            ans[i + 1..]
                .citer()
                .all(|(_, rect1)| !rect.overlap_with(&rect1))
        })
}

fn calc_score(xyr: &[(i32, i32, i32)], ans: &[(usize, Rectangle)]) -> i32 {
    let n = xyr.len();
    ans.citer()
        .map(|(i, rect)| {
            let (x, y, r) = xyr[i];
            if rect.include(x, y) {
                let mi = min(r, rect.area()) as f64;
                let ma = max(r, rect.area()) as f64;
                // eprintln!("{}/{} = {}", mi, ma, mi / ma);
                1.0 - (1.0 - mi / ma).powi(2)
            } else {
                eprintln!("rect does not include the point");
                0.0
            }
        })
        .map(|a| a * 1e9 / n as f64)
        .sum::<f64>()
        .round() as i32
}

// #[allow(dead_code)]
// fn print_line(xyr: &[(i32, i32, i32)], param0: &Parameter, param1: &Parameter) {
//     const M: i32 = 100;

//     let n = param0.params.len();

//     (-M..=2 * M)
//         .map(|i| {
//             (
//                 i,
//                 Parameter {
//                     params: (0..n)
//                         .map(|j| {
//                             assert!(param0.params[j].i == param1.params[j].i);

//                             EachParameter {
//                                 i: param0.params[j].i,
//                                 x: param0.params[j].x,
//                                 y: param0.params[j].y,
//                                 r: param0.params[j].r,
//                                 shape: param0.params[j].shape,
//                                 // tmp
//                                 extend_order: param0.params[j].extend_order,
//                             }
//                         })
//                         .collect(),
//                 },
//             )
//         })
//         .filter(|(_, param)| {
//             param.params.iter().all(|p| {
//                 let shape = p.shape;
//                 shape.left >= 0 && shape.top >= 0 && shape.right >= 0 && shape.bottom >= 0
//             })
//         })
//         .map(|(i, param)| (i, generate_rects(n, &param)))
//         .map(|(i, ans)| {
//             if check_validity(xyr, &ans) {
//                 (i, calc_score(xyr, &ans))
//             } else {
//                 (i, 0)
//             }
//         })
//         .for_each(|(i, score)| eprintln!("{} {}", i, score));
// }

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Operation {
    Deform(usize, Dir, i32),
    Deform2((usize, Dir, i32), (usize, Dir, i32)),
    SwapOrder(usize, usize),
    ChangeExtendOrder(usize, [Dir; 4]),
    Regenerate(usize),
    Rotate(usize, bool),
}

use Operation::*;

#[allow(dead_code)]
impl Operation {
    // return reverse op
    fn apply(&self, param: &mut Parameter) -> Operation {
        match self {
            &Deform(idx, dir, val) => {
                let prev_val = param.params[idx].shape.set(dir, val);
                Deform(idx, dir, prev_val)
            }
            &Deform2((idx0, dir0, val0), (idx1, dir1, val1)) => {
                let prev_val0 = param.params[idx0].shape.set(dir0, val0);
                let prev_val1 = param.params[idx1].shape.set(dir1, val1);
                Deform2((idx1, dir1, prev_val1), (idx0, dir0, prev_val0))
            }
            &SwapOrder(idx0, idx1) => {
                param.params.swap(idx0, idx1);
                self.clone()
            }
            &ChangeExtendOrder(idx, order) => {
                let prev_order = replace(&mut param.params[idx].extend_order, order);
                ChangeExtendOrder(idx, prev_order)
            }
            &Regenerate(_idx) => {
                // NOP
                self.clone()
            }
            &Rotate(idx, clockwise) => {
                if clockwise {
                    param.params[idx].shape = Shape {
                        left: param.params[idx].shape.bottom,
                        top: param.params[idx].shape.left,
                        right: param.params[idx].shape.top,
                        bottom: param.params[idx].shape.right,
                    };
                } else {
                    param.params[idx].shape = Shape {
                        left: param.params[idx].shape.top,
                        top: param.params[idx].shape.right,
                        right: param.params[idx].shape.bottom,
                        bottom: param.params[idx].shape.left,
                    };
                }

                Rotate(idx, !clockwise)
            }
        }
    }

    fn get_minimum_affected_index(&self) -> usize {
        match self {
            &Deform(idx, _, _) => idx,
            &Deform2((idx0, _, _), (idx1, _, _)) => min(idx0, idx1),
            &SwapOrder(idx0, idx1) => min(idx0, idx1),
            &ChangeExtendOrder(idx, _) => idx,
            &Regenerate(idx) => idx,
            &Rotate(idx, _) => idx,
        }
    }
}

const MAX_SHAPE: i32 = 100;

use rand::distributions::Distribution;
use rand::SeedableRng;

fn generate_operation<R: rand::Rng>(rng: &mut R, n: usize) -> Operation {
    let dist_type = rand::distributions::Uniform::new(0, 100);
    let dist_idx = rand::distributions::Uniform::new(0, n);
    let dist_dir = rand::distributions::Uniform::new(0, 4);
    let dist_shape = rand::distributions::Uniform::new(0, MAX_SHAPE + 1);

    let t = dist_type.sample(rng);

    if t < 50 {
        SwapOrder(dist_idx.sample(rng), dist_idx.sample(rng))
    } else if t < 80 {
        Regenerate(dist_idx.sample(rng))
    } else if t < 90 {
        Deform(
            dist_idx.sample(rng),
            Dir::from_usize(dist_dir.sample(rng)),
            max(0, dist_shape.sample(rng)),
        )
    // Deform2(
    //     (
    //         dist_idx.sample(rng),
    //         Dir::from_usize(dist_dir.sample(rng)),
    //         dist_shape.sample(rng),
    //     ),
    //     (
    //         dist_idx.sample(rng),
    //         Dir::from_usize(dist_dir.sample(rng)),
    //         dist_shape.sample(rng),
    //     ),
    // )
    } else {
        use rand::seq::SliceRandom;

        let mut v = [Dir::Left, Dir::Top, Dir::Right, Dir::Bottom];
        v.shuffle(rng);

        ChangeExtendOrder(dist_idx.sample(rng), v)
    }
    // } else {
    //     Rotate(dist_idx.sample(rng), rng.gen())
}

use std::time::Instant;

#[allow(dead_code)]
// fn climbing(n: usize, xyr: &[(i32, i32, i32)], time: u128) -> (i32, Vec<Rectangle>) {
//     const SEED: u64 = 42;

//     let mut rng = rand::rngs::SmallRng::seed_from_u64(SEED);
//     let dist_idx = rand::distributions::Uniform::new(0, n);
//     let dist_dir = rand::distributions::Uniform::new(0, 4);
//     let dist = rand::distributions::Uniform::new(0, MAX_SHAPE + 1);

//     let start = Instant::now();

//     let param0 = Parameter {
//         order: (0..n)
//             .sorted_by_key(|&i| (xyr[i].2, xyr[i]))
//             .collect::<Vec<_>>(),
//         shapes: repeat_with(|| loop {
//             let shape = Shape {
//                 left: dist.sample(&mut rng),
//                 top: dist.sample(&mut rng),
//                 right: dist.sample(&mut rng),
//                 bottom: dist.sample(&mut rng),
//             };

//             if !shape.is_zero() {
//                 break shape;
//             }
//         })
//         .take(n)
//         .collect(),
//     };

//     let ans0 = generate_rects(n, xyr, &param0);
//     let score0 = calc_score(xyr, &ans0);

//     // let mut i_trial = 0;
//     unfold((param0, ans0, score0), |(param, ans, score)| {
//         // if i_trial % 100 == 0 {
//         //     eprintln!("{} {} {}", i_trial, start.elapsed().as_millis(), *score);
//         // }
//         // i_trial += 1;

//         let idx = dist_idx.sample(&mut rng);
//         let dir = Dir::from_usize(dist_dir.sample(&mut rng));
//         let val = dist.sample(&mut rng);
//         let i = param.order[idx];

//         let prev_val = param.shapes[i].get(dir);

//         param.shapes[i].set(dir, val);
//         let next_ans = resume_generate_rects(n, xyr, &param, idx, &ans);
//         let next_score = calc_score(xyr, &next_ans);

//         if next_score > *score {
//             *score = next_score;
//             *ans = next_ans;
//             Some(Some(param.clone()))
//         } else {
//             param.shapes[i].set(dir, prev_val);
//             Some(None)
//         }
//     })
//     .take_while(|_| {
//         let t = start.elapsed();
//         t.as_millis() <= time
//     })
//     .flatten()
//     .last()
//     .map(|param| {
//         let ans = generate_rects(n, xyr, &param);
//         (calc_score(xyr, &ans), ans)
//     })
//     .unwrap()
// }

const START_TEMP: f64 = 100000000.0;
// const START_TEMP: f64 = 1000000000.0;
const END_TEMP: f64 = 10000.0;

trait AnnealingController {
    fn temp(&self, i_trial: usize) -> f64;
    fn should_terminate(&self, i_trial: usize) -> bool;
}

struct TimeAnnealingController {
    start: Instant,
    time_ms: u128,
}

#[allow(dead_code)]
impl TimeAnnealingController {
    fn new(time_ms: u128) -> Self {
        TimeAnnealingController {
            start: Instant::now(),
            time_ms,
        }
    }
}

impl AnnealingController for TimeAnnealingController {
    fn temp(&self, _: usize) -> f64 {
        START_TEMP
            * (END_TEMP / START_TEMP)
                .powf(self.start.elapsed().as_secs_f64() * 1000.0 / self.time_ms as f64)
    }

    fn should_terminate(&self, _: usize) -> bool {
        self.start.elapsed().as_millis() > self.time_ms
    }
}

struct NumTrialAnnealingController {
    num_trial: usize,
}

#[allow(dead_code)]
impl NumTrialAnnealingController {
    fn new(num_trial: usize) -> Self {
        NumTrialAnnealingController { num_trial }
    }
}

impl AnnealingController for NumTrialAnnealingController {
    fn temp(&self, i_trial: usize) -> f64 {
        START_TEMP * (END_TEMP / START_TEMP).powf(i_trial as f64 / self.num_trial as f64)
    }

    fn should_terminate(&self, i_trial: usize) -> bool {
        i_trial >= self.num_trial
    }
}

fn annealing<C: AnnealingController>(
    n: usize,
    xyr: &[(i32, i32, i32)],
    controller: &C,
) -> (i32, Vec<(usize, Rectangle)>, Parameter) {
    const SEED: u64 = 42;

    let mut rng = rand::rngs::SmallRng::seed_from_u64(SEED);
    // let mut rng = rand::rngs::SmallRng::from_entropy();
    // let dist_shape = rand::distributions::Uniform::new(0, MAX_SHAPE + 1);
    let dist_prob = rand::distributions::Uniform::new(0.0, 1.0);

    let ixyr = xyr
        .citer()
        .enumerate()
        .map(|(i, (x, y, r))| (i, x, y, r))
        .sorted_by_key(|&(i, x, y, r)| (r, x, y, i))
        .collect::<Vec<_>>();

    let param0 = Parameter {
        params: ixyr
            .citer()
            .map(|(i, x, y, r)| EachParameter {
                i,
                x,
                y,
                r,
                shape: loop {
                    let shape = Shape {
                        // left: dist_shape.sample(&mut rng),
                        // top: dist_shape.sample(&mut rng),
                        // right: dist_shape.sample(&mut rng),
                        // bottom: dist_shape.sample(&mut rng),
                        left: 1,
                        top: 1,
                        right: 1,
                        bottom: 1,
                    };

                    if !shape.is_zero() {
                        break shape;
                    }
                },
                extend_order: {
                    use rand::seq::SliceRandom;

                    let mut v = [Dir::Left, Dir::Top, Dir::Right, Dir::Bottom];
                    v.shuffle(&mut rng);

                    v
                },
            })
            .collect(),
    };

    let ans0 = generate_rects(n, &param0);
    let score0 = calc_score(xyr, &ans0);

    // eprintln!("initial score: {}.", score0);

    unfold(
        (param0.clone(), ans0, score0, score0, 0),
        |(param, ans, score, best_score, i_trial)| {
            if *i_trial % 1000 == 0 {
                // eprintln!("{} {} {}", i_trial, start.elapsed().as_millis(), *score);
                // eprintln!("{} {}", i_trial, *score);
            }
            *i_trial += 1;

            if controller.should_terminate(*i_trial) {
                return None;
            }

            let op = generate_operation(&mut rng, n);

            let temp = controller.temp(*i_trial);

            let rev_op = op.apply(param);
            let next_ans = if let SwapOrder(idx0, idx1) = op {
                let mut reordered_ans = ans.clone();
                reordered_ans.swap(idx0, idx1);
                resume_generate_rects_for_swap_order(
                    n,
                    &param,
                    min(idx0, idx1),
                    max(idx0, idx1),
                    &reordered_ans,
                )
                .unwrap_or(reordered_ans.clone())
            // } else {
            //     let idx = op.get_minimum_affected_index();
            //     resume_generate_rects(n, &param, idx, &ans).unwrap_or(ans.clone())
            // };
            } else if let Regenerate(idx) = op {
                resume_generate_rects(n, &param, idx, &ans).unwrap_or(ans.clone())
            } else {
                ans.clone()
            };

            let next_score = calc_score(xyr, &next_ans);

            if next_score > *score
                || dist_prob.sample(&mut rng) < ((next_score - *score) as f64 / temp).exp()
            {
                *score = next_score;
                *ans = next_ans;
            } else {
                rev_op.apply(param);
            }

            if next_score > *best_score {
                *best_score = next_score;
                Some(Some((*score, ans.clone(), param.clone())))
            } else {
                Some(None)
            }
        },
    )
    .flatten()
    .max_by_key(|t| t.0)
    .unwrap()
}

#[allow(unused_imports)]
use proconio::{input, source::line};
#[allow(unused_imports)]
use std::env;

fn main() {
    // let args = env::args().collect::<Vec<_>>();
    // let input_file = "./tools/in/0012.txt";
    // let input_file = &args[1];
    // let f = std::fs::File::open(input_file).unwrap_or_else(|_| {
    //     eprintln!("no such file exist: {}", input_file);
    //     std::process::exit(1)
    // });
    // let f = line::LineSource::new(std::io::BufReader::new(f));
    input! {
        // from f,
        n: usize,
        xyr: [(i32, i32, i32); n]
    };

    // let (score, ans) = many_climbing(n, &xyr);
    // let (score, ans) = climbing(n, &xyr, 4800);
    let (score, ans, _param) = annealing(n, &xyr, &TimeAnnealingController::new(4900));
    // let (score, ans, _param) = annealing(n, &xyr, &NumTrialAnnealingController::new(80000));

    let mut ans = ans;
    ans.sort_by_key(|t| t.0);
    for (_, rect) in &ans {
        println!("{}", rect);
    }

    // eprintln!("{:?}", _param);

    if !check_validity(&xyr, &ans) {
        eprintln!("invalid");
    } else {
        eprintln!("{}", score);
    }
}
