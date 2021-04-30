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

const N: usize = 50;

#[allow(dead_code)]
fn solve(
    si: usize,
    sj: usize,
    m: usize,
    tiles: &Vec<Vec<usize>>,
    points: &Vec<Vec<usize>>,
) -> (Vec<(usize, usize)>, usize) {
    let mut bs0 = bitset!(m, 0);
    bs0.set(tiles[si][sj], true);

    const W: usize = 1000;
    let mut last_states = iterate(vec![(vec![(si, sj)], points[si][sj], bs0)], |prev_states| {
        prev_states
            .iter()
            .flat_map(|(path, score, bs)| {
                let (i, j) = path.last().copied().unwrap();
                let tiles = &tiles;
                let points = &points;
                let path = path.clone();
                let bs = bs.clone();
                [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)]
                    .citer()
                    .map(move |(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
                    .filter(|&(ni, nj)| ni < N && nj < N)
                    .filter_map(move |(ni, nj)| {
                        if bs[tiles[ni][nj]] {
                            None
                        } else {
                            let mut bs2 = bs.clone();
                            bs2.set(tiles[ni][nj], true);
                            let mut path2 = path.clone();
                            path2.push((ni, nj));
                            Some((path2, score + points[ni][nj], bs2))
                        }
                    })
            })
            .sorted_by_key(|(_, score, _)| Reverse(*score))
            .take(W)
            .collect::<Vec<_>>()
    })
    .take_while(|states| !states.is_empty())
    .last()
    .unwrap();

    (replace(&mut last_states[0].0, vec![]), last_states[0].1)
}

#[derive(Clone, Debug)]
struct State {
    path: Vec<(u8, u8)>,
    score: usize,
    visited: BitSet,
}

impl State {
    fn new(si: usize, sj: usize, score0: usize, tile0: usize, m: usize) -> State {
        let mut visited = bitset!(m, 0);
        visited.set(tile0, true);

        State {
            path: vec![(si as u8, sj as u8)],
            score: score0,
            visited,
        }
    }
}

#[derive(Clone, Debug)]
struct StateScore(State);

impl PartialEq for StateScore {
    fn eq(&self, other: &Self) -> bool {
        // self.0.score == other.0.score
        self.0.path.len() == other.0.path.len()
    }
}

impl Eq for StateScore {}

impl Ord for StateScore {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.0.score.cmp(&other.0.score)
        self.0.path.len().cmp(&other.0.path.len())
    }
}

impl PartialOrd for StateScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn solve2(
    si: usize,
    sj: usize,
    m: usize,
    tiles: &Vec<Vec<usize>>,
    points: &Vec<Vec<usize>>,
) -> (Vec<(u8, u8)>, usize) {
    const TIME: u128 = 1500;

    use std::time::Instant;
    let start = Instant::now();

    let best_state = (0..)
        .scan(
            vvec![once(StateScore(State::new(
            si,
            sj,
            points[si][sj],
            tiles[si][sj],
            m,
        )))
        .collect::<BinaryHeap<_>>(); BinaryHeap::new(); N * N + 1],
            |qs, _| {
                (0..N * N)
                    .filter_map(|idx| {
                        if qs[idx].is_empty() {
                            return None;
                        }

                        let StateScore(state) = qs[idx].pop().unwrap();
                        let path = &state.path;
                        let score = state.score;
                        let visited = &state.visited;

                        let (i, j) = path.last().copied().unwrap();

                        qs[idx as usize + 1].extend(
                            [(0, 1), (0, std::u8::MAX), (1, 0), (std::u8::MAX, 0)]
                                .citer()
                                .map(move |(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj)))
                                .map(|(ni, nj)| (ni as usize, nj as usize))
                                .filter(|&(ni, nj)| ni < N && nj < N)
                                .filter_map(move |(ni, nj)| {
                                    if visited[tiles[ni][nj]] {
                                        None
                                    } else {
                                        let mut visited2 = visited.clone();
                                        visited2.set(tiles[ni][nj], true);
                                        // visited2[tiles[ni][nj]] = true;
                                        let mut path2 = path.clone();
                                        path2.push((ni as u8, nj as u8));
                                        Some(State {
                                            path: path2,
                                            score: score + points[ni][nj],
                                            visited: visited2,
                                        })
                                    }
                                })
                                .map(|state| StateScore(state)),
                        );

                        Some(state)
                    })
                    .max_by_key(|state| state.score)
            },
        )
        .take_while(|_| start.elapsed().as_millis() < TIME)
        .max_by_key(|state| state.score)
        .unwrap();
    (best_state.path, best_state.score)
}

#[allow(dead_code)]
fn solve3(
    si: usize,
    sj: usize,
    m: usize,
    tiles: &Vec<Vec<usize>>,
    points: &Vec<Vec<usize>>,
) -> (Vec<(u8, u8)>, usize) {
    const TIME: u128 = 1500;
    // const TIME: u128 = 15000;

    use std::time::Instant;
    let start = Instant::now();

    let mut qs0 = vec![vec![BinaryHeap::new(); N]; N];
    qs0[si][sj] = once(StateScore(State::new(
        si,
        sj,
        points[si][sj],
        tiles[si][sj],
        m,
    )))
    .collect::<BinaryHeap<_>>();

    let best_state = (0..)
        .scan(qs0, |qs, _| {
            iproduct!(0..N, 0..N)
                .filter_map(|(i, j)| {
                    if qs[i][j].is_empty() {
                        return None;
                    }

                    let StateScore(state) = qs[i][j].pop().unwrap();
                    let path = &state.path;
                    let score = state.score;
                    let visited = &state.visited;

                    [(0, 1), (0, std::u8::MAX), (1, 0), (std::u8::MAX, 0)]
                        .citer()
                        .map(move |(di, dj)| {
                            ((i as u8).wrapping_add(di), (j as u8).wrapping_add(dj))
                        })
                        .map(|(ni, nj)| (ni as usize, nj as usize))
                        .filter(|&(ni, nj)| ni < N && nj < N)
                        .filter_map(move |(ni, nj)| {
                            if visited[tiles[ni][nj]] {
                                None
                            } else {
                                let mut visited2 = visited.clone();
                                visited2.set(tiles[ni][nj], true);
                                let mut path2 = path.clone();
                                path2.push((ni as u8, nj as u8));
                                Some(State {
                                    path: path2,
                                    score: score + points[ni][nj],
                                    visited: visited2,
                                })
                            }
                        })
                        .for_each(|state| {
                            let (ni, nj) = state.path.last().copied().unwrap();
                            qs[ni as usize][nj as usize].push(StateScore(state));
                        });

                    Some(state)
                })
                .max_by_key(|state| state.score)
        })
        .take_while(|_| start.elapsed().as_millis() < TIME)
        .max_by_key(|state| state.score)
        .unwrap();
    (best_state.path, best_state.score)
}

#[allow(dead_code)]
fn solve4(
    si: usize,
    sj: usize,
    m: usize,
    tiles: &Vec<Vec<usize>>,
    points: &Vec<Vec<usize>>,
) -> (Vec<(u8, u8)>, usize) {
    const TIME: u128 = 1500;
    // const TIME: u128 = 15000;

    use std::time::Instant;
    let start = Instant::now();

    let q0 = once(StateScore(State::new(
        si,
        sj,
        points[si][sj],
        tiles[si][sj],
        m,
    )))
    .collect::<BinaryHeap<_>>();

    let best_state = (0..)
        .scan(q0, |q, _| {
            let StateScore(state) = q.pop()?;

            let path = &state.path;
            let score = state.score;
            let visited = &state.visited;

            let (i, j) = path.last().copied().unwrap();

            q.extend(
                [(0, 1), (0, std::u8::MAX), (1, 0), (std::u8::MAX, 0)]
                    .citer()
                    .map(move |(di, dj)| ((i as u8).wrapping_add(di), (j as u8).wrapping_add(dj)))
                    .map(|(ni, nj)| (ni as usize, nj as usize))
                    .filter(|&(ni, nj)| ni < N && nj < N)
                    .filter_map(move |(ni, nj)| {
                        if visited[tiles[ni][nj]] {
                            None
                        } else {
                            let mut visited2 = visited.clone();
                            visited2.set(tiles[ni][nj], true);
                            let mut path2 = path.clone();
                            path2.push((ni as u8, nj as u8));
                            Some(State {
                                path: path2,
                                score: score + points[ni][nj],
                                visited: visited2,
                            })
                        }
                    })
                    .map(|state| StateScore(state)),
            );

            Some(state)
        })
        .take_while(|_| start.elapsed().as_millis() < TIME)
        .max_by_key(|state| state.score)
        .unwrap();
    (best_state.path, best_state.score)
}

#[allow(dead_code)]
fn solve5(
    si: usize,
    sj: usize,
    m: usize,
    tiles: &Vec<Vec<usize>>,
    points: &Vec<Vec<usize>>,
) -> (Vec<(u8, u8)>, usize) {
    const TIME: u128 = 1500;

    use std::time::Instant;
    let start = Instant::now();

    let mut qs0 = vec![vec![None; N]; N];
    qs0[si][sj] = Some(State::new(si, sj, points[si][sj], tiles[si][sj], m));

    let best_state = iterate(qs0, |qs| {
        let mut next: Vec<Vec<Option<State>>> = vec![vec![None; N]; N];

            iproduct!(0..N, 0..N)
                .for_each(|(i, j)| {
                    if qs[i][j].is_none() {
                        return;
                    }

                    let state = qs[i][j].as_ref().unwrap();
                    let path = &state.path;
                    let score = state.score;
                    let visited = &state.visited;

                    [(0, 1), (0, std::u8::MAX), (1, 0), (std::u8::MAX, 0)]
                        .citer()
                        .map(move |(di, dj)| {
                            ((i as u8).wrapping_add(di), (j as u8).wrapping_add(dj))
                        })
                        .map(|(ni, nj)| (ni as usize, nj as usize))
                        .filter(|&(ni, nj)| ni < N && nj < N)
                        .filter_map(move |(ni, nj)| {
                            if visited[tiles[ni][nj]] {
                                None
                            } else {
                                let mut visited2 = visited.clone();
                                visited2.set(tiles[ni][nj], true);
                                let mut path2 = path.clone();
                                path2.push((ni as u8, nj as u8));
                                Some(State {
                                    path: path2,
                                    score: score + points[ni][nj],
                                    visited: visited2,
                                })
                            }
                        })
                        .for_each(|state| {
                            let (ni, nj) = state.path.last().copied().unwrap();

                            if !matches!(&next[ni as usize][nj as usize], Some(state2) if state2.score > state.score) {
                                next[ni as usize][nj as usize] = Some(state);
                            }
                        });
                });

                next
        })
        .take_while(|_| start.elapsed().as_millis() < TIME)
        .filter_map(|qs| {
            qs.into_iter().filter_map(|row| row.into_iter().flatten().max_by_key(|state| state.score)).max_by_key(|state| state.score)
        })
        .max_by_key(|state| state.score)
        .unwrap();
    (best_state.path, best_state.score)
}

#[allow(dead_code)]
fn solve6(
    si: usize,
    sj: usize,
    m: usize,
    tiles: &Vec<Vec<usize>>,
    points: &Vec<Vec<usize>>,
) -> (Vec<(u8, u8)>, usize) {
    const TIME: u128 = 1500;
    // const TIME: u128 = 60000;

    use std::time::Instant;
    let start = Instant::now();

    let state0 = State::new(si, sj, points[si][sj], tiles[si][sj], m);

    let mut qs0 = vec![vec![BinaryHeap::new(); N]; N];
    qs0[si][sj] = once(StateScore(state0.clone())).collect::<BinaryHeap<_>>();

    let mut best_state = state0;
    let mut qs = qs0;
    loop {
        let state = iproduct!(0..N, 0..N)
            .filter_map(|(i, j)| {
                if qs[i][j].is_empty() {
                    return None;
                }

                let StateScore(state) = qs[i][j].pop().unwrap();
                let path = &state.path;
                let score = state.score;
                let visited = &state.visited;

                [(0, 1), (0, std::u8::MAX), (1, 0), (std::u8::MAX, 0)]
                    .citer()
                    .map(move |(di, dj)| ((i as u8).wrapping_add(di), (j as u8).wrapping_add(dj)))
                    .map(|(ni, nj)| (ni as usize, nj as usize))
                    .filter(|&(ni, nj)| ni < N && nj < N)
                    .filter_map(move |(ni, nj)| {
                        if visited[tiles[ni][nj]] {
                            None
                        } else {
                            let mut visited2 = visited.clone();
                            visited2.set(tiles[ni][nj], true);
                            let mut path2 = path.clone();
                            path2.push((ni as u8, nj as u8));
                            Some(State {
                                path: path2,
                                score: score + points[ni][nj],
                                visited: visited2,
                            })
                        }
                    })
                    .for_each(|state| {
                        let (ni, nj) = state.path.last().copied().unwrap();
                        qs[ni as usize][nj as usize].push(StateScore(state));
                    });

                Some(state)
            })
            .max_by_key(|state| state.score);
        if best_state.score < state.as_ref().map_or(0, |s| s.score) {
            best_state = state.unwrap();
        }

        if start.elapsed().as_millis() >= TIME {
            break;
        }
    }

    // (best_state.path, best_state.score)

    let mut path = vec![(si as u8, sj as u8)];
    let mut score = points[si][sj];
    let mut left_visited = best_state.visited.clone();
    left_visited.set(tiles[si][sj], false);
    let path0 = best_state.path.clone();
    for (i, j) in path0.into_iter().skip(1) {
        path.push((i, j));
        score += points[i as usize][j as usize];
        left_visited.set(tiles[i as usize][j as usize], false);
        if let Some(state) = qs[i as usize][j as usize]
            .drain()
            .map(|ss| ss.0)
            .filter(|state| state.score > score)
            .filter(|state| {
                let union_visited = &left_visited & &state.visited;
                union_visited.count_ones() == 0
            })
            .max_by_key(|state| state.score)
        {
            path = state.path;
            score = state.score;
        }
    }

    (path, score)
}

fn main() {
    let (si, sj) = read_tuple!(usize, usize);

    let tiles = read_vec(N, || read_row::<usize>());
    let points = read_vec(N, || read_row::<usize>());

    let m = tiles
        .iter()
        .map(|row| row.citer().max().unwrap())
        .max()
        .unwrap()
        + 1;

    let (path, score) = solve6(si, sj, m, &tiles, &points);

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
