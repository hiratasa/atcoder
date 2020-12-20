#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

// based on https://tubo28.me/compprog/algorithm/doubling-lca/
class lca {
   public:
    using Graph = vector<vector<pair<int64_t, pair<int64_t, int64_t>>>>;
    const int n = 0;
    const int log2_n = 0;
    std::vector<std::vector<int>> parent;
    std::vector<int> depth;

    lca() {}

    lca(const Graph& g, int root)
            : n(g.size()),
              log2_n(log2(n) + 1),
              parent(log2_n, std::vector<int>(n)),
              depth(n) {
        dfs(g, root, -1, 0);
        for (int k = 0; k + 1 < log2_n; k++) {
            for (int v = 0; v < (int)g.size(); v++) {
                if (parent[k][v] < 0)
                    parent[k + 1][v] = -1;
                else
                    parent[k + 1][v] = parent[k][parent[k][v]];
            }
        }
    }

    void dfs(const Graph& g, int v, int p, int d) {
        parent[0][v] = p;
        depth[v] = d;
        for (const auto& e : g[v]) {
            if (e.first != p) dfs(g, e.first, v, d + 1);
        }
    }

    int get(int u, int v) {
        if (depth[u] > depth[v]) std::swap(u, v);
        for (int k = 0; k < log2_n; k++) {
            if ((depth[v] - depth[u]) >> k & 1) {
                v = parent[k][v];
            }
        }
        if (u == v) return u;
        for (int k = log2_n - 1; k >= 0; k--) {
            if (parent[k][u] != parent[k][v]) {
                u = parent[k][u];
                v = parent[k][v];
            }
        }
        return parent[0][u];
    }
};

main() {
    int64_t n, q;
    cin >> n >> q;

    vector<vector<pair<int64_t, pair<int64_t, int64_t>>>> links(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b, c, d;
        cin >> a >> b;
        --a;
        --b;
        --c;
        links[a].emplace_back(b, make_pair(c, d));
        links[b].emplace_back(a, make_pair(c, d));
    }

    lca g(links, 0);

    for (auto _ : irange(0L, q)) {
        int64_t x, y, u, v;
        cin >> x >> y >> u >> v;
        auto ans = g.get(u, v);
    }
}
