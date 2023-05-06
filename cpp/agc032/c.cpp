#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t dfs(const vector<vector<pair<int64_t, int64_t>>>& adj, int64_t c, vector<vector<bool>>& used) {
    for (int64_t i = 0; i < adj[c].size(); ++i) {
        if (used[c][i]) {
            continue;
        }

        auto a = adj[c][i].first;
        auto r = adj[c][i].second;

        used[c][i] = true;
        used[a][r] = true;

        if (adj[a].size() > 2) {
            return a;
        }

        auto ret = dfs(adj, a, used);
        if (ret >= 0) {
            return ret;
        }
    }

    return -1;
}

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<pair<int64_t, int64_t>>> adj(n);
    vector<int64_t> degree(n, 0);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        ++degree[a - 1];
        ++degree[b - 1];
        adj[a - 1].emplace_back(b - 1, adj[b - 1].size());
        adj[b - 1].emplace_back(a - 1, adj[a - 1].size() - 1);
    }

    for (auto d : degree) {
        if (d % 2 == 1) {
            cout << "No" << endl;
            return 0;
        }
    }

    vector<vector<bool>> used(n);
    for (auto i : irange(0L, n)) {
        used[i].resize(adj[i].size());
    }

    vector<unordered_map<int64_t, int64_t>> adj2(n);
    for (auto i : irange(0L, n)) {
        int64_t a;
        while ((a = dfs(adj, i, used)) >= 0) {
            ++adj2[i][a];
            ++adj2[a][i];

            // cerr << i << "-" << a << endl;
        }
    }

    int64_t count = 0;
    for (auto i : irange(0L, n)) {
        for (const auto& kv : adj2[i]) {
            if (kv.first == i) {
                count += kv.second;
            } else if (kv.first > i) {
                count += kv.second / 2;
            }
        }
    }

    if (count >= 3) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}