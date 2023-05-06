#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

pair<int64_t, bool> dfs(const vector<vector<int64_t>>& adjs,
                        vector<int64_t>& depth, int64_t v, int64_t p) {
    int64_t n = adjs.size();

    int64_t s = 1;
    bool has = (v == n - 1);
    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        depth[u] = depth[v] + 1;

        auto t = dfs(adjs, depth, u, v);
        s += t.first;
        if (t.second) {
            has = true;

            if (depth[v] <= depth[n - 1] / 2) {
                return t;
            }
        }
    }

    return make_pair(s, has);
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

    vector<int64_t> depth(n);
    auto t = dfs(adjs, depth, 0, -1);

    cout << (t.first < (n - t.first) ? "Fennec" : "Snuke") << endl;
}