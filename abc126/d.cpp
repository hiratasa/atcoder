#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

void dfs(const vector<vector<pair<int, int>>>& links, vector<int>& color, int v, int64_t value) {
    if (color[v] != -1) {
        return;
    }

    color[v] = value % 2;
    for (const auto& link : links[v]) {
        dfs(links, color, link.first, value + link.second);
    }
}

main() {
    int n;
    cin >> n;

    vector<vector<pair<int, int>>> links(n);
    for (auto _ : irange(0, n - 1)) {
        int u, v, w;
        cin >> u >> v >> w;
        --u;
        --v;
        links[u].emplace_back(v, w);
        links[v].emplace_back(u, w);
    }

    vector<int> color(n, -1);
    dfs(links, color, 0, 0);

    for (auto c : color) {
        cout << c << endl;
    }
}