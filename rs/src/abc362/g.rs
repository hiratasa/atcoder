fn main() {
    input! {
        s: Chars,
        q: usize,
        t: [Chars; q],
    };

    const M: usize = 300;
    let short = t.iter().positions(|x| x.len() < M).collect::<Vec<_>>();
    let long = t.iter().positions(|x| x.len() >= M).collect::<Vec<_>>();

    let mut ans = vec![0; q];

    // short
    {
        let mut trie = Trie::new();
        for &i in &short {
            trie.reserve(0, &t[i]);
        }

        for i in 0..s.len() {
            trie.add(0, &s[i..min(i + M, s.len())]);
        }

        for i in short {
            ans[i] = trie.get(0, &t[i]);
        }
    }

    // long
    {
        for i in long {
            let tmp = t[i].iter().chain(s.iter()).copied().collect::<Vec<_>>();
            let z = z_algorithm(&tmp);

            ans[i] = z[t[i].len()..].iter().filter(|&&l| l >= t[i].len()).count();
        }
    }

    for x in ans {
        println!("{x}");
    }
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
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Default)]
struct TrieNode {
    num: usize,
    children: [Option<usize>; 26],
}

#[derive(Debug, Clone)]
struct Trie {
    elements: Vec<TrieNode>,
    next: usize,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            elements: vec![TrieNode::default(); 600000],
            next: 1,
        }
    }

    fn reserve(&mut self, idx: usize, t: &[char]) {
        if let Some(&c) = t.get(0) {
            let c = c as usize - 'a' as usize;

            if self.elements[idx].children[c].is_none() {
                self.elements[idx].children[c] = Some(self.next);
                self.next += 1;
            }

            let ch = self.elements[idx].children[c].unwrap();

            self.reserve(ch, &t[1..]);
        }
    }

    fn add(&mut self, idx: usize, t: &[char]) {
        self.elements[idx].num += 1;

        if let Some(&c) = t.get(0) {
            let c = c as usize - 'a' as usize;

            if let Some(ch) = self.elements[idx].children[c] {
                self.add(ch, &t[1..]);
            }
        }
    }

    fn get(&self, idx: usize, t: &[char]) -> usize {
        if let Some(&c) = t.get(0) {
            let c = c as usize - 'a' as usize;

            let ch = self.elements[idx].children[c].unwrap();

            self.get(ch, &t[1..])
        } else {
            self.elements[idx].num
        }
    }
}

#[allow(dead_code)]
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
