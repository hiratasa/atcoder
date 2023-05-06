#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr auto M = 1000000007L;

void dijkstra(const vector<vector<pair<int64_t, int64_t>>>& adjs,
              vector<int64_t>& costs, vector<int64_t>& nums, int64_t s) {
    int64_t n = adjs.size();

    costs.resize(n, 1L << 50);
    nums.resize(n, 0L);

    costs[s] = 0;
    nums[s] = 1;

    priority_queue<pair<int64_t, int64_t>, vector<pair<int64_t, int64_t>>,
                   std::greater<>>
            q;
    q.emplace(0L, s);

    while (!q.empty()) {
        auto cost = q.top().first;
        auto v = q.top().second;

        q.pop();

        if (cost > costs[v]) {
            continue;
        }

        for (const auto& link : adjs[v]) {
            auto u = link.first;
            auto u_cost = cost + link.second;

            if (u_cost < costs[u]) {
                costs[u] = u_cost;
                q.emplace(u_cost, u);
                nums[u] = nums[v];
            } else if (u_cost == costs[u]) {
                nums[u] += nums[v];
                nums[u] %= M;
            }
        }
    }
}

int main() {
    int64_t n, m, s, t;
    cin >> n >> m >> s >> t;

    --s;
    --t;

    vector<vector<pair<int64_t, int64_t>>> adjs(n);
    for (auto _ : irange(0L, m)) {
        int64_t u, v, d;
        cin >> u >> v >> d;
        --u;
        --v;
        adjs[u].emplace_back(v, d);
        adjs[v].emplace_back(u, d);
    }

    array<vector<int64_t>, 2> costs, nums;
    dijkstra(adjs, costs[0], nums[0], s);
    dijkstra(adjs, costs[1], nums[1], t);

    auto dist = costs[0][t];
    assert(dist == costs[1][s]);

    int64_t ans = nums[0][t] * nums[1][s] % M;
    for (auto v : irange(0L, n)) {
        if (2 * costs[0][v] == dist && 2 * costs[1][v] == dist) {
            ans += M - nums[0][v] * nums[0][v] % M * nums[1][v] % M *
                               nums[1][v] % M;
            ans %= M;
        }

        if (2 * costs[0][v] < dist && costs[0][v] + costs[1][v] == dist) {
            for (const auto& link : adjs[v]) {
                auto u = link.first;
                auto cost = link.second;

                if (costs[0][v] + cost + costs[1][u] == dist &&
                    2 * costs[0][u] > dist) {
                    ans += M - nums[0][v] * nums[0][v] % M * nums[1][u] % M *
                                       nums[1][u] % M;
                    ans %= M;
                }
            }
        }
    }

    cout << ans << endl;
}