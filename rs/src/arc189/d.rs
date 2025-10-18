fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let (parents, _) = (0..n).fold((vec![usize::MAX; n], vec![]), |(mut parents, mut st), i| {
        if st.is_empty() {
            st.push(i);
        } else {
            if a[st[st.len() - 1]] <= a[i] {
                while st.len() >= 2 && a[st[st.len() - 2]] <= a[i] {
                    st.pop();
                }
                parents[st[st.len() - 1]] = i;

                if st.len() >= 2 {
                    parents[i] = st[st.len() - 2];
                }
                st.pop();
            } else {
                parents[i] = st[st.len() - 1];
            }
            st.push(i);
        }

        (parents, st)
    });

    let root = (0..n).find(|&i| parents[i] == usize::MAX).unwrap();
    let children = (0..n).fold(vec![vec![]; n], |mut children, i| {
        if parents[i] != usize::MAX {
            children[parents[i]].push(i);
        }
        children
    });

    let mut sums = vec![0; n];
    dfs(&children, root, &a, &mut sums);

    let mut ans = vec![0; n];
    solve(
        &children,
        root,
        usize::MAX,
        &a,
        &sums,
        &mut ans,
        &mut vec![0; n],
    );

    println!("{}", ans.iter().join(" "));
}

fn dfs(children: &[Vec<usize>], v: usize, a: &[usize], sums: &mut [usize]) {
    sums[v] = a[v];
    for &u in &children[v] {
        dfs(children, u, a, sums);

        sums[v] += sums[u];
    }
}

fn solve(
    children: &[Vec<usize>],
    v: usize,
    p: usize,
    a: &[usize],
    sums: &[usize],
    ans: &mut [usize],
    ans1: &mut [usize],
) {
    ans1[v] = if p < a.len() && sums[v] > a[p] {
        ans1[p]
    } else {
        sums[v]
    };
    ans[v] = if children[v].len() == 1 && a[children[v][0]] == a[v] && children[v][0] + 1 == v {
        a[v]
    } else if p < a.len() && sums[v] > a[p] {
        ans1[p]
    } else {
        sums[v]
    };

    for &u in &children[v] {
        solve(children, u, v, a, sums, ans, ans1);
    }
}

fn solve0(a: &[usize], k: usize) -> usize {
    let n = a.len();
    let mut l = k;
    let mut r = k;
    let mut s = a[k];
    loop {
        if l > 0 && a[l - 1] < s {
            s += a[l - 1];
            l -= 1;
        } else if r + 1 < n && a[r + 1] < s {
            s += a[r + 1];
            r += 1;
        } else {
            return s;
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
use rand::{Rng, SeedableRng, rngs::SmallRng};
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
