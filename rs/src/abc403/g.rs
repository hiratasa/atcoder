fn main() {
    input! {
        q: usize,
        ys: [usize; q],
    };

    ys.into_iter()
        .scan(BinaryTrie::new(), |trie, y| {
            let last = trie.odd_sum;
            trie.add((y + last) % 1000000000 + 1, 30);
            Some(trie.odd_sum)
        })
        .for_each(|ans| {
            println!("{ans}");
        });
}

#[derive(Debug, Clone)]
struct BinaryTrie {
    children: [Option<Box<BinaryTrie>>; 2],
    odd_sum: usize,
    even_sum: usize,
    num: usize,
}

impl BinaryTrie {
    fn new() -> BinaryTrie {
        BinaryTrie {
            children: [None, None],
            odd_sum: 0,
            even_sum: 0,
            num: 0,
        }
    }

    fn add(&mut self, x: usize, idx: usize) {
        if idx == usize::MAX {
            self.num += 1;
            if self.num % 2 == 0 {
                self.even_sum += x;
            } else {
                self.odd_sum += x;
            }
            return;
        }

        let c = (x >> idx) & 1;
        if let Some(ch) = self.children[c].as_mut() {
            ch.add(x, idx.wrapping_sub(1));
        } else {
            let mut ch = BinaryTrie::new();
            ch.add(x, idx.wrapping_sub(1));
            self.children[c] = Some(Box::new(ch));
        }

        let (mut odd_sum, mut even_sum, num) = if let Some(ch) = self.children[0].as_ref() {
            (ch.odd_sum, ch.even_sum, ch.num)
        } else {
            (0, 0, 0)
        };

        if let Some(ch) = self.children[1].as_ref() {
            if num % 2 == 0 {
                odd_sum += ch.odd_sum;
                even_sum += ch.even_sum;
            } else {
                odd_sum += ch.even_sum;
                even_sum += ch.odd_sum;
            }
        }

        self.odd_sum = odd_sum;
        self.even_sum = even_sum;
        self.num += 1;
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
