#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

struct ST {
    using ValueType = pair<int64_t, int64_t>;

    static constexpr int64_t kInit = (1L << 40);

    ST(int n)
     : n(n), cap(pow(2, int(log2(n - 1)) + 1)), value(2 * cap - 1, make_pair(kInit, -1)), extra(2 * cap - 1, kInit) {}

    static ValueType calc(ValueType x, ValueType y) {
        return min(x, y);
    }

    ValueType get_value(int idx) {
        return make_pair(value[idx].first + extra[idx], value[idx].second);
    }

    bool is_root(int idx) const {
        return idx == 0;
    }

    bool has_children(int idx) const {
        return right(idx) < value.size();
    }

    int parent(int idx) const {
        return (idx - 1) / 2;
    }

    int left(int idx) const {
        return 2 * idx + 1;
    }

    int right(int idx) const {
        return 2 * idx + 2;
    }

    int leaf(int idx) const {
        return cap - 1 + idx;
    }

    // only for init (can't be executed after setminoffset())
    void assign(int idx, int64_t v) {
        int c = leaf(idx);
        value[c].first = v;
        value[c].second = idx;
        extra[c] = kInit;

        while (!is_root(c)) {
            c = parent(c);
            value[c] = calc(value[left(c)], value[right(c)]);
            assert(extra[c] == kInit);
        }
    }

    void reset(int a, int cur = 0, int l = 0, int r = -1) {
        if (r == -1) {
            r = cap;
        }

        if (r <= a || a < l) {
            return;
        }

        if (!has_children(cur)) {
            value[cur].first = kInit;
            return;
        }

        reset(a, left(cur), l, (l + r) / 2);
        reset(a, right(cur), (l + r) / 2, r);

        value[cur] = calc(get_value(left(cur)), get_value(right(cur)));
    }

    void setminoffset(int a, int b, int64_t v, int cur = 0, int l = 0, int r = -1) {
        if (r == -1) {
            r = cap;
        }

        if (r <= a || b <= l) {
            return;
        }

        if (a <= l && r <= b) {
            extra[cur] = min(extra[cur], v);
            return;
        }

        extra[left(cur)] = min(extra[left(cur)], extra[cur]);
        setminoffset(a, b, v, left(cur), l, (l + r) / 2);

        extra[right(cur)] = min(extra[right(cur)], extra[cur]);
        setminoffset(a, b, v, right(cur), (l + r) / 2, r);
        
        extra[cur] = kInit;
        value[cur] = calc(get_value(left(cur)), get_value(right(cur)));
    }

    ValueType query(int a, int b, int cur = 0, int l = 0, int r = -1) {
        if (r == -1) {
            r = cap;
        }

        if (r <= a || b <= l) {
            return make_pair(kInit, -1L);
        }

        // delay evaluation
        if (extra[cur] != kInit && has_children(cur)) {
            extra[left(cur)] = min(extra[left(cur)], extra[cur]);
            extra[right(cur)] = min(extra[right(cur)], extra[cur]);
            value[cur] = calc(get_value(left(cur)), get_value(right(cur)));
            extra[cur] = kInit;
        }


        if (a <= l && r <= b) {
            return get_value(cur);
        }

        auto vl = query(a, b, left(cur), l, (l + r) / 2);
        auto vr = query(a, b, right(cur), (l + r) / 2, r);

        return calc(vl, vr);
    }

    void dump() const {
        int num_col = 0;
        int i_col = 0;
        for (int idx = 0, i_col = 0, num_col = 1; idx < value.size(); ++idx) {
            cerr << " (" << value[idx].first << "," << extra[idx] << ")";
            ++i_col;
            if (!(i_col < num_col)) {
                cerr << "\n";
                i_col = 0;
                num_col *= 2;
            }
        }
    }

    int n;
    int cap;
    vector<ValueType> value;
    vector<int64_t> extra;
};

const int64_t ST::kInit;

main() {
    int64_t n, d;
    cin >> n >> d;

    ST postree(n);
    ST negtree(n);

    vector<int64_t> a(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
        postree.assign(i, i * d + a[i]);
        negtree.assign(i, -i * d + a[i]);
    }

    constexpr auto kOffset = (1L << 35);
    postree.reset(0);
    postree.setminoffset(0, n, a[0]);
    negtree.reset(0);
    negtree.setminoffset(0, n, kOffset);
    int64_t cost = 0;
    for (auto _ : irange(1L, n)) {
        postree.dump();
        negtree.dump();

        auto mp = postree.query(0, n);
        auto mn = negtree.query(0, n);

        if (mp < mn) {
            cost += mp.first;
            auto j = mp.second;
            cerr << "p" << j << ":" << mp.first << endl;
            negtree.setminoffset(0, j, j * d + a[j]);
            postree.reset(j);
            negtree.reset(j);
        } else {
            cost += mn.first;
            auto j = mn.second;
            cerr << "n" << j << ":" << mn.first << endl;
            postree.setminoffset(j + 1, n, -j * d + a[j]);
            postree.reset(j);
            negtree.reset(j);
        }

    }

    cout << cost << endl;
}