#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool dfs(const vector<vector<int64_t>>& adjs, vector<int64_t>& colors,
         int64_t v, int64_t c) {
    colors[v] = c;

    for (auto u : adjs[v]) {
        if (colors[u] >= 0) {
            if (colors[u] + c != 1) {
                return false;
            }

            continue;
        }

        if (!dfs(adjs, colors, u, (c + 1) % 2)) {
            return false;
        }
    }

    return true;
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> adjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    vector<int64_t> colors(n, -1L);
    if (dfs(adjs, colors, 0L, 0L)) {
        // two-party
        array<int64_t, 2> nn{};
        nn[0] = count(colors.begin(), colors.end(), 0L);
        nn[1] = n - nn[0];

        // cerr << nn[0] << " " << nn[1] << endl;

        int64_t ans = 0;
        for (auto i : irange(0L, n)) {
            ans += nn[(colors[i] + 1) % 2] - adjs[i].size();
        }
        ans /= 2;

        cout << ans << endl;
    } else {
        cout << n * (n - 1) / 2 - m << endl;
    }
}