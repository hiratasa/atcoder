#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct UnionFind {
   public:
    explicit UnionFind(int64_t n) : g(n), s(n, 1L) {
        std::iota(g.begin(), g.end(), 0L);
    }

    int64_t root(int64_t v) {
        if (g[v] != v) {
            g[v] = root(g[v]);
        }

        return g[v];
    }

    void unite(int64_t v, int64_t u) {
        if (root(u) != root(v)) {
            s[root(u)] += s[root(v)];
            g[root(v)] = root(u);
        }
    }

    int64_t size(int64_t v) { return s[root(v)]; }

   private:
    std::vector<int64_t> g;
    std::vector<int64_t> s;
};

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, vector(n, 0L));
    for (auto&& aa : a) {
        for (auto&& aaa : aa) {
            cin >> aaa;
        }
    }

    UnionFind uf1(n), uf2(n);
    for (auto i : irange(0L, n)) {
        for (auto j : irange(i + 1, n)) {
            bool ok = true;
            for (auto l : irange(0L, n)) {
                if (a[i][l] + a[j][l] > k) {
                    ok = false;
                    break;
                }
            }
            if (ok) {
                uf1.unite(i, j);
            }
        }
    }
    for (auto i : irange(0L, n)) {
        for (auto j : irange(i + 1, n)) {
            bool ok = true;
            for (auto l : irange(0L, n)) {
                if (a[l][i] + a[l][j] > k) {
                    ok = false;
                    break;
                }
            }
            if (ok) {
                uf2.unite(i, j);
            }
        }
    }

    constexpr auto M = 998244353;

    vector fact(n + 1, 1L);
    for (auto i : irange(2L, n + 1)) {
        fact[i] = fact[i - 1] * i % M;
    }

    int64_t ans = 1;
    for (auto i : irange(0L, n)) {
        if (uf1.root(i) == i) {
            ans *= fact[uf1.size(i)];
        }
        ans %= M;
        if (uf2.root(i) == i) {
            ans *= fact[uf2.size(i)];
        }
        ans %= M;
    }

    cout << ans << endl;
}