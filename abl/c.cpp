#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

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

int main() {
    int64_t n, m;
    cin >> n >> m;

    UnionFind uf(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;

        uf.unite(a, b);
    }

    unordered_set<int64_t> k;
    for (auto i : irange(0L, n)) {
        k.insert(uf.root(i));
    }

    cout << k.size() - 1 << endl;
}