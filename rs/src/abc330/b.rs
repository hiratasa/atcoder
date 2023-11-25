use itertools::Itertools;
use proconio::input;

use easy_ext::ext;

#[ext(IterCopyExt)]
impl<'a, I, T> I
where
    Self: IntoIterator<Item = &'a T>,
    T: 'a + Copy,
{
    fn citer(self) -> std::iter::Copied<Self::IntoIter> {
        self.into_iter().copied()
    }
}

fn main() {
    input! {
        n: usize, l: usize, r: usize,
        a: [usize; n],
    };

    println!(
        "{}",
        a.citer()
            .map(|x| {
                [x, l, r]
                    .citer()
                    .filter(|&z| l <= z && z <= r)
                    .min_by_key(|&z| z.abs_diff(x))
                    .unwrap()
            })
            .join(" ")
    );
}
