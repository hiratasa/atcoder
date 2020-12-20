#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t dfs(const vector<unordered_set<int64_t>>& friends, vector<int64_t>& g,
            int64_t r, int64_t v) {
    if (g[v] >= 0) {
        return 0;
    }

    g[v] = r;

    int64_t ret = 1;
    for (auto u : friends[v]) {
        ret += dfs(friends, g, r, u);
    }

    return ret;
}

int main() {
    int64_t n, m, k;
    cin >> n >> m >> k;

    vector<unordered_set<int64_t>> friends(n), blocks(n);
    for (auto i : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        friends[a].insert(b);
        friends[b].insert(a);
    }
    for (auto i : irange(0L, k)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        blocks[a].insert(b);
        blocks[b].insert(a);
    }

    vector<int64_t> g(n, -1), c(n);
    for (auto i : irange(0L, n)) {
        c[i] = dfs(friends, g, i, i);
    }

    const auto* delim = "";
    for (auto i : irange(0L, n)) {
        int64_t tmp = c[g[i]] - /* self */ 1 - friends[i].size();
        // 高々K回しか回らない
        for (auto v : blocks[i]) {
            if (g[i] == g[v]) {
                --tmp;
            }
        }
        cout << delim << tmp;
        delim = " ";
    }
    cout << endl;
}