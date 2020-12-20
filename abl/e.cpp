#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

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

constexpr auto M = 998244353L;

struct Mod {
    using type = pair<int64_t, int64_t>;

    static type id() { return make_pair(0L, 1L); }
    static type op(type v1, type v2) {
        return make_pair((v1.first * v2.second % M + v2.first) % M,
                         v1.second * v2.second % M);
    }
};

struct ReplaceDigit {
    using type = int64_t;

    static type id() { return 0; }
    static type op(type v1, type v2) {
        if (v1 == 0)
            return v2;
        else
            return v1;
    }
};

struct Apply {
    pair<int64_t, int64_t> operator()(int64_t x, pair<int64_t, int64_t> y) {
        if (x == 0) {
            return y;
        }

        constexpr auto kInv9 = 443664157L;
        return make_pair(x * (y.second - 1) % M * kInv9 % M, y.second);
    }
};

using ST = SegmentTree<Mod, ReplaceDigit, Apply>;

int main() {
    int64_t n, q;
    cin >> n >> q;

    ST st(n);
    for (auto i : irange(0L, n)) {
        st.set(i, make_pair(1, 10));
    }

    for (auto _ : irange(0L, q)) {
        int64_t l, r, d;
        cin >> l >> r >> d;
        --l;

        st.update(l, r, d);
        cout << st.query(0, n).first << "\n";
    }
}