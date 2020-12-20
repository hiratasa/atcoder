#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

template <typename M>
class SegmentTree {
   public:
    using value_type = typename M::type;

    explicit SegmentTree(int64_t n)
            : n(n), cap(pow(2, ceil(log2(n)))), values(2 * cap - 1, +M::id) {}

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
            return M::id;
        }

        if (a <= l && r <= b) {
            return get_node_value(idx);
        }

        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);

        fix_value(idx);

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

struct Sum {
    using type = int64_t;
    static constexpr auto id = 0L;

    static type op(type v1, type v2) { return v1 + v2; }
};

using ST = SegmentTree<Sum>;

int main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    vector<ST> st(26, ST(n));

    for (auto i : irange(0L, n)) {
        st[s[i] - 'a'].set(i, 1);
    }

    int64_t q;
    cin >> q;

    for (auto _ : irange(0L, q)) {
        int64_t t;
        cin >> t;
        if (t == 1) {
            int64_t i;
            char c;
            cin >> i >> c;
            --i;

            st[s[i] - 'a'].set(i, 0L);
            s[i] = c;
            st[s[i] - 'a'].set(i, 1L);
        } else {
            int64_t l, r;
            cin >> l >> r;
            --l;
            --r;

            int64_t ans = 0;
            for (auto i : irange(0L, 26L)) {
                if (st[i].query(l, r + 1) > 0) {
                    ++ans;
                }
            }
            cout << ans << endl;
        }
    }
}