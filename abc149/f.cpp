#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007L;

int64_t mpow(int64_t a, int64_t b) {
    if (b == 0) {
        return 1;
    }

    if (b == 1) {
        return a;
    }

    auto t = mpow(a, b / 2);

    return t * t % M * mpow(a, b % 2) % M;
}

int64_t calc_tree_size(const vector<vector<int64_t>>& adjs,
                       vector<int64_t>& tree_size, int64_t v, int64_t p = -1) {
    tree_size[v] = 1;
    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        tree_size[v] += calc_tree_size(adjs, tree_size, u, v);
    }

    return tree_size[v];
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    vector<int64_t> tree_size(n);
    calc_tree_size(adjs, tree_size, 0);

    int64_t inv2 = M / 2 + 1;

    int64_t ans = 0;
    for (auto i : irange(1L, n)) {
        int64_t s1 = tree_size[i];
        int64_t s2 = n - tree_size[i];

        int64_t q1 = (M + 1 - mpow(inv2, s1)) % M;
        int64_t q2 = (M + 1 - mpow(inv2, s2)) % M;

        ans += q1 * q2 % M;
        ans %= M;
    }

    ans = (2 * M + ans + 1 - mpow(inv2, n) - n * inv2 % M) % M;

    cout << ans << endl;
}