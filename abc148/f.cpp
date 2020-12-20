#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& adjs, int64_t current,
         vector<int64_t>& parent, vector<int64_t>& depth,
         vector<int64_t>& max_depth) {
    for (auto v : adjs[current]) {
        if (v == parent[current]) {
            continue;
        }

        depth[v] = depth[current] + 1;
        max_depth[v] = depth[current] + 1;
        parent[v] = current;
        dfs(adjs, v, parent, depth, max_depth);
        max_depth[current] = max(max_depth[current], max_depth[v]);
    }
}

int main() {
    int64_t n, u, v;
    cin >> n >> u >> v;
    --u;
    --v;

    vector<vector<int64_t>> adjs(n);
    for (auto i : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    vector<int64_t> parent(n, -1), depth(n), max_depth(n);
    dfs(adjs, v, parent, depth, max_depth);
    int64_t u_depth = depth[u];
    int64_t m = max_depth[u];
    while (parent[u] >= 0) {
        u = parent[u];
        if (2 * depth[u] <= u_depth) {
            break;
        }

        m = max(m, max_depth[u]);
    }

    cout << m - 1 << endl;
}