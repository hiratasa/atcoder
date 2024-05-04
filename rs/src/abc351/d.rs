use itertools::{iproduct, Itertools};
use proconio::{input, marker::Chars};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        h: usize, w: usize,
        mut s: [Chars; h],
    };

    iproduct!(0..h, 0..w).for_each(|(i, j)| {
        if s[i][j] != '#'
            && [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
                .filter(|&(ni, nj)| ni < h && nj < w)
                .any(|(ni, nj)| s[ni][nj] == '#')
        {
            s[i][j] = 'o';
        }
    });

    let mut uf = iproduct!(0..h, 0..w).filter(|&(i, j)| s[i][j] == '.').fold(
        UnionFind::new(h * w),
        |uf, (i, j)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
                .filter(|&(ni, nj)| ni < h && nj < w)
                .filter(|&(ni, nj)| s[ni][nj] == '.')
                .fold(uf, |mut uf, (ni, nj)| {
                    uf.unite(i * w + j, ni * w + nj);
                    uf
                })
        },
    );

    let mut nums = iproduct!(0..h, 0..w)
        .filter_map(|(i, j)| {
            if uf.root(i * w + j) == i * w + j {
                Some((i * w + j, uf.size(i * w + j)))
            } else {
                None
            }
        })
        .collect::<FxHashMap<_, _>>();

    iproduct!(0..h, 0..w)
        .filter(|&(i, j)| s[i][j] == 'o')
        .for_each(|(i, j)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(di, dj)| Some((i.checked_add_signed(di)?, j.checked_add_signed(dj)?)))
                .filter(|&(ni, nj)| ni < h && nj < w)
                .filter(|&(ni, nj)| s[ni][nj] == '.')
                .map(|(ni, nj)| uf.root(ni * w + nj))
                .unique()
                .for_each(|idx| {
                    *nums.get_mut(&idx).unwrap() += 1;
                });
        });

    let ans = nums.values().max().unwrap();

    println!("{ans}");
}

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}
struct UnionFind {
    g: Vec<UnionFindNode>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        use UnionFindNode::*;
        UnionFind {
            g: (0..n).map(|_| Root { size: 1 }).collect(),
        }
    }
    fn root(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let p = match self.g[v] {
            Root { size: _ } => return v,
            Child { parent: p } => p,
        };
        let r = self.root(p);
        self.g[v] = Child { parent: r };
        r
    }
    fn unite(&mut self, v: usize, u: usize) -> bool {
        use UnionFindNode::*;
        let rv = self.root(v);
        let ru = self.root(u);
        if rv == ru {
            return false;
        }
        let size_rv = self.size(rv);
        let size_ru = self.size(ru);
        let (rsmall, rlarge) = if size_rv < size_ru {
            (rv, ru)
        } else {
            (ru, rv)
        };
        self.g[rsmall] = Child { parent: rlarge };
        self.g[rlarge] = Root {
            size: size_rv + size_ru,
        };
        true
    }
    fn same(&mut self, v: usize, u: usize) -> bool {
        self.root(v) == self.root(u)
    }
    fn size(&mut self, v: usize) -> usize {
        use UnionFindNode::*;
        let rv = self.root(v);
        match self.g[rv] {
            Root { size } => size,
            Child { parent: _ } => unreachable!(),
        }
    }
}
