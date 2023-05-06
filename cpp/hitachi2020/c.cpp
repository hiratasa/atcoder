#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& adjs, vector<int64_t>& depth, int64_t v,
         int64_t p) {
    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        depth[u] = depth[v] + 1;
        dfs(adjs, depth, u, v);
    }
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

    vector<int64_t> depth(n, 0L);
    dfs(adjs, depth, 0, -1);

    auto n1 = count_if(depth.begin(), depth.end(),
                       [](int64_t d) { return d % 2 == 0; });
    auto n2 = n - n1;

    int64_t r = 0;
    if (n1 > n2) {
        r = 1;
        swap(n1, n2);
    }

    vector<int64_t> ans(n);
    if (n1 <= n / 3) {
        int64_t k1 = 3, k2 = 1;
        for (auto i : irange(0L, n)) {
            if (depth[i] % 2 == r || k2 > n) {
                ans[i] = k1;
                k1 += 3;
            } else {
                ans[i] = k2;
                ++k2;
                if (k2 % 3 == 0) {
                    ++k2;
                }
            }
        }
    } else {
        int64_t k1 = 1, k2 = 2, k3 = 3;
        for (auto i : irange(0L, n)) {
            if (depth[i] % 2 == r) {
                if (k1 <= n) {
                    ans[i] = k1;
                    k1 += 3;
                } else {
                    ans[i] = k3;
                    k3 += 3;
                }
            } else {
                if (k2 <= n) {
                    ans[i] = k2;
                    k2 += 3;
                } else {
                    ans[i] = k3;
                    k3 += 3;
                }
            }
        }
    }

    const auto* delim = "";
    for (auto a : ans) {
        cout << delim << a;
        delim = " ";
    }
    cout << "\n";
}