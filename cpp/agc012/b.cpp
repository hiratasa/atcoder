#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    int64_t q;
    cin >> q;
    vector<vector<int64_t>> k(n, vector<int64_t>(11, 0L));
    vector<int64_t> c(q + 1);
    for (auto i : irange(0L, q)) {
        int64_t v, d;
        cin >> v >> d >> c[i + 1];
        --v;
        k[v][d] = max(k[v][d], i + 1);
    }

    for (auto i : irange(1L, 11L) | reversed) {
        for (auto v : irange(0L, n)) {
            k[v][0] = max(k[v][0], k[v][i]);
            for (auto u : adjs[v]) {
                k[u][i - 1] = max(k[u][i - 1], k[v][i]);
            }
        }
    }

    for (auto i : irange(0L, n)) {
        cout << c[k[i][0]] << "\n";
    }
}