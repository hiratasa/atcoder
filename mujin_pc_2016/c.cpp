#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

template <typename T>
void dfs(const vector<vector<int64_t>>& adjs, const T& color,
         vector<bool>& visited, int64_t v) {
    visited[v] = true;
    for (auto u : adjs[v]) {
        if (color[v] == color[u]) {
            continue;
        }

        if (visited[u]) {
            continue;
        }

        dfs(adjs, color, visited, u);
    }
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t x, y;
        cin >> x >> y;
        --x;
        --y;
        adjs[x].push_back(y);
        adjs[y].push_back(x);
    }

    int64_t ans = 0;
    for (auto u : irange(0uL, 1uL << (n - 1))) {
        bitset<16> bs(u);

        vector<bool> visited(n);
        dfs(adjs, bs, visited, 0);

        if (find(visited.begin(), visited.end(), false) != visited.end()) {
            continue;
        }

        ++ans;
    }

    cout << ans << endl;
}