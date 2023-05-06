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

    unordered_set<int64_t> langs;
    vector<int64_t> g(m);
    iota(g.begin(), g.end(), 0L);
    for (auto _ : irange(0L, n)) {
        int64_t k;
        cin >> k;

        int64_t l0;
        cin >> l0;
        --l0;
        langs.insert(l0);
        for (auto _ : irange(0L, k - 1)) {
            int64_t l;
            cin >> l;
            --l;
            merge(g, l0, l);
            langs.insert(l);
        }
    }

    auto l0 = *langs.begin();
    for (auto l : langs) {
        if (root(g, l0) != root(g, l)) {
            cout << "NO" << endl;
            return 0;
        }
    }

    cout << "YES" << endl;
}