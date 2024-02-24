use std::{cmp::max, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    };

    let cap = n.next_power_of_two();
    let mut st = vec![BinaryHeap::new(); 2 * cap];
    for i in 0..n {
        st[cap + i - 1].push((a[i], 0));
    }

    let mut enabled = vec![true; q + 1];
    for i_query in 1..=q {
        input! {
            ty: usize,
        };

        match ty {
            1 => {
                input! { l: usize, r: usize, x: usize };
                let l = l - 1;

                update(&mut st, 0, 0, cap, l, r, x, i_query);
            }
            2 => {
                input! { i: usize };
                enabled[i] = false;
            }
            3 => {
                input! { i: usize };
                let i = i - 1;

                let ans = get(&mut st, 0, 0, cap, i, &enabled);

                println!("{ans}");
            }
            _ => unreachable!(),
        }
    }
}

fn update(
    st: &mut [BinaryHeap<(usize, usize)>],
    idx: usize,
    begin: usize,
    end: usize,
    l: usize,
    r: usize,
    x: usize,
    i_query: usize,
) {
    if r <= begin || end <= l {
        // no overlap
        return;
    }

    if l <= begin && end <= r {
        // contains
        st[idx].push((x, i_query));
        return;
    }

    let n = st.len();
    let idx_left = 2 * idx + 1;
    let idx_right = 2 * idx + 2;
    let mid = (begin + end) / 2;
    if idx_left < n {
        update(st, idx_left, begin, mid, l, r, x, i_query);
    }
    if idx_right < n {
        update(st, idx_right, mid, end, l, r, x, i_query);
    }
}

fn get(
    st: &mut [BinaryHeap<(usize, usize)>],
    idx: usize,
    begin: usize,
    end: usize,
    i: usize,
    enabled: &[bool],
) -> usize {
    while matches!(st[idx].peek(), Some(&(_, i_query)) if !enabled[i_query]) {
        st[idx].pop();
    }

    let x0 = st[idx].peek().map_or(0, |&(x, _)| x);

    if begin + 1 == end && begin == i {
        return st[idx].peek().unwrap().0;
    }

    assert!(begin <= i && i < end);

    let n = st.len();
    let idx_left = 2 * idx + 1;
    let idx_right = 2 * idx + 2;
    assert!(idx_right < n);

    let mid = (begin + end) / 2;
    let x = if i < mid {
        get(st, idx_left, begin, mid, i, enabled)
    } else {
        get(st, idx_right, mid, end, i, enabled)
    };

    max(x0, x)
}
