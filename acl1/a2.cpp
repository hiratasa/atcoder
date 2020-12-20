#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
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

   private:
    std::vector<int64_t> g;
};

int main() {
    int64_t n;
    cin >> n;

    vector<std::tuple<int64_t, int64_t, int64_t>> p;
    for (auto i : irange(0L, n)) {
        int64_t x, y;
        cin >> x >> y;

        p.emplace_back(x - 1, y - 1, i);
    }

    std::set<pair<int64_t, int64_t>> s;

    UnionFind uf(n);

    sort(p.begin(), p.end());
    for (auto [x, y, i] : p) {
        auto it = s.lower_bound(std::make_pair(y, i));

        if (it == s.begin()) {
            s.emplace(y, i);
        } else {
            auto rit = std::make_reverse_iterator(it);
            while (rit != s.rend()) {
                uf.unite(rit->second, i);
                ++rit;
            }

            auto b = s.begin();
            ++b;

            s.erase(b, it);
        }
    }

    vector sz(n, 0L);
    for (auto i : irange(0L, n)) {
        ++sz[uf.root(i)];
    }

    for (auto i : irange(0L, n)) {
        cout << sz[uf.root(i)] << "\n";
    }
}