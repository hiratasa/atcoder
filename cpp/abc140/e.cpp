#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

class SegmentTree {
   public:
    SegmentTree(int64_t n)
            : n(n),
              cap(pow(2, ceil(log2(n)))),
              values(2 * cap - 1, make_pair(numeric_limits<int64_t>::min(),
                                            numeric_limits<int64_t>::min())) {}

    void set(int64_t pos, int64_t v) {
        int64_t idx = cap - 1 + pos;
        values[idx] = make_pair(v, numeric_limits<int64_t>::min());
        while (idx > 0) {
            idx = (idx - 1) / 2;

            if (values[idx].first < v) {
                values[idx].second = values[idx].first;
                values[idx].first = v;
            } else if (values[idx].second < v) {
                values[idx].second = v;
            }
        }
    }

    pair<int64_t, int64_t> query(int64_t a, int64_t b, int64_t idx = 0,
                                 int64_t l = 0, int64_t r = -1) {
        if (r < 0) {
            r = cap;
        }

        if (a >= r || b <= l) {
            // no overlap
            return make_pair(numeric_limits<int64_t>::min(),
                             numeric_limits<int64_t>::min());
        }

        if (a <= l && r <= b) {
            return values[idx];
        }

        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);

        auto left_v = query(a, b, left_idx, l, (l + r) / 2);
        auto right_v = query(a, b, right_idx, (l + r) / 2, r);

        array<int64_t, 4> tmp{left_v.first, left_v.second, right_v.first,
                              right_v.second};
        sort(tmp.rbegin(), tmp.rend());

        return make_pair(tmp[0], tmp[1]);
    }

    void dump() const {
        for (auto i : irange(0uL, values.size())) {
            cerr << i << ":" << values[i].first << "," << values[i].second
                 << endl;
        }
    }

   private:
    int64_t n;
    int64_t cap;
    vector<pair<int64_t, int64_t>> values;
};

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> p(n);
    for (auto i : irange(0L, n)) {
        cin >> p[i];
    }

    vector<pair<int64_t, int64_t>> nums_l(n);
    SegmentTree st(n + 1);
    for (auto i : irange(0L, n)) {
        auto tmp = st.query(p[i] + 1, n + 1);

        cerr << tmp.first << " " << tmp.second << endl;
        // st.dump();

        if (tmp.first == numeric_limits<int64_t>::min()) {
            nums_l[i] = make_pair(0, i + 1);
        } else if (tmp.second == numeric_limits<int64_t>::min()) {
            nums_l[i] = make_pair(tmp.first + 1, i - tmp.first);
        } else {
            nums_l[i] = make_pair(tmp.first - tmp.second, i - tmp.first);
        }

        st.set(p[i], i);
    }

    vector<pair<int64_t, int64_t>> nums_r(n);
    SegmentTree st2(n + 1);
    for (auto i : irange(n - 1, -1L, -1L)) {
        auto tmp = st2.query(p[i] + 1, n + 1);

        cerr << tmp.first << " " << tmp.second << endl;
        // st2.dump();

        if (tmp.first == numeric_limits<int64_t>::min()) {
            nums_r[i] = make_pair(0, n - i);
        } else if (tmp.second == numeric_limits<int64_t>::min()) {
            nums_r[i] = make_pair(n - (-tmp.first), (-tmp.first) - i);
        } else {
            nums_r[i] =
                    make_pair((-tmp.second) - (-tmp.first), (-tmp.first) - i);
        }

        st2.set(p[i], -i);
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        ans += p[i] * (nums_l[i].first * nums_r[i].second +
                       nums_l[i].second * nums_r[i].first);
    }

    cout << ans << endl;
}