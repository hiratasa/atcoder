fn main() {
    input! {
        q: usize,
        ts: [(usize, Chars); q],
    };

    ts.into_iter()
        .scan(TrieNode::new(), |trie, (t, s)| {
            match t {
                1 => {
                    trie.disable(&s);
                }
                2 => {
                    trie.insert(&s);
                }
                _ => unreachable!(),
            };

            Some(trie.num)
        })
        .for_each(|ans| {
            println!("{ans}");
        })
}

const K: usize = 26;

#[derive(Debug, Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; K],
    num: usize,
    enabled: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            num: 0,
            enabled: true,
        }
    }

    fn insert(&mut self, s: &[char]) -> bool {
        if !self.enabled {
            return false;
        }

        if s.is_empty() {
            self.num += 1;
            return true;
        }

        let c = s[0] as usize - b'a' as usize;
        let success = if let Some(child) = &mut self.children[c] {
            child.insert(&s[1..])
        } else {
            let mut child = Box::new(TrieNode::new());
            let success = child.insert(&s[1..]);
            self.children[c] = Some(child);
            success
        };

        if success {
            self.num += 1;
        }

        success
    }

    fn disable(&mut self, s: &[char]) -> usize {
        if !self.enabled {
            return 0;
        }

        if s.is_empty() {
            self.enabled = false;
            let num = self.num;
            self.num = 0;
            return num;
        }

        let c = s[0] as usize - b'a' as usize;
        if let Some(child) = &mut self.children[c] {
            let num = child.disable(&s[1..]);
            self.num -= num;
            return num;
        } else {
            let mut child = Box::new(TrieNode::new());
            let num = child.disable(&s[1..]);
            assert_eq!(num, 0);
            self.children[c] = Some(child);
            return 0;
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
#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
