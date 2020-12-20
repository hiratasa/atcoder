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

    vector<int64_t> g(2 * n);
    iota(g.begin(), g.end(), 0L);

    for (auto i : irange(0L, n)) {
        int64_t p;
        cin >> p;
        --p;

        merge(g, i, n + p);
    }

    for (auto i : irange(0L, m)) {
        int64_t x, y;
        cin >> x >> y;
        --x;
        --y;

        merge(g, n + x, n + y);
    }

    int64_t ans = 0L;
    for (auto i : irange(0L, n)) {
        if (root(g, i) == root(g, n + i)) {
            ++ans;
        }
    }

    cout << ans << endl;
}