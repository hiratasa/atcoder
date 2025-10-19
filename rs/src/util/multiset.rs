use cargo_snippet::snippet;

#[snippet("multiset")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct BTreeMultiSet<T>
where
    T: Ord,
{
    length: usize,
    m: std::collections::BTreeMap<T, usize>,
}

#[snippet("multiset")]
#[allow(dead_code)]
impl<T> BTreeMultiSet<T>
where
    T: Ord,
{
    fn new() -> BTreeMultiSet<T> {
        Self {
            length: 0,
            m: std::collections::BTreeMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.m.is_empty()
    }

    fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        self.m.contains_key(value)
    }

    fn len(&self) -> usize {
        self.length
    }

    fn count<Q>(&self, value: &Q) -> usize
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        *self.m.get(value).unwrap_or(&0)
    }

    fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        self.m.get_key_value(value).map(|(k, _v)| k)
    }

    fn first(&self) -> Option<&T> {
        self.m.iter().next().map(|(k, _v)| k)
    }

    fn last(&self) -> Option<&T> {
        self.m.iter().next_back().map(|(k, _v)| k)
    }

    fn clear(&mut self) {
        self.length = 0;
        self.m.clear();
    }

    fn insert(&mut self, value: T) {
        self.length += 1;
        *self.m.entry(value).or_insert(0) += 1;
    }

    fn append(&mut self, other: &mut BTreeMultiSet<T>) {
        self.length += other.length;
        other.length = 0;
        std::mem::take(&mut other.m).into_iter().for_each(|(k, v)| {
            *self.m.entry(k).or_insert(0) += v;
        });
    }

    fn remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: Ord + ?Sized,
        T: std::borrow::Borrow<Q>,
    {
        if let Some(c) = self.m.get_mut(value) {
            self.length -= 1;
            *c -= 1;
            if *c == 0 {
                self.m.remove(value);
            }
            true
        } else {
            false
        }
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.m.iter().flat_map(|(v, m)| (0..*m).map(move |_| v))
    }

    fn range<'a, K, R>(&'a self, range: R) -> impl 'a + DoubleEndedIterator<Item = &'a T>
    where
        K: Ord + ?Sized,
        R: std::ops::RangeBounds<K>,
        T: std::borrow::Borrow<K>,
    {
        self.m
            .range(range)
            .flat_map(|(v, m)| (0..*m).map(move |_| v))
    }
}

#[snippet("multiset")]
#[allow(dead_code)]
impl<T> BTreeMultiSet<T>
where
    T: Ord + Clone,
{
    fn pop_first(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let first = self.first().unwrap().clone();
        self.remove(&first);
        Some(first)
    }

    fn pop_last(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let last = self.last().unwrap().clone();
        self.remove(&last);
        Some(last)
    }
}

#[snippet("multiset")]
#[allow(dead_code)]
impl<'a, T> Extend<&'a T> for BTreeMultiSet<T>
where
    T: 'a + Ord + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        for value in iter {
            self.insert(value.clone());
        }
    }
}

#[snippet("multiset")]
#[allow(dead_code)]
impl<T> Extend<T> for BTreeMultiSet<T>
where
    T: Ord,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            self.insert(value);
        }
    }
}

#[snippet("multiset")]
#[allow(dead_code)]
impl<T> std::iter::FromIterator<T> for BTreeMultiSet<T>
where
    T: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}
