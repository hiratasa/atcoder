#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007;

int64_t dfs(const vector<vector<int64_t>>& links, vector<bool>& visited,
            const vector<vector<int64_t>>& p, int64_t offset, int64_t current) {
    visited[current] = true;

    int64_t ret = p[offset][links[current].size() - 1 + offset];
    for (auto v : links[current]) {
        if (visited[v]) {
            continue;
        }

        ret *= dfs(links, visited, p, 0, v);
        ret %= M;
    }

    return ret;
}

main() {
    int64_t n, k;
    cin >> n >> k;

    vector<vector<int64_t>> links(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        links[a].push_back(b);
        links[b].push_back(a);
    }

    vector<vector<int64_t>> p(2, vector<int64_t>(n));
    p[0][0] = 1;
    p[1][0] = 1;
    for (auto i : irange(0L, n - 1)) {
        p[0][i + 1] = p[0][i] * (k - 2 - i);
        p[0][i + 1] %= M;
        p[1][i + 1] = p[1][i] * (k - 1 - i);
        p[1][i + 1] %= M;
    }

    vector<bool> visited(n, false);
    int64_t ans = dfs(links, visited, p, 1, 0);
    ans *= k;
    ans %= M;

    cout << ans << endl;
}
