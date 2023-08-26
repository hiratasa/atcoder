use cargo_snippet::snippet;

use std::iter::*;
use std::{cmp::PartialOrd, collections::VecDeque};

#[snippet("slidemin")]
#[allow(dead_code)]
fn slide_min<'a, T: PartialOrd + Copy>(a: &'a [T], len: usize) -> impl 'a + Iterator<Item = T> {
    assert!(len <= a.len());

    let q = a[..len - 1]
        .iter()
        .enumerate()
        .fold(VecDeque::new(), |mut q, (i, &x)| {
            while matches!(q.back(), Some(&(_, y)) if y >= x) {
                q.pop_back();
            }
            q.push_back((i, x));
            q
        });
    a[len - 1..].iter().enumerate().scan(q, move |q, (i, &x)| {
        while matches!(q.back(), Some(&(_, y)) if y >= x) {
            q.pop_back();
        }
        q.push_back((i + len - 1, x));

        let r = q.front().unwrap().1;

        if q.front().unwrap().0 == i {
            q.pop_front();
        }

        Some(r)
    })
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, Rng, SeedableRng};

    #[test]
    fn test_slide_min() {
        let mut rng = SmallRng::seed_from_u64(42);
        let n = 100;
        let a = repeat_with(|| rng.gen_range(0..1000))
            .take(n)
            .collect::<Vec<_>>();
        let len = 20;

        itertools::assert_equal(
            slide_min(&a, len),
            (0..=n - len).map(|i| *a[i..i + len].iter().min().unwrap()),
        );
    }

    #[test]
    fn test_slide_min_maxlen() {
        let mut rng = SmallRng::seed_from_u64(42);
        let n = 100;
        let a = repeat_with(|| rng.gen_range(0..1000))
            .take(n)
            .collect::<Vec<_>>();
        let len = n;

        itertools::assert_equal(slide_min(&a, len), once(*a.iter().min().unwrap()));
    }
}
