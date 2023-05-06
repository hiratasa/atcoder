#include <bits/stdc++.h>

using namespace std;

struct UnionFind {
   public:
    explicit UnionFind(int64_t n) : g(n) { std::iota(g.begin(), g.end(), 0L); }

    int64_t root(int64_t v) {
        if (g[v] != v) {
            g[v] = root(g[v]);
        }

        return g[v];
    }

    void unite(int64_t v, int64_t u) { g[root(v)] = root(u); }

    bool same(int64_t v, int64_t u) { return root(v) == root(u); }

   private:
    std::vector<int64_t> g;
};