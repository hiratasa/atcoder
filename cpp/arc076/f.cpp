#include <iostream>
#include <vector>
#include <utility>
#include <string>
#include <numeric>
#include <cmath>
#include <cassert>
#include <algorithm>
#include <cstdint>
#include <unordered_map>
#include <unordered_set>
#include <queue>

using namespace std;

// Segment Tree
struct ST {
    ST(int n)
     : n(n), cap(pow(2, int(log2(n - 1)) + 1)), value(2 * cap - 1, int64_t(kInit)), extra(2 * cap - 1, 0) {}

    static int64_t calc(int64_t x, int64_t y) {
        return max(x, y);
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

    // only for init (can't be executed after range_add())
    void assign(int idx, int64_t v) {
        int c = leaf(idx);
        value[c] = v;
        extra[c] = 0;

        while (!is_root(c)) {
            c = parent(c);
            value[c] = calc(value[left(c)], value[right(c)]);
            assert(extra[c] == 0);
        }
    }

    void range_add(int a, int b, int v, int cur = 0, int l = 0, int r = -1) {
        if (r == -1) {
            r = cap;
        }

        if (r <= a || b <= l) {
            return;
        }

        if (a <= l && r <= b) {
            extra[cur] += v;
            return;
        }

        range_add(a, b, v, left(cur), l, (l + r) / 2);
        range_add(a, b, v, right(cur), (l + r) / 2, r);

        value[cur] = calc(value[left(cur)] + extra[left(cur)], value[right(cur)] + extra[right(cur)]);
    }

    int64_t query(int a, int b, int cur = 0, int l = 0, int r = -1) {
        if (r == -1) {
            r = cap;
        }

        if (r <= a || b <= l) {
            return kInit;
        }

        // delay evaluation
        if (extra[cur] > 0 && has_children(cur)) {
            extra[left(cur)] += extra[cur];
            extra[right(cur)] += extra[cur];
        }

        value[cur] += extra[cur];
        extra[cur] = 0;

        if (a <= l && r <= b) {
            return value[cur];
        }

        int64_t vl = query(a, b, left(cur), l, (l + r) / 2);
        int64_t vr = query(a, b, right(cur), (l + r) / 2, r);

        return calc(vl, vr);
    }

    void dump() const {
        int num_col = 0;
        int i_col = 0;
        for (int idx = 0, i_col = 0, num_col = 1; idx < value.size(); ++idx) {
            cerr << " (" << value[idx] << "," << extra[idx] << ")";
            ++i_col;
            if (!(i_col < num_col)) {
                cerr << "\n";
                i_col = 0;
                num_col *= 2;
            }
        }
    }

    static constexpr int64_t kInit = -1000000L;

    int n;
    int cap;
    vector<int64_t> value;
    vector<int64_t> extra;
};

int64_t solve(int m, vector<pair<int, int>>& lr) {
    int n = lr.size();

    ST tree(m + 2);
    tree.assign(0, -m);
    for (int i = 1; i <= m; ++i) {
        tree.assign(i, -(m - i + 1));
    }
    tree.assign(m + 1, 0);

    // tree.dump();

    int64_t ans = n - m;
    sort(lr.begin(), lr.end());
    for (int i = 0; i < lr.size(); ++i) {
        tree.range_add(0, lr[i].second + 1, 1);
        while (i + 1 < lr.size() && lr[i].first == lr[i + 1].first) {
            ++i;
            tree.range_add(0, lr[i].second + 1, 1);
        }
        // tree.dump();

        if (lr[i].first == m) {
            break;
        }

        auto tmp = tree.query(lr[i].first + 2, m + 2);
        // tree.dump();
        tmp -= lr[i].first;
        if (tmp > ans) {
            ans = tmp;
        }
    }

    if (ans < 0) {
        ans = 0;
    }

    return ans;
}

int main() {
    int n, m;
    cin >> n >> m;

    vector<pair<int, int>> lr(n);
    for (auto&& c : lr) {
        cin >> c.first >> c.second;
    }

    cout << solve(m, lr) << endl;

    return 0;
}