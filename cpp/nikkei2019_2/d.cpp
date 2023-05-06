#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<pair<int64_t, pair<int64_t, int64_t>>> events;
    vector<int64_t> cost(m);
    for (auto i : irange(0L, m)) {
        int64_t l, r;
        cin >> l >> r >> cost[i];

        events.emplace_back(l, make_pair(1, i));
        events.emplace_back(r, make_pair(-1, i));
    }

    sort(events.begin(), events.end());

    vector<bool> active(m);
    priority_queue<pair<int64_t, int64_t>, vector<pair<int64_t, int64_t>>,
                   greater<pair<int64_t, int64_t>>>
            q;
    constexpr auto kInfinity = numeric_limits<int64_t>::max();
    vector<int64_t> total_cost(n + 1, kInfinity);
    total_cost[1] = 0;
    for (auto e : events) {
        auto current = e.first;

        if (!q.empty()) {
            total_cost[current] = min(total_cost[current], q.top().first);
        }

        if (e.second.first == -1) {
            // R
            active[e.second.second] = false;

            while (!q.empty() && !active[q.top().second]) {
                q.pop();
            }
        } else {
            // L
            if (total_cost[current] != kInfinity) {
                active[e.second.second] = true;
                q.emplace(total_cost[current] + cost[e.second.second],
                          e.second.second);
            }
        }
    }

    if (total_cost[n] == kInfinity) {
        cout << -1 << endl;
    } else {
        cout << total_cost[n] << endl;
    }
}