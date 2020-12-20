#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& adjs, vector<bool>& visited,
         vector<int64_t>& path, int64_t v) {
    visited[v] = true;
    path.push_back(v);

    for (auto u : adjs[v]) {
        if (!visited[u]) {
            dfs(adjs, visited, path, u);
            return;
        }
    }
}

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

    vector<bool> visited(n);
    vector<int64_t> path;
    dfs(adjs, visited, path, 0L);
    reverse(path.begin(), path.end());
    path.pop_back();
    dfs(adjs, visited, path, 0L);

    cout << path.size() << endl;
    cout << path.front() + 1;
    for (auto v : path | sliced(1L, path.size())) {
        cout << " " << v + 1;
    }
    cout << endl;
}