#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t dijkstra(const vector<vector<int64_t>>& links,
                 vector<vector<bool>>& visited, int64_t idx, int64_t s,
                 int64_t t) {}

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> links(n);
    for (auto _ : irange(0L, m)) {
        int64_t u, v;
        cin >> u >> v;
        --u;
        --v;
        links[u].push_back(v);
    }

    int64_t s, t;
    cin >> s >> t;
    --s;
    --t;

    auto kInfinity = numeric_limits<int32_t>::max();

    list<pair<int64_t, int64_t>> q;
    vector<vector<int64_t>> cost(3, vector<int64_t>(n, kInfinity));

    q.emplace_back(0L, s);
    cost[0][s] = 0;
    while (!q.empty()) {
        auto current = q.front();
        q.pop_front();

        auto next_cost = cost[current.first][current.second] + 1;
        auto next_idx = (current.first + 1) % 3;
        for (auto v : links[current.second]) {
            if (cost[next_idx][v] < kInfinity) {
                // already visited
                continue;
            }

            if (next_idx == 0 && v == t) {
                cout << next_cost / 3 << endl;
                return 0;
            }

            cost[next_idx][v] = next_cost;
            q.emplace_back(next_idx, v);
        }
    }

    cout << -1 << endl;
}