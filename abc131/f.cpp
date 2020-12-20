#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t root(vector<int64_t>& g, int64_t a) {
    if (g[a] != a) {
        g[a] = root(g, g[a]);
    }

    return g[a];
}

void merge(vector<int64_t>& g, int64_t a, int64_t b) {
    g[root(g, a)] = root(g, b);
}

main() {
    int64_t n;
    cin >> n;

    constexpr auto M = 100000L;

    vector<pair<int64_t, int64_t>> p(n);
    vector<int64_t> g(2 * M + 1);
    iota(g.begin(), g.end(), 0);
    for (auto i : irange(0L, n)) {
        auto&& pp = p[i];
        cin >> pp.first >> pp.second;
        merge(g, pp.first, M + pp.second);
    }

    unordered_map<int64_t, pair<int64_t, int64_t>> m;
    for (auto i : irange(1L, M + 1)) {
        ++m[root(g, i)].first;
        ++m[root(g, M + i)].second;
    }

    int64_t ans = 0;
    for (auto kv : m) {
        auto v = kv.second;
        ans += v.first * v.second;
    }

    ans -= n;

    cout << ans << endl;
}