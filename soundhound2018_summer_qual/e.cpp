#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool dfs(const vector<vector<pair<int64_t, int64_t>>>& adjs, vector<int64_t>& f,
         vector<int64_t>& k, int64_t& x, int64_t v) {
    for (const auto& link : adjs[v]) {
        auto u = link.first;
        auto s = link.second;

        auto ff = (f[v] + 1) % 2;
        auto kk = s - k[v];

        // if visited, verify consistency
        if (f[u] >= 0) {
            if (f[u] != ff) {
                int64_t xx = kk - k[u];
                if (xx % 2 != 0) {
                    return false;
                }
                xx /= 2;

                if (f[u]) {
                    xx *= -1;
                }

                if (x >= 0 && x != xx) {
                    return false;
                }

                if (xx < 0) {
                    return false;
                }

                x = xx;
            } else {
                if (kk != k[u]) {
                    return false;
                }
            }
        } else {
            f[u] = ff;
            k[u] = kk;

            if (!dfs(adjs, f, k, x, u)) {
                return false;
            }
        }
    }

    return true;
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<pair<int64_t, int64_t>>> adjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t u, v, s;
        cin >> u >> v >> s;
        --u;
        --v;
        adjs[u].emplace_back(v, s);
        adjs[v].emplace_back(u, s);
    }

    vector<int64_t> f(n, -1), k(n, -1);
    int64_t x = -1;
    f[0] = 0;
    k[0] = 1;
    if (!dfs(adjs, f, k, x, 0)) {
        cout << 0 << endl;
        return 0;
    }

    if (x >= 0) {
        for (auto i : irange(0L, n)) {
            if (f[i] == 0) {
                if (x + k[i] <= 0) {
                    cout << 0 << endl;
                    return 0;
                }
            } else {
                if (-x + k[i] <= 0) {
                    cout << 0 << endl;
                    return 0;
                }
            }
        }
        cout << 1 << endl;
    } else {
        int64_t l0 = numeric_limits<int64_t>::max(),
                l1 = numeric_limits<int64_t>::max();
        for (auto i : irange(0L, n)) {
            if (f[i] == 0) {
                l0 = min(l0, k[i]);
            } else {
                l1 = min(l1, k[i]);
            }
        }

        l1 += l0 - 1;
        l0 = 1;

        if (l1 <= 0) {
            cout << 0 << endl;
            return 0;
        }

        cout << l1 << endl;
    }
}