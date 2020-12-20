#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t dfs(const vector<vector<int64_t>>& adjs, int64_t v, int64_t p) {
    int64_t t = 0;
    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        auto r = dfs(adjs, u, v);
        if (r == 2) {
            return 2;
        }

        t += r;
    }

    if (t == 0) {
        return 1;
    } else if (t == 1) {
        return 0;
    } else {
        return 2;
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

    auto r = dfs(adjs, 0, -1);
    cout << (r == 0 ? "Second" : "First") << endl;
}