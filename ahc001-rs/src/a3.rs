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

#[derive(Debug, Clone, Copy)]
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

fn find_overlap(ans: &[Rectangle], i: usize, j0: usize, rect: &Rectangle) -> Option<usize> {
    ans[j0..]
        .iter()
        .enumerate()
        .map(|(j, rect0)| (j0 + j, rect0))
        .find(|(j, rect0)| *j != i && rect0.overlap_with(rect))
        .map(|(j, _)| j)
}

#[derive(Debug, Clone)]
struct OverlapChecker {
    rects: Vec<Rectangle>,
    tate_edges_map: BTreeMap<(i32, usize), (i32, i32)>,
    yoko_edges_map: BTreeMap<(i32, usize), (i32, i32)>,
}

#[allow(dead_code)]
impl OverlapChecker {
    fn new(n: usize) -> Self {
        Self {
            rects: vec![Rectangle::new(0, 0, 0, 0); n],
            tate_edges_map: BTreeMap::new(),
            yoko_edges_map: BTreeMap::new(),
        }
    }

    fn register_tate(&mut self, i: usize, x: i32, y0: i32, y1: i32) {
        self.tate_edges_map.insert((x, i), (y0, y1));
    }

    fn register_yoko(&mut self, i: usize, y: i32, x0: i32, x1: i32) {
        self.yoko_edges_map.insert((y, i), (x0, x1));
    }

    fn register_rect(&mut self, i: usize, rect: &Rectangle) {
        self.register_tate(i, rect.left, rect.top, rect.bottom);
        self.register_tate(i, rect.right, rect.top, rect.bottom);
        self.register_yoko(i, rect.top, rect.left, rect.right);
        self.register_yoko(i, rect.bottom, rect.left, rect.right);
    }

    fn unregister_tate(&mut self, i: usize, x: i32) {
        self.tate_edges_map.remove(&(x, i));
    }

    fn unregister_yoko(&mut self, i: usize, y: i32) {
        self.yoko_edges_map.remove(&(y, i));
    }

    fn unregister_rect(&mut self, i: usize, rect: &Rectangle) {
        self.unregister_tate(i, rect.left);
        self.unregister_tate(i, rect.right);
        self.unregister_yoko(i, rect.top);
        self.unregister_yoko(i, rect.bottom);
    }

    fn replace_rect(&mut self, i: usize, rect: &Rectangle) {
        self.unregister_rect(i, &self.rects[i].clone());
        self.register_rect(i, &rect);
        self.rects[i] = *rect;
    }

    fn find_overlap(&self, i: usize, _j0: usize, rect: &Rectangle) -> Option<usize> {
        use std::ops::Bound::{Excluded, Included};
        if self
            .tate_edges_map
            .range((Included(&(rect.left + 1, 0)), Excluded(&(rect.right, 0))))
            .any(|(&(x, j), &(y0, y1))| j != i && y1 > rect.top && y0 < rect.bottom)
            || self
                .yoko_edges_map
                .range((Included(&(rect.top + 1, 0)), Excluded(&(rect.bottom, 0))))
                .any(|(&(y, j), &(x0, x1))| j != i && x1 > rect.left && x0 < rect.right)
        {
            // tmp
            Some(0)
        } else {
            None
        }
    }
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
struct Parameter {
    ixyr: Vec<(usize, i32, i32, i32)>,
    shapes: Vec<Shape>,
}

fn extend_rect(
    i: usize,
    r: i32,
    shape: &Shape,
    rect0: &Rectangle,
    ans: &[Rectangle],
    overlap_checker: &OverlapChecker,
) -> Rectangle {
    let extend_left = |rect1: Rectangle| {
        let mut j0 = 0;
        let left = lower_bound_int(0, rect1.left, |left| {
            let rect = Rectangle { left, ..rect1 };
            if rect.area() > r {
                Ordering::Less
            } else if let Some(j) = overlap_checker.find_overlap(i, j0, &rect) {
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
        let right = lower_bound_int(rect1.right, B + 1, |right| {
            let rect = Rectangle { right, ..rect1 };
            if rect.area() > r {
                Ordering::Greater
            } else if let Some(j) = overlap_checker.find_overlap(i, j0, &rect) {
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
            } else if let Some(j) = overlap_checker.find_overlap(i, j0, &rect) {
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
        let bottom = lower_bound_int(rect1.bottom, B + 1, |bottom| {
            let rect = Rectangle { bottom, ..rect1 };
            if rect.area() > r {
                Ordering::Greater
            } else if let Some(j) = overlap_checker.find_overlap(i, j0, &rect) {
                // rectangles between [j0, j) have no overlap after now.
                j0 = j;
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }) - 1;
        Rectangle { bottom, ..rect1 }
    };

    [Dir::Left, Dir::Top, Dir::Right, Dir::Bottom]
        .citer()
        .sorted_by_key(|&dir| Reverse(shape.get(dir)))
        .fold(rect0.clone(), |rect, dir| match dir {
            Dir::Left => extend_left(rect),
            Dir::Top => extend_top(rect),
            Dir::Right => extend_right(rect),
            Dir::Bottom => extend_bottom(rect),
        })
}

fn generate_rect(
    i: usize,
    x: i32,
    y: i32,
    r: i32,
    shape: &Shape,
    ans: &[Rectangle],
    overlap_checker: &OverlapChecker,
) -> Rectangle {
    let mut j0 = 0;
    let z = lower_bound_int(0, B + 1, |z| {
        let rect = Rectangle::new(
            x - z * shape.left,
            y - z * shape.top,
            x + 1 + z * shape.right,
            y + 1 + z * shape.bottom,
        );
        if !rect.valid() || rect.area() > r {
            Ordering::Greater
        } else if let Some(j) = overlap_checker.find_overlap(i, j0, &rect) {
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

    extend_rect(i, r, shape, &rect0, ans, overlap_checker)
}

fn generate_rects(n: usize, param: &Parameter) -> (Vec<Rectangle>, OverlapChecker) {
    let (ans0, overlap_checker) = param.ixyr.citer().fold(
        (vec![Rectangle::new(0, 0, 0, 0); n], OverlapChecker::new(n)),
        |(mut rects, mut overlap_checker), (i, x, y, _r)| {
            rects[i] = Rectangle::new(x, y, x + 1, y + 1);
            overlap_checker.register_rect(i, &rects[i]);
            (rects, overlap_checker)
        },
    );

    izip!(param.ixyr.citer(), param.shapes.iter()).fold(
        (ans0, overlap_checker),
        |(mut ans, mut overlap_checker), ((i, x, y, r), shape)| {
            ans[i] = generate_rect(i, x, y, r, shape, &ans, &overlap_checker);
            overlap_checker.replace_rect(i, &ans[i]);

            (ans, overlap_checker)
        },
    )
}

fn resume_generate_rects(
    _n: usize,
    param: &Parameter,
    idx: usize,
    prev_ans: &Vec<Rectangle>,
    prev_overlap_checker: &OverlapChecker,
) -> (Vec<Rectangle>, OverlapChecker) {
    let (ans0, overlap_checker) = param.ixyr[idx..].citer().fold(
        (prev_ans.clone(), prev_overlap_checker.clone()),
        |(mut ans, mut overlap_checker), (i, x, y, _r)| {
            ans[i] = Rectangle::new(x, y, x + 1, y + 1);
            overlap_checker.replace_rect(i, &ans[i]);
            (ans, overlap_checker)
        },
    );

    izip!(param.ixyr[idx..].citer(), param.shapes[idx..].iter()).fold(
        (ans0, overlap_checker),
        |(mut ans, mut overlap_checker), ((i, x, y, r), shape)| {
            if i == idx {
                ans[i] = generate_rect(i, x, y, r, shape, &ans, &overlap_checker);
            } else if overlap_checker.find_overlap(i, 0, &prev_ans[i]).is_none() {
                // This is incorrect.
                ans[i] = extend_rect(i, r, shape, &prev_ans[i], &ans, &overlap_checker);
            } else {
                ans[i] = generate_rect(i, x, y, r, shape, &ans, &overlap_checker);
            }
            overlap_checker.replace_rect(i, &ans[i]);

            (ans, overlap_checker)
        },
    )
}

fn check_validity(_xyr: &[(i32, i32, i32)], ans: &[Rectangle]) -> bool {
    ans.citer().all(|rect| rect.valid())
        && ans
            .citer()
            .enumerate()
            .all(|(i, rect)| ans[i + 1..].citer().all(|rect1| !rect.overlap_with(&rect1)))
}

fn calc_score(xyr: &[(i32, i32, i32)], ans: &[Rectangle]) -> i32 {
    let n = xyr.len();
    izip!(xyr.citer(), ans)
        .map(|((x, y, r), rect)| {
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

#[allow(dead_code)]
fn print_line(xyr: &[(i32, i32, i32)], param0: &Parameter, param1: &Parameter) {
    const M: i32 = 100;

    let n = param0.ixyr.len();

    assert_eq!(param0.ixyr, param1.ixyr);

    (-M..=2 * M)
        .map(|i| {
            (
                i,
                Parameter {
                    ixyr: param0.ixyr.clone(),
                    shapes: (0..n)
                        .map(|j| Shape {
                            left: (param0.shapes[j].left * (M - i) + param1.shapes[j].left * i) / M,
                            top: (param0.shapes[j].top * (M - i) + param1.shapes[j].top * i) / M,
                            right: (param0.shapes[j].right * (M - i) + param1.shapes[j].right * i)
                                / M,
                            bottom: (param0.shapes[j].bottom * (M - i)
                                + param1.shapes[j].bottom * i)
                                / M,
                        })
                        .collect(),
                },
            )
        })
        .filter(|(_, param)| {
            param.shapes.iter().all(|shape| {
                shape.left >= 0 && shape.top >= 0 && shape.right >= 0 && shape.bottom >= 0
            })
        })
        .map(|(i, param)| (i, generate_rects(n, &param).0))
        .map(|(i, ans)| {
            if check_validity(xyr, &ans) {
                (i, calc_score(xyr, &ans))
            } else {
                (i, 0)
            }
        })
        .for_each(|(i, score)| eprintln!("{} {}", i, score));
}

#[derive(Debug, Clone)]
enum Operation {
    Deformation(usize, Dir, i32),
    SwapOrder(usize, usize),
    Rotate(usize, bool),
}

use Operation::*;

impl Operation {
    // return reverse op
    fn apply(&self, param: &mut Parameter) -> Operation {
        match self {
            &Deformation(idx, dir, val) => {
                let prev_val = param.shapes[idx].set(dir, val);
                Deformation(idx, dir, prev_val)
            }
            &SwapOrder(idx0, idx1) => {
                param.ixyr.swap(idx0, idx1);
                param.shapes.swap(idx0, idx1);
                self.clone()
            }
            &Rotate(idx, clockwise) => {
                if clockwise {
                    param.shapes[idx] = Shape {
                        left: param.shapes[idx].bottom,
                        top: param.shapes[idx].left,
                        right: param.shapes[idx].top,
                        bottom: param.shapes[idx].right,
                    };
                } else {
                    param.shapes[idx] = Shape {
                        left: param.shapes[idx].top,
                        top: param.shapes[idx].right,
                        right: param.shapes[idx].bottom,
                        bottom: param.shapes[idx].left,
                    };
                }

                Rotate(idx, !clockwise)
            }
        }
    }

    fn get_minimum_affected_index(&self) -> usize {
        match self {
            &Deformation(idx, _, _) => idx,
            &SwapOrder(idx0, idx1) => min(idx0, idx1),
            &Rotate(idx, _) => idx,
        }
    }
}

const MAX_SHAPE: i32 = 100;

use rand::distributions::Distribution;
use rand::SeedableRng;

fn generate_operation<R: rand::Rng>(rng: &mut R, n: usize) -> Operation {
    let dist_type = rand::distributions::Uniform::new(0, 2);
    let dist_idx = rand::distributions::Uniform::new(0, n);
    let dist_dir = rand::distributions::Uniform::new(0, 4);
    let dist_shape = rand::distributions::Uniform::new(0, MAX_SHAPE + 1);

    let t = dist_type.sample(rng);

    if t == 0 {
        SwapOrder(dist_idx.sample(rng), dist_idx.sample(rng))
    } else if t == 1 {
        Deformation(
            dist_idx.sample(rng),
            Dir::from_usize(dist_dir.sample(rng)),
            dist_shape.sample(rng),
        )
    } else {
        Rotate(dist_idx.sample(rng), rng.gen())
    }
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

fn annealing(n: usize, xyr: &[(i32, i32, i32)], time: u128) -> (i32, Vec<Rectangle>, Parameter) {
    const SEED: u64 = 42;

    let mut rng = rand::rngs::SmallRng::seed_from_u64(SEED);
    // let mut rng = rand::rngs::SmallRng::from_entropy();
    let dist_shape = rand::distributions::Uniform::new(0, MAX_SHAPE + 1);
    let dist_prob = rand::distributions::Uniform::new(0.0, 1.0);

    const START_TEMP: f64 = 100000000.0;
    const END_TEMP: f64 = 10000.0;

    let start = Instant::now();

    let ixyr = xyr
        .citer()
        .enumerate()
        .map(|(i, (x, y, r))| (i, x, y, r))
        .sorted_by_key(|&(i, x, y, r)| (r, x, y, i))
        .collect::<Vec<_>>();

    let param0 = Parameter {
        ixyr: ixyr,
        shapes: repeat_with(|| loop {
            let shape = Shape {
                left: dist_shape.sample(&mut rng),
                top: dist_shape.sample(&mut rng),
                right: dist_shape.sample(&mut rng),
                bottom: dist_shape.sample(&mut rng),
            };

            if !shape.is_zero() {
                break shape;
            }
        })
        .take(n)
        .collect(),
    };

    let (ans0, overlap_checker0) = generate_rects(n, &param0);
    let score0 = calc_score(xyr, &ans0);

    const M: usize = 160000;

    unfold(
        (param0.clone(), ans0, overlap_checker0, score0, score0, 0),
        |(param, ans, overlap_checker, score, best_score, i_trial)| {
            if *i_trial % 1000 == 0 {
                eprintln!("{} {} {}", i_trial, start.elapsed().as_millis(), *score);
            }
            *i_trial += 1;

            if *i_trial == M {
                return None;
            }

            let op = generate_operation(&mut rng, n);
            let idx = op.get_minimum_affected_index();

            // let temp = START_TEMP
            //     * (END_TEMP / START_TEMP)
            //         .powf(start.elapsed().as_secs_f64() * 1000.0 / time as f64);
            let temp = START_TEMP * (END_TEMP / START_TEMP).powf(*i_trial as f64 / M as f64);

            let rev_op = op.apply(param);

            let (next_ans, next_overlap_checker) =
                resume_generate_rects(n, &param, idx, &ans, &overlap_checker);
            let next_score = calc_score(xyr, &next_ans);

            if next_score > *score
                || dist_prob.sample(&mut rng) < ((next_score - *score) as f64 / temp).exp()
            {
                *score = next_score;
                *ans = next_ans;
                *overlap_checker = next_overlap_checker;
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
    .take_while(|_| {
        let t = start.elapsed();
        t.as_millis() <= time
    })
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
    // let (score, ans, _param) = annealing(n, &xyr, 4800);
    let (score, ans, _param) = annealing(n, &xyr, 300000);

    for rect in &ans {
        println!("{}", rect);
    }

    if !check_validity(&xyr, &ans) {
        eprintln!("invalid");
    } else {
        eprintln!("{}", score);
    }
}
