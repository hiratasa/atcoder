#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t root(vector<int64_t>& g, int64_t v) {
    if (g[v] != v) {
        g[v] = root(g, g[v]);
    }

    return g[v];
}

void merge(vector<int64_t>& g, int64_t v, int64_t u) {
    g[root(g, v)] = root(g, u);
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> g(n);
    iota(g.begin(), g.end(), 0L);

    auto b = a;
    for (auto _ : irange(0L, m)) {
        int64_t x, y;
        cin >> x >> y;

        int64_t tmp = min(b[root(g, x)], b[root(g, y)]);
        merge(g, x, y);
        b[root(g, x)] = tmp;
    }

    int64_t c = n - m;
    if (n < 2 * (c - 1)) {
        cout << "Impossible" << endl;
        return 0;
    }

    if (c == 1) {
        cout << 0 << endl;
        return 0;
    }

    int64_t ans = 0;
    vector<int64_t> r;
    for (auto i : irange(0L, n)) {
        if (b[root(g, i)] == a[i]) {
            b[root(g, i)] = 1L << 40;
            ans += a[i];
        } else {
            r.push_back(a[i]);
        }
    }

    sort(r.begin(), r.end());
    for (auto i : irange(0L, 2 * (c - 1) - c)) {
        ans += r[i];
    }

    cout << ans << endl;
}