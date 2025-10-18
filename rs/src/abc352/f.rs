use itertools::Itertools;
use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, usize); m],
    };

    let mut uf = abc
        .into_iter()
        .fold(UnionFind::new(n), |mut uf, (a, b, c)| {
            uf.unite(a, b, c);
            uf
        });

    let (blocks, ones) = (0..n).fold(
        (FxHashMap::default(), vec![]),
        |(mut blocks, mut ones), i| {
            let r = uf.root(i);
            let l = uf.labels[i];
            let s = uf.size(i);

            if s == 1 {
                ones.push(i);
            } else {
                blocks.entry(r).or_insert(vec![]).push((l, i));
            }

            (blocks, ones)
        },
    );

    let blocks = blocks.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    let mut order = vec![];
    check(
        n,
        &blocks,
        &ones,
        &mut vec![None; n],
        &mut vec![false; blocks.len()],
        0,
        &mut order,
    );

    if ones.len() > 1 {
        for i in ones {
            order[i] = None;
        }
    }

    println!(
        "{}",
        order
            .into_iter()
            .map(|x| { if let Some(x) = x { (x as i64) + 1 } else { -1 } })
            .join(" ")
    );
}

fn check(
    n: usize,
    blocks: &[Vec<(usize, usize)>],
    ones: &[usize],
    perm: &mut [Option<usize>],
    used_blocks: &mut [bool],
    idx: usize,
    order: &mut Vec<Option<usize>>,
) {
    if idx == n {
        let is_first = if order.is_empty() {
            order.resize(n, None);
            true
        } else {
            false
        };

        for i in 0..n {
            let j = perm[i].unwrap();
            if is_first {
                order[j] = Some(i);
            } else if order[j] == Some(i) {
                // NOP
            } else {
                order[j] = None;
            }
        }

        return;
    }

    if !perm[idx].is_none() {
        check(n, blocks, ones, perm, used_blocks, idx + 1, order);
        return;
    }

    for i in 0..blocks.len() {
        if used_blocks[i] {
            continue;
        }

        used_blocks[i] = true;

        if let Some(mut perm2) =
            blocks[i]
                .iter()
                .copied()
                .try_fold(perm.to_vec(), |mut perm2, (x, y)| {
                    if idx + x >= n {
                        None
                    } else if perm2[idx + x].is_some() {
                        None
                    } else {
                        perm2[idx + x] = Some(y);
                        Some(perm2)
                    }
                })
        {
            check(n, blocks, ones, &mut perm2, used_blocks, idx + 1, order);
        }

        used_blocks[i] = false;
    }

    if let Some(&x) = ones.last() {
        perm[idx] = Some(x);

        check(
            n,
            blocks,
            &ones[..ones.len() - 1],
            perm,
            used_blocks,
            idx + 1,
            order,
        );

        perm[idx] = None;
    }
}

#[derive(Clone, Copy, Debug)]
enum UnionFindNode {
    Root { size: usize },
    Child { parent: usize },
}
struct UnionFind {
    g: Vec<UnionFindNode>,
    labels: Vec<usize>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        use UnionFindNode::*;
        UnionFind {
            g: (0..n).map(|_| Root { size: 1 }).collect(),
            labels: vec![0; n],
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
        self.labels[v] = self.labels[p] + self.labels[v];
        r
    }
    fn unite(&mut self, v: usize, u: usize, w: usize) -> bool {
        use UnionFindNode::*;
        let rv = self.root(v);
        let ru = self.root(u);
        if rv == ru {
            return false;
        }

        let lv = self.labels[v];
        let lu = self.labels[u];

        let size = self.size(rv) + self.size(ru);

        if lv > w + lu {
            // rv側が親
            self.g[ru] = Child { parent: rv };
            self.g[rv] = Root { size };
            self.labels[ru] = lv - (w + lu);
        } else {
            // ru側が親
            self.g[rv] = Child { parent: ru };
            self.g[ru] = Root { size };
            self.labels[rv] = (w + lu) - lv;
        }

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
