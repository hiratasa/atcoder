use cargo_snippet::snippet;

#[snippet("iteratorext")]
trait IteratorExt: Iterator + Sized {
    fn fold_vec<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T);

    fn fold_vec2<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T);

    fn fold_vec3<T, F>(self: Self, init: Vec<T>, f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T;
}

#[snippet("iteratorext")]
impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn fold_vec<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(item);
            v[idx] = t;
            v
        })
    }

    fn fold_vec2<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> (usize, T),
    {
        self.fold(init, |mut v, item| {
            let (idx, t) = f(&v, item);
            v[idx] = t;
            v
        })
    }

    fn fold_vec3<T, F>(self: Self, init: Vec<T>, mut f: F) -> Vec<T>
    where
        F: FnMut(&Vec<T>, Self::Item) -> T,
    {
        self.fold(init, |mut v, item| {
            let t = f(&v, item);
            v.push(t);
            v
        })
    }
}

#[snippet("iteratorpick")]
trait Pick0 {
    type Output;

    fn pick0(self) -> Self::Output;
}

#[snippet("iteratorpick")]
impl<T, T2> Pick0 for (T, T2) {
    type Output = T;

    fn pick0(self) -> Self::Output {
        self.0
    }
}

#[snippet("iteratorpick")]
impl<T, T2, T3> Pick0 for (T, T2, T3) {
    type Output = T;

    fn pick0(self) -> Self::Output {
        self.0
    }
}

#[snippet("iteratorpick")]
trait IteratorPick0Ext<T>: std::iter::Iterator<Item = T> + std::marker::Sized
where
    T: Pick0,
{
    fn pick0(self) -> std::iter::Map<Self, fn(T) -> T::Output> {
        self.map(Pick0::pick0)
    }
}

#[snippet("iteratorpick")]
impl<T, I> IteratorPick0Ext<T> for I
where
    I: std::iter::Iterator<Item = T>,
    T: Pick0,
{
}

#[snippet("iteratorpick")]
trait Pick1 {
    type Output;

    fn pick1(self) -> Self::Output;
}

#[snippet("iteratorpick")]
impl<T, T2> Pick1 for (T, T2) {
    type Output = T2;

    fn pick1(self) -> Self::Output {
        self.1
    }
}

#[snippet("iteratorpick")]
impl<T, T2, T3> Pick1 for (T, T2, T3) {
    type Output = T2;

    fn pick1(self) -> Self::Output {
        self.1
    }
}

#[snippet("iteratorpick")]
trait IteratorPick1Ext<T>: std::iter::Iterator<Item = T> + std::marker::Sized
where
    T: Pick1,
{
    fn pick1(self) -> std::iter::Map<Self, fn(T) -> T::Output> {
        self.map(Pick1::pick1)
    }
}

#[snippet("iteratorpick")]
impl<T, I> IteratorPick1Ext<T> for I
where
    I: std::iter::Iterator<Item = T>,
    T: Pick1,
{
}

#[test]
fn test_fold_vec() {
    let t = vec![3, 0, 2];

    let actual = t
        .iter()
        .copied()
        .enumerate()
        .fold_vec(vec![None; 4], |(i, tt)| (tt, Some(i)));

    assert_eq!(actual, vec![Some(1), None, Some(2), Some(0)]);
}
