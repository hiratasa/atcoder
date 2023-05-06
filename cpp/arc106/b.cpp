#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t dfs(const vector<vector<int64_t>>& adjs, vector<bool>& visited,
            vector<int64_t>& vals, int64_t v) {
    if (visited[v]) {
        return 0;
    }

    int64_t s = vals[v];
    visited[v] = true;
    for (auto u : adjs[v]) {
        s += dfs(adjs, visited, vals, u);
    }

    return s;
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector a(n, 0L), b(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t c, d;
        cin >> c >> d;
        --c;
        --d;
        adjs[c].push_back(d);
        adjs[d].push_back(c);
    }

    vector visited(n, false), visited2(n, false);
    for (auto i : irange(0L, n)) {
        if (!visited[i]) {
            int64_t sa = dfs(adjs, visited, a, i);
            int64_t sb = dfs(adjs, visited2, b, i);

            if (sa != sb) {
                cout << "No" << endl;
                return 0;
            }
        }
    }

    cout << "Yes" << endl;
}