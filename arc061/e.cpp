#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Link {
    int64_t id;
    int64_t src;
    int64_t dst;
    int64_t company;
};

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<Link> links;
    vector<unordered_map<int64_t, vector<int64_t>>> out_links(n);
    vector<unordered_map<int64_t, int64_t>> costs(n);
    for (auto i : irange(0L, m)) {
        int64_t p, q, c;
        cin >> p >> q >> c;
        --p;
        --q;
        links.push_back(Link{2 * i, p, q, c});
        links.push_back(Link{2 * i + 1, q, p, c});
        out_links[p][c].push_back(2 * i);
        out_links[q][c].push_back(2 * i + 1);
        costs[p][c] = numeric_limits<int64_t>::max();
        costs[q][c] = numeric_limits<int64_t>::max();
    }

    for (auto i : irange(0L, n)) {
        costs[i][-1] = numeric_limits<int64_t>::max();
    }

    priority_queue<pair<int64_t, pair<int64_t, int64_t>>> q;
    costs[0][-1] = 0;
    for (const auto& kv : out_links[0]) {
        for (auto link_id : kv.second) {
            const auto& link = links[link_id];
            costs[link.dst][link.company] = 1;
            q.emplace(-1, make_pair(link.dst, link.company));
        }
    }

    while (!q.empty()) {
        auto cost = -q.top().first;
        auto node = q.top().second.first;
        auto company = q.top().second.second;
        q.pop();

        if (cost > costs[node][company]) {
            continue;
        }

        if (company >= 0) {
            for (auto next_link_id : out_links[node][company]) {
                const auto& next_link = links[next_link_id];
                auto next_cost = cost;

                if (next_cost < costs[next_link.dst][next_link.company]) {
                    costs[next_link.dst][next_link.company] = next_cost;
                    q.emplace(-next_cost,
                              make_pair(next_link.dst, next_link.company));
                }
            }

            auto next_cost = cost;
            if (next_cost < costs[node][-1]) {
                costs[node][-1] = next_cost;
                q.emplace(-next_cost, make_pair(node, -1));
            }

            continue;
        }

        if (node == n - 1) {
            cout << cost << endl;
            return 0;
        }

        for (const auto& kv : out_links[node]) {
            for (auto next_link_id : kv.second) {
                const auto& next_link = links[next_link_id];
                auto next_cost = cost + 1;

                if (next_cost < costs[next_link.dst][next_link.company]) {
                    costs[next_link.dst][next_link.company] = next_cost;
                    q.emplace(-next_cost,
                              make_pair(next_link.dst, next_link.company));
                }
            }
        }
    }

    cout << -1 << endl;
}
