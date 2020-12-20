#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 5000L;

void dijkstra(const vector<vector<pair<int64_t, pair<int64_t, int64_t>>>>& adjs,
              const vector<pair<int64_t, int64_t>>& cd,
              vector<vector<int64_t>>& costs, int64_t s) {
    using Entry = std::tuple<int64_t, int64_t, int64_t>;

    priority_queue<Entry, vector<Entry>, std::greater<>> q;

    costs[0][min(s, M)] = 0;
    q.emplace(0L, 0L, min(s, M));

    while (!q.empty()) {
        auto [cost, v, money] = q.top();
        q.pop();

        if (costs[v][money] < cost) {
            continue;
        }

        {
            auto next_money = min(money + cd[v].first, M);
            auto next_cost = cost + cd[v].second;

            if (next_cost < costs[v][next_money]) {
                costs[v][next_money] = next_cost;
                q.emplace(next_cost, v, next_money);
            }
        }

        for (const auto& link : adjs[v]) {
            auto u = link.first;

            auto next_money = min(money - link.second.first, M);
            if (next_money < 0) {
                continue;
            }

            auto next_cost = cost + link.second.second;

            if (next_cost < costs[u][next_money]) {
                costs[u][next_money] = next_cost;
                q.emplace(next_cost, u, next_money);
            }
        }
    }
}

int main() {
    int64_t n, m, s;
    cin >> n >> m >> s;

    vector adjs(n, vector<pair<int64_t, pair<int64_t, int64_t>>>());
    for (auto _ : irange(0L, m)) {
        int64_t u, v, a, b;
        cin >> u >> v >> a >> b;
        --u;
        --v;
        adjs[u].emplace_back(v, make_pair(a, b));
        adjs[v].emplace_back(u, make_pair(a, b));
    }

    vector cd(n, make_pair(0L, 0L));
    for (auto&& t : cd) {
        cin >> t.first >> t.second;
    }

    vector costs(n, vector(M + 1L, numeric_limits<int64_t>::max()));
    dijkstra(adjs, cd, costs, s);

    for (auto i : irange(1L, n)) {
        cout << *min_element(costs[i].begin(), costs[i].end()) << "\n";
    }
}