#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& edges, const vector<int64_t>& d,
         vector<int64_t>& colors, int64_t v) {
    colors[v] = -2;  // visiting and color is not determined yet.
    for (const auto& u : edges[v]) {
        if (d[u] > d[v]) {
            continue;
        }

        if (colors[u] == -2L) {
            continue;
        }

        if (colors[u] == /* not visited */ -1L) {
            dfs(edges, d, colors, u);
        }

        if (colors[v] == -2L) {
            colors[v] = 1 - colors[u];
        }
    }

    if (colors[v] == -2L) {
        colors[v] = 0L;
    }
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> d(n);
    for (auto&& dd : d) {
        cin >> dd;
    }

    vector<vector<int64_t>> edges(n);
    vector<int64_t> weights(m), colors(n, -1L);
    vector<bool> ok(n, false);
    for (auto i : irange(0L, m)) {
        int64_t u, v;
        cin >> u >> v;
        --u;
        --v;

        auto weight = max(d[u], d[v]);

        edges[u].push_back(v);
        edges[v].push_back(u);
        weights[i] = weight;

        if (d[u] >= d[v]) {
            ok[u] = true;
        }
        if (d[v] >= d[u]) {
            ok[v] = true;
        }
    }

    if (find(ok.begin(), ok.end(), false) != ok.end()) {
        cout << -1 << endl;
        return 0;
    }

    for (auto v : irange(0L, n)) {
        if (colors[v] < 0) {
            dfs(edges, d, colors, v);
        }
    }

    for (auto v : irange(0L, n)) {
        cout << (colors[v] ? "B" : "W");
    }
    cout << endl;

    for (auto w : weights) {
        cout << w << "\n";
    }
}