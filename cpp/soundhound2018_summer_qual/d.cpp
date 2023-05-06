#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, s, t;
    cin >> n >> m >> s >> t;
    --s;
    --t;

    array<vector<vector<pair<int64_t, int64_t>>>, 2> adjs;
    adjs[0].resize(n);
    adjs[1].resize(n);
    for (auto _ : irange(0L, m)) {
        int64_t u, v, a, b;
        cin >> u >> v >> a >> b;
        --u;
        --v;

        adjs[0][u].emplace_back(v, a);
        adjs[0][v].emplace_back(u, a);
        adjs[1][u].emplace_back(v, b);
        adjs[1][v].emplace_back(u, b);
    }

    array<int64_t, 2> start{s, t};
    vector<vector<int64_t>> costs(
            2, vector<int64_t>(n, numeric_limits<int64_t>::max()));
    for (auto i : irange(0L, 2L)) {
        priority_queue<pair<int64_t, int64_t>> q;
        q.emplace(0, start[i]);
        costs[i][start[i]] = 0;

        while (!q.empty()) {
            auto cost = -q.top().first;
            auto v = q.top().second;

            q.pop();

            if (cost > costs[i][v]) {
                continue;
            }

            for (const auto& edge : adjs[i][v]) {
                auto u = edge.first;
                auto new_cost = cost + edge.second;
                if (new_cost < costs[i][u]) {
                    costs[i][u] = new_cost;
                    q.emplace(-new_cost, u);
                }
            }
        }
    }

    vector<int64_t> ans(n + 1, numeric_limits<int64_t>::max());
    for (auto i : irange(0L, n) | reversed) {
        ans[i] = min(ans[i + 1], costs[0][i] + costs[1][i]);
    }

    for (auto i : irange(0L, n)) {
        cout << 1000000000000000L - ans[i] << "\n";
    }
}