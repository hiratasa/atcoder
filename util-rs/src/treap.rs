trait Monoid {
    type Item: Clone;

    fn id() -> Self::Item;
    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item;
}

trait Operator<T>: Monoid {
    fn apply(op: &Self::Item, v: &T) -> T;
}

// 通常のtreapとimplicit treap兼用
struct TreapNode<K, M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    left: Option<Box<TreapNode<K, M, Op>>>,
    right: Option<Box<TreapNode<K, M, Op>>>,
    size: usize,
    priority: usize,
    key: K,
    reversed: bool,
    lazy: Op::Item,
    // lazy適用後の値
    acc: M::Item,
    // lazy適用後の値
    value: M::Item,
}

impl<K, M, Op> TreapNode<K, M, Op>
where
    K: Ord,
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new(priority: usize, key: K, value: M::Item) -> TreapNode<K, M, Op> {
        TreapNode {
            left: None,
            right: None,
            size: 1,
            reversed: false,
            priority,
            key,
            lazy: Op::id(),
            acc: value.clone(),
            value,
        }
    }

    fn fix(&mut self) {
        self.size = 1
            + self.left.as_deref().map_or(0, |l| l.size)
            + self.right.as_deref().map_or(0, |r| r.size);
        self.acc = M::op(
            &M::op(
                &Op::apply(
                    &self.lazy,
                    self.left.as_deref().map_or(&M::id(), |l| &l.acc),
                ),
                &self.value,
            ),
            &Op::apply(
                &self.lazy,
                self.right.as_deref().map_or(&M::id(), |l| &l.acc),
            ),
        );
    }

    fn push(&mut self) {
        if let Some(t) = self.left.as_deref_mut() {
            t.lazy = Op::op(&self.lazy, &t.lazy);
            t.acc = Op::apply(&self.lazy, &t.acc);
            t.value = Op::apply(&self.lazy, &t.value);

            if self.reversed {
                t.reversed ^= true;
            }
        }
        if let Some(t) = self.right.as_deref_mut() {
            t.lazy = Op::op(&self.lazy, &t.lazy);
            t.acc = Op::apply(&self.lazy, &t.acc);
            t.value = Op::apply(&self.lazy, &t.value);

            if self.reversed {
                t.reversed ^= true;
            }
        }

        if self.reversed {
            std::mem::swap(&mut self.left, &mut self.right);
        }

        self.lazy = Op::id();
        self.reversed = false;
    }

    // selfの要素がrhsの要素より小さいときのみ使用可
    fn merge(lhs: Option<Box<Self>>, rhs: Option<Box<Self>>) -> Option<Box<Self>> {
        let mut lhs = if let Some(lhs) = lhs {
            lhs
        } else {
            return rhs;
        };

        let mut rhs = if let Some(rhs) = rhs {
            rhs
        } else {
            return Some(lhs);
        };

        if lhs.priority < rhs.priority {
            rhs.push();
            rhs.left = TreapNode::merge(Some(lhs), rhs.left);
            rhs.fix();
            Some(rhs)
        } else {
            lhs.push();
            lhs.right = TreapNode::merge(lhs.right, Some(rhs));
            lhs.fix();
            Some(lhs)
        }
    }

    // x以下とxより大きい部分に分ける
    fn split_upper_bound(t: Option<Box<Self>>, x: &K) -> (Option<Box<Self>>, Option<Box<Self>>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        t.push();
        if *x < t.key {
            let (l, r) = TreapNode::split_upper_bound(t.left, x);
            t.left = r;
            t.fix();
            (l, Some(t))
        } else {
            let (l, r) = TreapNode::split_upper_bound(t.right, x);
            t.right = l;
            t.fix();
            (Some(t), r)
        }
    }

    // x未満とx以上の部分に分ける
    fn split_lower_bound(t: Option<Box<Self>>, x: &K) -> (Option<Box<Self>>, Option<Box<Self>>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        t.push();
        if *x <= t.key {
            let (l, r) = TreapNode::split_lower_bound(t.left, x);
            t.left = r;
            t.fix();
            (l, Some(t))
        } else {
            let (l, r) = TreapNode::split_lower_bound(t.right, x);
            t.right = l;
            t.fix();
            (Some(t), r)
        }
    }

    // nth番目以降を分離
    fn split_at(t: Option<Box<Self>>, nth: usize) -> (Option<Box<Self>>, Option<Box<Self>>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        t.push();
        let left_size = TreapNode::size(&t.left);

        if nth <= left_size {
            let (l, r) = TreapNode::split_at(t.left, nth);
            t.left = r;
            t.fix();
            (l, Some(t))
        } else {
            let (l, r) = TreapNode::split_at(t.right, nth - (left_size + 1));
            t.right = l;
            t.fix();
            (Some(t), r)
        }
    }

    fn insert(t: Option<Box<Self>>, priority: usize, key: K, value: M::Item) -> Box<Self> {
        let mut t = if let Some(t) = t {
            t
        } else {
            return Box::new(TreapNode::new(priority, key, value));
        };

        if t.priority < priority {
            let (l, r) = TreapNode::split_upper_bound(Some(t), &key);
            let mut v = Box::new(TreapNode::new(priority, key, value));
            v.left = l;
            v.right = r;
            v.fix();
            v
        } else if key < t.key {
            t.push();
            t.left = Some(TreapNode::insert(t.left, priority, key, value));
            t.fix();
            t
        } else {
            t.push();
            t.right = Some(TreapNode::insert(t.right, priority, key, value));
            t.fix();
            t
        }
    }

    // keyの順序を無視してnth番目に挿入する
    // Implicit treap用
    fn insert_at(
        t: Option<Box<Self>>,
        nth: usize,
        priority: usize,
        key: K,
        value: M::Item,
    ) -> Box<Self> {
        let mut t = if let Some(t) = t {
            t
        } else {
            return Box::new(TreapNode::new(priority, key, value));
        };

        t.push();
        let left_size = TreapNode::size(&t.left);
        if t.priority < priority {
            let (l, r) = TreapNode::split_at(Some(t), nth);
            let mut v = Box::new(TreapNode::new(priority, key, value));
            v.left = l;
            v.right = r;
            v.fix();
            v
        } else if nth <= left_size {
            t.left = Some(TreapNode::insert_at(t.left, nth, priority, key, value));
            t.fix();
            t
        } else {
            t.right = Some(TreapNode::insert_at(
                t.right,
                nth - (left_size + 1),
                priority,
                key,
                value,
            ));
            t.fix();
            t
        }
    }

    // 指定されたkeyを持つ要素が複数あったら一つだけ消す
    fn remove(t: Option<Box<Self>>, key: &K) -> Option<Box<Self>> {
        let mut t = if let Some(t) = t {
            t
        } else {
            return None;
        };

        t.push();

        use std::cmp::Ordering;
        match t.key.cmp(&key) {
            Ordering::Less => {
                t.right = TreapNode::remove(t.right, key);
                t.fix();
                Some(t)
            }
            Ordering::Equal => TreapNode::merge(t.left, t.right),
            Ordering::Greater => {
                t.left = TreapNode::remove(t.left, key);
                t.fix();
                Some(t)
            }
        }
    }

    // nth番目の要素を消す
    fn remove_at(t: Option<Box<Self>>, nth: usize) -> (Option<Box<Self>>, Option<(K, M::Item)>) {
        let mut t = if let Some(t) = t {
            t
        } else {
            return (None, None);
        };

        if nth >= t.size {
            return (Some(t), None);
        }

        t.push();

        let (tl, v) = TreapNode::remove_at(t.left, nth);
        t.left = tl;
        if let Some(v) = v {
            t.fix();
            return (Some(t), Some(v));
        }

        let left_size = TreapNode::size(&t.left);

        if left_size == nth {
            let ret = (t.key, t.value);
            return (TreapNode::merge(t.left, t.right), Some(ret));
        }

        let (tr, v) = TreapNode::remove_at(t.right, nth - (left_size + 1));
        t.right = tr;
        t.fix();

        (Some(t), v)
    }

    fn size(t: &Option<Box<Self>>) -> usize {
        t.as_deref().map_or(0, |t| t.size)
    }

    fn at(t: &mut Option<Box<Self>>, nth: usize) -> Option<(&K, &M::Item)> {
        let t = t.as_deref_mut()?;

        if nth >= t.size {
            return None;
        }

        t.push();

        let left_size = TreapNode::size(&t.left);
        if let Some(x) = TreapNode::at(&mut t.left, nth) {
            return Some(x);
        }

        if left_size == nth {
            return Some((&t.key, &t.value));
        }

        TreapNode::at(&mut t.right, nth - (left_size + 1))
    }

    // TODO: implement without Box
    fn iter<'a>(
        t: &'a mut Option<Box<Self>>,
    ) -> Box<dyn 'a + Iterator<Item = (&'a K, &'a M::Item)>> {
        Box::new(t.iter_mut().flat_map(|t| {
            t.push();

            TreapNode::iter(&mut t.left)
                .chain(std::iter::once((&t.key, &t.value)))
                .chain(TreapNode::iter(&mut t.right))
        }))
    }
}

struct Treap<K, M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    // To share when split, use Rc.
    rng: std::rc::Rc<std::cell::RefCell<rand::rngs::SmallRng>>,
    root: Option<Box<TreapNode<K, M, Op>>>,
}

#[allow(dead_code)]
impl<K, M, Op> Treap<K, M, Op>
where
    K: Ord + Copy,
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new() -> Treap<K, M, Op> {
        use rand::SeedableRng;

        Treap {
            rng: std::rc::Rc::new(std::cell::RefCell::new(rand::rngs::SmallRng::from_entropy())),
            root: None,
        }
    }

    fn merge(&mut self, rhs: Treap<K, M, Op>) {
        self.root = TreapNode::merge(std::mem::replace(&mut self.root, None), rhs.root);
    }

    fn split(&mut self, x: &K) -> Treap<K, M, Op> {
        let (l, r) = TreapNode::split_upper_bound(std::mem::replace(&mut self.root, None), x);
        self.root = l;

        Treap {
            rng: std::rc::Rc::clone(&self.rng),
            root: r,
        }
    }

    fn insert(&mut self, key: K) {
        self.insert_with_value(key, M::id());
    }

    fn insert_with_value(&mut self, key: K, value: M::Item) {
        use rand::Rng;

        self.root = Some(TreapNode::insert(
            std::mem::replace(&mut self.root, None),
            self.rng.borrow_mut().gen(),
            key,
            value,
        ));
    }

    fn remove(&mut self, key: &K) {
        self.root = TreapNode::remove(std::mem::replace(&mut self.root, None), key);
    }

    fn remove_at(&mut self, nth: usize) -> Option<(K, M::Item)> {
        let (t, v) = TreapNode::remove_at(std::mem::replace(&mut self.root, None), nth);
        self.root = t;
        v
    }

    fn size(&self) -> usize {
        TreapNode::size(&self.root)
    }

    // 内部的に変更走るのでmut
    fn at(&mut self, nth: usize) -> Option<(&K, &M::Item)> {
        TreapNode::at(&mut self.root, nth)
    }

    // 内部的に変更走るのでmut
    fn query(&mut self, lower: &K, upper: &K) -> M::Item {
        assert!(*lower <= *upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_lower_bound(root, lower);
        let (t1, t2) = TreapNode::split_lower_bound(t12, upper);

        let v = t1.as_deref().map_or(M::id(), |t1| t1.acc.clone());

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));

        v
    }

    fn update(&mut self, lower: &K, upper: &K, x: Op::Item) {
        assert!(*lower <= *upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_lower_bound(root, lower);
        let (mut t1, t2) = TreapNode::split_lower_bound(t12, upper);

        if let Some(t1) = t1.as_deref_mut() {
            t1.acc = Op::apply(&x, &t1.acc);
            t1.value = Op::apply(&x, &t1.value);
            t1.lazy = x;
        }

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));
    }

    fn iter(&mut self) -> impl Iterator<Item = (&K, &M::Item)> {
        TreapNode::iter(&mut self.root)
    }
}

// keyを保持せずにindexで操作するtreap
struct ImplicitTreap<M, Op>
where
    M: Monoid,
    Op: Monoid,
{
    // To share when split, use Rc.
    rng: std::rc::Rc<std::cell::RefCell<rand::rngs::SmallRng>>,
    root: Option<Box<TreapNode<(), M, Op>>>,
}

#[allow(dead_code)]
impl<M, Op> ImplicitTreap<M, Op>
where
    M: Monoid,
    Op: Monoid + Operator<M::Item>,
{
    fn new() -> ImplicitTreap<M, Op> {
        use rand::SeedableRng;

        ImplicitTreap {
            rng: std::rc::Rc::new(std::cell::RefCell::new(rand::rngs::SmallRng::from_entropy())),
            root: None,
        }
    }

    fn merge(&mut self, rhs: ImplicitTreap<M, Op>) {
        self.root = TreapNode::merge(std::mem::replace(&mut self.root, None), rhs.root);
    }

    fn split_at(&mut self, nth: usize) -> ImplicitTreap<M, Op> {
        let (l, r) = TreapNode::split_at(std::mem::replace(&mut self.root, None), nth);
        self.root = l;

        ImplicitTreap {
            rng: std::rc::Rc::clone(&self.rng),
            root: r,
        }
    }

    fn insert_at(&mut self, nth: usize, value: M::Item) {
        use rand::Rng;

        self.root = Some(TreapNode::insert_at(
            std::mem::replace(&mut self.root, None),
            nth,
            self.rng.borrow_mut().gen(),
            (),
            value,
        ));
    }

    fn remove_at(&mut self, nth: usize) -> Option<M::Item> {
        let (t, v) = TreapNode::remove_at(std::mem::replace(&mut self.root, None), nth);
        self.root = t;
        v.map(|(_, v)| v)
    }

    fn size(&self) -> usize {
        TreapNode::size(&self.root)
    }

    // 内部的に変更走るのでmut
    fn at(&mut self, nth: usize) -> Option<&M::Item> {
        TreapNode::at(&mut self.root, nth).map(|(_, v)| v)
    }

    // 内部的に変更走るのでmut
    fn query(&mut self, lower: usize, upper: usize) -> M::Item {
        assert!(lower <= upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_at(root, lower);
        let (t1, t2) = TreapNode::split_at(t12, upper - lower);

        let v = t1.as_deref().map_or(M::id(), |t1| t1.acc.clone());

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));

        v
    }

    fn update(&mut self, lower: usize, upper: usize, x: Op::Item) {
        assert!(lower <= upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_at(root, lower);
        let (mut t1, t2) = TreapNode::split_at(t12, upper - lower);

        if let Some(t1) = t1.as_deref_mut() {
            t1.acc = Op::apply(&x, &t1.acc);
            t1.value = Op::apply(&x, &t1.value);
            t1.lazy = x;
        }

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));
    }

    fn reverse(&mut self, lower: usize, upper: usize) {
        assert!(lower <= upper);

        let root = std::mem::replace(&mut self.root, None);

        let (t0, t12) = TreapNode::split_at(root, lower);
        let (mut t1, t2) = TreapNode::split_at(t12, upper - lower);

        if let Some(t1) = t1.as_deref_mut() {
            t1.reversed ^= true;
        }

        self.root = TreapNode::merge(t0, TreapNode::merge(t1, t2));
    }
}

struct NullMonoid;

impl Monoid for NullMonoid {
    type Item = ();

    fn id() -> Self::Item {
        ()
    }

    fn op(_lhs: &Self::Item, _rhs: &Self::Item) -> Self::Item {
        ()
    }
}

impl<T> Operator<T> for NullMonoid
where
    T: Clone,
{
    fn apply(_: &Self::Item, v: &T) -> T {
        v.clone()
    }
}

struct Sum;

impl Monoid for Sum {
    type Item = usize;

    fn id() -> Self::Item {
        0
    }

    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        lhs + rhs
    }
}

struct Minimum;

impl Monoid for Minimum {
    type Item = usize;

    fn id() -> Self::Item {
        std::usize::MAX
    }

    fn op(lhs: &Self::Item, rhs: &Self::Item) -> Self::Item {
        std::cmp::min(*lhs, *rhs)
    }
}

impl Operator<usize> for Sum {
    fn apply(x: &usize, y: &usize) -> usize {
        x + y
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn without_lazy() {
        let mut t = Treap::<usize, Sum, NullMonoid>::new();

        assert_eq!(t.size(), 0);

        t.insert_with_value(3, 10);
        assert_eq!(t.size(), 1);
        assert_eq!(t.at(0), Some((&3, &10)));
        assert_eq!(t.query(&2, &5), 10);
        assert_eq!(t.query(&6, &10), 0);
        assert_eq!(t.at(1), None);

        t.insert_with_value(5, 42);
        assert_eq!(t.size(), 2);
        assert_eq!(t.at(0), Some((&3, &10)));
        assert_eq!(t.at(1), Some((&5, &42)));
        assert_eq!(t.query(&2, &5), 10);
        assert_eq!(t.query(&2, &6), 52);
        assert_eq!(t.query(&6, &10), 0);
        assert_eq!(t.at(2), None);

        t.insert_with_value(1, 100);
        assert_eq!(t.size(), 3);
        assert_eq!(t.at(0), Some((&1, &100)));
        assert_eq!(t.at(1), Some((&3, &10)));
        assert_eq!(t.at(2), Some((&5, &42)));
        assert_eq!(t.query(&1, &6), 152);
        assert_eq!(t.query(&1, &2), 100);
        assert_eq!(t.query(&1, &3), 100);
        assert_eq!(t.query(&1, &4), 110);
        assert_eq!(t.query(&2, &5), 10);
        assert_eq!(t.query(&2, &6), 52);
        assert_eq!(t.query(&6, &10), 0);
        assert_eq!(t.at(3), None);

        t.remove(&3);
        assert_eq!(t.size(), 2);
        assert_eq!(t.at(0), Some((&1, &100)));
        assert_eq!(t.at(1), Some((&5, &42)));
        assert_eq!(t.query(&1, &6), 142);
        assert_eq!(t.query(&1, &4), 100);
        assert_eq!(t.query(&2, &5), 0);
        assert_eq!(t.query(&2, &6), 42);
        assert_eq!(t.query(&6, &10), 0);
        assert_eq!(t.at(2), None);
    }

    #[test]
    fn wiht_lazy() {
        let mut t = Treap::<usize, Minimum, Sum>::new();

        t.insert_with_value(3, 10);
        t.insert_with_value(5, 42);
        t.insert_with_value(1, 100);

        assert_eq!(t.query(&0, &8), 10);
        assert_eq!(t.query(&1, &3), 100);
        assert_eq!(t.query(&3, &6), 10);
        assert_eq!(t.query(&4, &6), 42);

        t.update(&3, &7, 200);
        assert_eq!(t.query(&0, &8), 100);
        assert_eq!(t.query(&1, &3), 100);
        assert_eq!(t.query(&3, &6), 210);
        assert_eq!(t.query(&4, &6), 242);

        t.remove(&3);
        assert_eq!(t.query(&0, &8), 100);
        assert_eq!(t.query(&1, &3), 100);
        assert_eq!(t.query(&3, &6), 242);
        assert_eq!(t.query(&4, &6), 242);
    }
}
