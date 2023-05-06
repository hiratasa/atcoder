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

    static type id() { return 0L; }
    static type op(type v1, type v2) { return max(v1, v2); }
};

using ST = SegmentTree<Max>;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    constexpr auto M = 300000L;
    ST st(M + 1);
    for (auto i : irange(0L, n)) {
        auto p = st.query(max(0L, a[i] - k), min(M, a[i] + k) + 1);

        st.set(a[i], p + 1);
    }

    cout << st.query(0L, M + 1) << endl;
}