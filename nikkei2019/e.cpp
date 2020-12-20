#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

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

    vector<int64_t> x(n);
    for (auto&& xx : x) {
        cin >> xx;
    }

    vector<pair<int64_t, pair<int64_t, int64_t>>> links(m);
    for (auto i : irange(0L, m)) {
        cin >> links[i].second.first >> links[i].second.second >>
                links[i].first;
        --links[i].second.first;
        --links[i].second.second;
    }

    sort(links.begin(), links.end());

    vector<int64_t> g(n), a(n);
    iota(g.begin(), g.end(), 0L);

    int64_t r = 0;
    for (const auto& link : links) {
        auto w = link.first;
        auto v = link.second.first;
        auto u = link.second.second;

        if (root(g, v) != root(g, u)) {
            auto x1 = x[root(g, v)];
            auto x2 = x[root(g, u)];
            auto a1 = a[root(g, v)];
            auto a2 = a[root(g, u)];
            merge(g, v, u);
            x[root(g, v)] = x1 + x2;
            a[root(g, v)] = a1 + a2 + 1;
        } else {
            ++a[root(g, v)];
        }

        if (w <= x[root(g, v)]) {
            r += a[root(g, v)];
            // cerr << v << "-" << u << ":" << a[root(g, v)] << endl;
            a[root(g, v)] = 0;
        }
    }

    cout << m - r << endl;
}