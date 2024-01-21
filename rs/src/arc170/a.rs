use std::iter::once;

use itertools::izip;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    if matches!(
        izip!(s.iter().copied(), t.iter().copied())
            .skip_while(|&(x, y)| x == 'B' && y == 'B')
            .next(),
        Some(('A', 'B'))
    ) {
        println!("-1");
        return;
    }

    if matches!(
        izip!(s.iter().copied(), t.iter().copied())
            .rev()
            .skip_while(|&(x, y)| x == 'A' && y == 'A')
            .next(),
        Some(('B', 'A'))
    ) {
        println!("-1");
        return;
    }

    let ans = izip!(s.iter().copied(), t.iter().copied())
        .filter(|&(x, y)| x != y)
        .map(|(_, y)| y)
        .chain(once('#'))
        .scan(0usize, |d, x| match x {
            'A' => {
                *d += 1;
                Some(0)
            }
            'B' => {
                *d = d.saturating_sub(1);
                Some(1)
            }
            '#' => Some(*d),
            _ => unreachable!(),
        })
        .sum::<usize>();

    println!("{ans}");
}
