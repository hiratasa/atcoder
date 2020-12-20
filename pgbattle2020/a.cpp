#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t dfs(const vector<vector<int64_t>>& adjs, int64_t v, int64_t p) {
    int64_t ret = 0;
    for (auto u : adjs[v]) {
        if (u == p) {
            continue;
        }

        ret += dfs(adjs, u, v);
    }

    if (ret == 0) {
        return 1;
    } else {
        return ret;
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

    cout << dfs(adjs, 0, -1) << endl;
}