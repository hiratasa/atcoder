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

const N: usize = 50;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Up,
    Right,
    Down,
}

#[allow(dead_code)]
impl Dir {
    fn apply(&self, i: usize, j: usize) -> Option<(usize, usize)> {
        match self {
            &Dir::Left => {
                if j == 0 {
                    None
                } else {
                    Some((i, j - 1))
                }
            }
            &Dir::Up => {
                if i == 0 {
                    None
                } else {
                    Some((i - 1, j))
                }
            }
            &Dir::Right => {
                if j == N - 1 {
                    None
                } else {
                    Some((i, j + 1))
                }
            }
            &Dir::Down => {
                if i == N - 1 {
                    None
                } else {
                    Some((i + 1, j))
                }
            }
        }
    }

    fn from_usize(idx: usize) -> Dir {
        match idx {
            0 => Dir::Left,
            1 => Dir::Up,
            2 => Dir::Right,
            3 => Dir::Down,
            _ => unreachable!(),
        }
    }

    fn to_str(self) -> &'static str {
        match self {
            Dir::Left => "L",
            Dir::Up => "U",
            Dir::Right => "R",
            Dir::Down => "D",
        }
    }
}

// const START_TEMP: f64 = 400.0;
// const END_TEMP: f64 = 0.1;
const START_TEMP: f64 = 10000.0;
const END_TEMP: f64 = 5.0;

trait AnnealingController {
    fn temp(&self, i_trial: usize) -> f64;
    fn should_terminate(&self, i_trial: usize) -> bool;
}

use std::time::Instant;

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

fn calc_partners(tiles: &[Vec<usize>]) -> Vec<Vec<Option<(usize, usize)>>> {
    tiles
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.citer()
                .enumerate()
                .map(|(j, t)| {
                    [(0, usize::MAX), (0, 1), (usize::MAX, 0), (1, 0)]
                        .citer()
                        .map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
                        .filter(|&(ni, nj)| ni < N && nj < N)
                        .find(|&(ni, nj)| tiles[ni][nj] == t)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VisitStatus {
    NonVisited,
    Visited(u32),
    Visiting,
}

fn dfs0<F: FnMut() -> bool>(
    tiles: &[Vec<usize>],
    points: &[Vec<i32>],
    partners: &[Vec<Option<(usize, usize)>>],
    si: usize,
    sj: usize,
    visited: &mut [bool],
    path: &mut Vec<(usize, usize)>,
    best_path: &mut Vec<(usize, usize)>,
    should_terminate: &mut F,
) {
    if visited[si * N + sj] {
        if path.len() > best_path.len() {
            *best_path = path.clone();
        }
        return;
    }

    if let Some((pi, pj)) = partners[si][sj] {
        if visited[pi * N + pj] {
            if path.len() > best_path.len() {
                *best_path = path.clone();
            }
            return;
        }
    }

    if should_terminate() {
        if path.len() > best_path.len() {
            *best_path = path.clone();
        }
        return;
    }

    visited[si * N + sj] = true;
    path.push((si, sj));

    let dirs = [Dir::Left, Dir::Up, Dir::Right, Dir::Down];
    let ret = dirs
        .citer()
        .filter_map(|dir| dir.apply(si, sj))
        .for_each(|(ni, nj)| {
            dfs0(
                tiles,
                points,
                partners,
                ni,
                nj,
                visited,
                path,
                best_path,
                should_terminate,
            )
        });

    visited[si * N + sj] = false;
    path.pop();

    ret
}

fn random_walk<
    R: Rng,
    F: FnMut() -> bool,
    F2: FnMut(&[(usize, usize)], i32, &[VisitStatus]) -> bool,
>(
    tiles: &[Vec<usize>],
    points: &[Vec<i32>],
    partners: &[Vec<Option<(usize, usize)>>],
    si: usize,
    sj: usize,
    idx0: usize,
    idx1: Option<usize>,
    rng: &mut R,
    visited: &mut [VisitStatus],
    path: &mut Vec<(usize, usize)>,
    cscore: i32,
    should_terminate: &mut F,
    process: &mut F2,
) -> bool {
    if should_terminate() {
        // return false;
        return true;
    }

    let mut idx1 = idx1;
    let new_cscore = cscore + points[si][sj];

    match visited[si * N + sj] {
        VisitStatus::NonVisited => {}
        VisitStatus::Visited(idx2) => {
            let idx2 = idx2 as usize;
            let idx1_ = idx1.unwrap_or(idx2);
            // should same side for idx0
            if (idx0 < idx1_) != (idx0 < idx2) {
                return false;
            }

            if idx1_ == idx2 || (idx0 < idx1_) == (idx1_ < idx2) {
                path.push((si, sj));
                if process(path, new_cscore, visited) {
                    return true;
                }

                path.pop();
                // return false;
            }

            if idx2 == 0 {
                return false;
            }

            if idx0 < idx1_ {
                idx1 = Some(max(idx1_, idx2));
            } else {
                idx1 = Some(min(idx1_, idx2));
            }
        }
        VisitStatus::Visiting => {
            return false;
        }
    }

    if let Some((pi, pj)) = partners[si][sj] {
        match visited[pi * N + pj] {
            VisitStatus::NonVisited => {}
            VisitStatus::Visited(idx2) => {
                if idx2 == 0 {
                    return false;
                }

                let idx2 = idx2 as usize;
                let idx1_ = idx1.unwrap_or(idx2);
                // should same side for idx0
                if (idx0 < idx1_) != (idx0 < idx2) {
                    return false;
                }
                if idx0 < idx1_ {
                    idx1 = Some(max(idx1_, idx2));
                } else {
                    idx1 = Some(min(idx1_, idx2));
                }
            }
            VisitStatus::Visiting => {
                return false;
            }
        }
    }

    let old_visited = replace(&mut visited[si * N + sj], VisitStatus::Visiting);
    let old_visited_partner = partners[si][sj]
        .map(|(pi, pj)| replace(&mut visited[pi * N + pj], VisitStatus::NonVisited));
    path.push((si, sj));

    let mut dirs = [Dir::Left, Dir::Up, Dir::Right, Dir::Down];
    dirs.shuffle(rng);
    let ret = dirs
        .citer()
        .filter_map(|dir| dir.apply(si, sj))
        .any(|(ni, nj)| {
            random_walk(
                tiles,
                points,
                partners,
                ni,
                nj,
                idx0,
                idx1,
                rng,
                visited,
                path,
                new_cscore,
                should_terminate,
                process,
            )
        });

    visited[si * N + sj] = old_visited;
    if let Some((pi, pj)) = partners[si][sj] {
        visited[pi * N + pj] = old_visited_partner.unwrap();
    }
    if !ret {
        if idx1.is_none() {
            if process(path, new_cscore, visited) {
                return true;
            }
        }
        path.pop();
    }

    ret
}

#[allow(dead_code)]
fn is_valid_path(path: &[(usize, usize)]) -> bool {
    path.citer().tuple_windows().all(|((i0, j0), (i1, j1))| {
        (i1 as i64 - i0 as i64).abs() + (j1 as i64 - j0 as i64).abs() == 1
    })
}

fn solve(
    tiles: &[Vec<usize>],
    points: &[Vec<i32>],
    si: usize,
    sj: usize,
) -> (Vec<(usize, usize)>, i32) {
    let partners = calc_partners(tiles);

    let start0 = Instant::now();

    let mut init_path = vec![];
    dfs0(
        tiles,
        points,
        &partners,
        si,
        sj,
        &mut vec![false; N * N],
        &mut vec![],
        &mut init_path,
        &mut || start0.elapsed().as_millis() > 100,
    );

    const SEED: u64 = 42;

    let mut rng = rand::rngs::SmallRng::seed_from_u64(SEED);

    let dist_prob = rand::distributions::Uniform::new(0.0, 1.0);

    let mut visited = vec![VisitStatus::NonVisited; N * N];

    init_path.citer().enumerate().for_each(|(idx, (i, j))| {
        visited[i * N + j] = VisitStatus::Visited(idx as u32);
    });

    let controller = TimeAnnealingController::new(1800);
    // let controller = TimeAnnealingController::new(60000);

    // tmp
    // let mut sum_nanos = 0;
    let init_cscores = once(0i32)
        .chain(init_path.citer().map(|(ii, jj)| points[ii][jj]))
        .cumsum::<i32>()
        .collect::<Vec<_>>();
    let init_score = init_cscores[init_path.len()];
    eprintln!("init score: {}", init_score);
    let path = unfold(
        (
            visited,
            init_path.clone(),
            init_cscores,
            init_score,
            init_score,
            0usize,
        ),
        |(visited, path, cscores, score, best_score, i_trial)| {
            *i_trial += 1;

            if *i_trial % 20000 == 0 {
                eprintln!("i_trial: {}", *i_trial);
            }

            if controller.should_terminate(*i_trial) {
                return None;
            }

            let dist_idx = rand::distributions::Uniform::new(0, path.len());
            let idx = dist_idx.sample(&mut rng);

            let (ii, jj) = path[idx];
            assert!(
                visited[ii * N + jj] == VisitStatus::Visited(idx as u32),
                "{} {} {} {:?}",
                ii,
                jj,
                idx,
                visited[ii * N + jj]
            );
            visited[ii * N + jj] = VisitStatus::NonVisited;

            let start1 = Instant::now();
            let mut sub_path = vec![];
            // let mut best_score_diff = 0;
            let mut best_score_diff = std::i32::MIN;
            let mut i_check_timer = 0;

            let _ok = random_walk(
                tiles,
                points,
                &partners,
                ii,
                jj,
                idx,
                None,
                &mut rng,
                visited,
                &mut vec![],
                0,
                &mut || {
                    i_check_timer += 1;
                    if i_check_timer % 16 == 0 {
                        // start1.elapsed().as_micros() > 10
                        start1.elapsed().as_micros() > 5
                    } else {
                        false
                    }
                },
                &mut |sub_path2: &[(usize, usize)], sub_score: i32, visited_: &[VisitStatus]| {
                    let (ii2, jj2) = *sub_path2.last().unwrap();
                    let idx2 = match visited_[ii2 * N + jj2] {
                        VisitStatus::Visited(idx2) => idx2 as usize,
                        _ => path.len() - 1,
                    };

                    let score_diff =
                        sub_score - (cscores[max(idx, idx2) + 1] - cscores[min(idx, idx2)]);

                    if score_diff > best_score_diff {
                        best_score_diff = score_diff;
                        sub_path = sub_path2.to_vec();
                        // true
                        false
                    } else {
                        // true
                        false
                    }
                    // idx2 != path.len() - 1
                },
            );
            // sum_nanos += start1.elapsed().as_nanos();
            // if *i_trial > 0 && *i_trial % 100000 == 0 {
            //     eprintln!("{} {}ns", *i_trial, sum_nanos / *i_trial as u128);
            // }

            visited[ii * N + jj] = VisitStatus::Visited(idx as u32);

            // if !ok {
            //     return Some(None);
            // }
            if sub_path.is_empty() {
                return Some(None);
            }

            assert!(sub_path[0] == (ii, jj));

            let (ii2, jj2) = *sub_path.last().unwrap();
            let idx2 = match visited[ii2 * N + jj2] {
                VisitStatus::NonVisited => path.len() - 1,
                VisitStatus::Visited(idx2) => idx2 as usize,
                _ => unreachable!(),
            };

            // let old_len = (idx2 as i64 - idx as i64).abs() + 1;
            // let new_len = sub_path.len() as i64;
            // let new_score = *score - old_len + new_len;

            // let new_score = *score + sub_path.citer().map(|(ii, jj)| points[ii][jj]).sum::<i64>()
            //     - (cscores[max(idx, idx2) + 1] - cscores[min(idx, idx2)]);
            let new_score = *score + best_score_diff;
            let temp = controller.temp(*i_trial);
            if new_score >= *score
                || dist_prob.sample(&mut rng) < ((new_score - *score) as f64 / temp).exp()
            {
                // eprintln!("{}", new_score);
                path[min(idx, idx2)..=max(idx, idx2)]
                    .citer()
                    .for_each(|(i, j)| {
                        visited[i * N + j] = VisitStatus::NonVisited;
                    });
                if idx <= idx2 {
                    path.splice(idx..=idx2, sub_path.citer());
                } else {
                    path.splice(idx2..=idx, sub_path.citer().rev());
                }
                // assert!(
                //     is_valid_path(&path),
                //     "{} {} {:?} {:?} {:?}",
                //     idx,
                //     idx2,
                //     old_path,
                //     path,
                //     sub_path
                // );
                let midx = min(idx, idx2);
                cscores.resize(path.len() + 1, 0);
                for (i_in_path, (i, j)) in path[midx..].citer().enumerate() {
                    visited[i * N + j] = VisitStatus::Visited((i_in_path + midx) as u32);
                    cscores[i_in_path + midx + 1] = cscores[i_in_path + midx] + points[i][j];
                }

                // iproduct!(0..N, 0..N).for_each(|(i, j)| {
                //     if let Some(idx_tmp) = path.citer().position(|(ii, jj)| i == ii && j == jj) {
                //         assert_eq!(
                //             visited[i][j],
                //             VisitStatus::Visited(idx_tmp),
                //             "{} {} {} {} {} {} {:?}",
                //             i,
                //             j,
                //             idx,
                //             idx2,
                //             sub_path.len(),
                //             idx_tmp,
                //             sub_path,
                //         );
                //     } else {
                //         assert_eq!(
                //             visited[i][j],
                //             VisitStatus::NonVisited,
                //             "{} {} {} {}",
                //             idx,
                //             idx2,
                //             sub_path.len(),
                //             '-'
                //         );
                //     }
                // });

                *score = new_score;

                if *score > *best_score {
                    *best_score = *score;
                    // eprintln!("best_score: {}", *best_score);
                    Some(Some(path.clone()))
                } else {
                    Some(None)
                }
            } else {
                Some(None)
            }
        },
    )
    .flatten()
    .last()
    .unwrap_or(init_path.clone());

    let score = path.citer().map(|(ii, jj)| points[ii][jj]).sum::<i32>();
    (path, score)
}

fn main() {
    let (si, sj) = read_tuple!(usize, usize);

    let t = read_vec(N, || read_row::<usize>());
    let points = read_vec(N, || read_row::<i32>());

    let (path, score) = solve(&t, &points, si, sj);

    eprintln!("{}", score);

    let ans = path
        .citer()
        .tuple_windows()
        .map(|((i0, j0), (i1, j1))| match (i1, j1) {
            _ if (i1, j1) == (i0, j0.wrapping_sub(1)) => 'L',
            _ if (i1, j1) == (i0, j0 + 1) => 'R',
            _ if (i1, j1) == (i0.wrapping_sub(1), j0) => 'U',
            _ if (i1, j1) == (i0 + 1, j0) => 'D',
            _ => unreachable!(),
        })
        .join("");
    println!("{}", ans);
}
