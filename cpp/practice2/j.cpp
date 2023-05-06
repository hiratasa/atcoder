#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

// M (type, id, op): Monoid of value
template <typename M>
class SegmentTree {
   public:
    using value_type = typename M::type;

    explicit SegmentTree(int64_t n)
            : n(n), cap(pow(2, ceil(log2(n)))), values(2 * cap - 1, M::id()) {}

    void set(int64_t pos, value_type v) {
        int64_t idx = cap - 1 + pos;

        values[idx] = v;

        while (idx > 0) {
            idx = (idx - 1) / 2;
            fix_value(idx);
        }
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

        return M::op(left_v, right_v);
    }

    // pred(op(value[a], ..., value[b-1])) = true
    // pred(op(value[a], ..., value[b])) = false
    // となる最初のb(>=a)を返す
    template <typename Pred>
    int64_t max_right(int64_t a, Pred&& pred) {
        value_type carry = M::id();
        assert(pred(carry));
        return min(max_right(a, pred, 0, 0, cap, carry), n);
    }

    // 入力されるcarryは以下のいずれか:
    //  * a<lであれば、M::op(value[a], ..., value[l-1])
    //  * そうでなければ、M::id()
    // 出力するcarryは以下のいずれか
    //  * r<=aであれば、入力まま(=M::id())
    //  * そうでなければ、M::op(value[a], ..., value[ret-1])
    //  いずれの場合もpred(carry)=trueを満たす
    template <typename Pred>
    int64_t max_right(int64_t a, Pred& pred, int64_t idx, int64_t l, int64_t r,
                      value_type& carry) {
        if (r <= a) {
            return a;
        }

        if (a <= l) {
            auto tmp = M::op(carry, get_node_value(idx));
            if (pred(tmp)) {
                carry = tmp;
                return r;
            }
        }

        if (l + 1 == r) {
            return l;
        }

        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);

        auto left_r = max_right(a, pred, left_idx, l, (l + r) / 2, carry);
        if (left_r < (l + r) / 2) {
            return left_r;
        }

        return max_right(a, pred, right_idx, (l + r) / 2, r, carry);
    }

   private:
    value_type get_node_value(int64_t idx) { return values[idx]; }

    void fix_value(int64_t idx) {
        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);
        values[idx] =
                M::op(get_node_value(left_idx), get_node_value(right_idx));
    }

    int64_t n;
    int64_t cap;
    vector<value_type> values;
};

struct Max {
    using type = int64_t;

    static type id() { return -1L; }
    static type op(type v1, type v2) { return max(v1, v2); }
};

using ST = SegmentTree<Max>;

int main() {
    int64_t n, q;
    cin >> n >> q;

    ST st(n);
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        st.set(i, a);
    }

    for (auto _ : irange(0L, q)) {
        int64_t t, x, v;
        cin >> t >> x >> v;

        if (t == 1) {
            st.set(x - 1, v);
        } else if (t == 2) {
            cout << st.query(x - 1, v) << "\n";
        } else {
            auto idx = st.max_right(x - 1, [&](int64_t a) { return a < v; });
            cout << idx + 1 << "\n";
        }
    }
}