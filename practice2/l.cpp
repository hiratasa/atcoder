#include <bits/stdc++.h>

#include <atcoder/lazysegtree>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

// M (type, id, op): Monoid of value
// Op (type, id, op, apply): Monoid of lazy operation
template <typename M, typename Op, typename F>
class SegmentTree {
   public:
    using value_type = typename M::type;
    using operator_type = typename Op::type;

    explicit SegmentTree(int64_t n)
            : f(F{}),
              n(n),
              height(ceil(log2(n)) + 1),
              cap(pow(2, ceil(log2(n)))),
              values(2 * cap - 1, M::id()),
              lazy(2 * cap - 1, Op::id()) {}

    void set(int64_t pos, value_type v) {
        int64_t idx = cap - 1 + pos;

        for (auto i : irange(1L, height) | reversed) {
            auto parent_idx = ((idx + 1) >> i) - 1;

            auto left_idx = 2 * (parent_idx + 1) - 1;
            auto right_idx = 2 * (parent_idx + 1);

            lazy[left_idx] = Op::op(lazy[parent_idx], lazy[left_idx]);
            lazy[right_idx] = Op::op(lazy[parent_idx], lazy[right_idx]);
            lazy[parent_idx] = Op::id();

            fix_value(parent_idx);
        }

        values[idx] = v;
        lazy[idx] = Op::id();

        while (idx > 0) {
            idx = (idx - 1) / 2;
            fix_value(idx);
        }
    }

    void update(int64_t a, int64_t b, operator_type p) {
        update(a, b, p, 0, 0, cap);
    }

    void update(int64_t a, int64_t b, operator_type p, int64_t idx, int64_t l,
                int64_t r) {
        if (a >= r || b <= l) {
            // no overlap
            return;
        }

        if (a <= l && r <= b) {
            lazy[idx] = Op::op(p, lazy[idx]);
            return;
        }

        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);

        // モノイドOpが可換でない場合、pの適用前にlazy[idx]の適用が必要
        lazy[left_idx] = Op::op(lazy[idx], lazy[left_idx]);
        lazy[right_idx] = Op::op(lazy[idx], lazy[right_idx]);
        lazy[idx] = Op::id();

        update(a, b, p, left_idx, l, (l + r) / 2);
        update(a, b, p, right_idx, (l + r) / 2, r);

        fix_value(idx);
    }

    value_type query(int64_t a, int64_t b) { return query(a, b, 0, 0, cap); }

    value_type query(int64_t a, int64_t b, int64_t idx, int64_t l, int64_t r) {
        if (a >= r || b <= l) {
            // no overlap
            return M::id();
        }

        if (a <= l && r <= b) {
            return get_node_value(idx);
        }

        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);

        auto left_v = query(a, b, left_idx, l, (l + r) / 2);
        auto right_v = query(a, b, right_idx, (l + r) / 2, r);

        return f(lazy[idx], M::op(left_v, right_v));
    }

   private:
    value_type get_node_value(int64_t idx) { return f(lazy[idx], values[idx]); }

    void fix_value(int64_t idx) {
        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);
        values[idx] =
                M::op(get_node_value(left_idx), get_node_value(right_idx));
    }

    F f;
    int64_t n;
    int64_t height;
    int64_t cap;
    vector<value_type> values;
    vector<operator_type> lazy;
};

struct S {
    int64_t sum;
    int64_t len;
    int64_t inversion;
};

struct Monoid {
    using type = S;

    static type id() { return S{0L, 0L, 0L}; }

    static type op(S x, S y) {
        return S{x.sum + y.sum, x.len + y.len,
                 x.inversion + y.inversion + x.sum * (y.len - y.sum)};
    }
};

struct BoolXor {
    using type = bool;

    static type id() { return false; }

    static type op(bool x, bool y) { return x ^ y; }
};

struct Invert {
    S operator()(bool b, S s) const {
        if (b) {
            return S{s.len - s.sum, s.len,
                     s.sum * (s.len - s.sum) - s.inversion};
        } else {
            return s;
        }
    }
};

using ST = SegmentTree<Monoid, BoolXor, Invert>;

int main() {
    int64_t n, q;
    cin >> n >> q;

    ST st(n);
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        st.set(i, S{a, 1L, 0L});
    }

    for (auto _ : irange(0L, q)) {
        int64_t t, l, r;
        cin >> t >> l >> r;

        --l;
        if (t == 1) {
            st.update(l, r, true);
        } else {
            cout << st.query(l, r).inversion << "\n";
        }
    }
}