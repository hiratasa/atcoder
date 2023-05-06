#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<std::tuple<string, string, int64_t>> m;
    for (auto i : irange(0L, n)) {
        string s;
        int64_t c;
        cin >> s >> c;

        string r = s;
        reverse(r.begin(), r.end());

        m.emplace_back(std::move(s), std::move(r), c);
    }

    vector<pair<bool, string_view>> nodes;
    array<unordered_map<string_view, int64_t>, 2> idx;
    nodes.emplace_back(false, "");
    for (const auto& [s, r, c] : m) {
        const auto& sv = string_view(s);
        const auto& rv = string_view(r);
        for (auto sz : irange(1uL, s.size() + 1)) {
            const auto& ls = sv.substr(s.size() - sz);
            if (idx[0].count(ls) == 0) {
                nodes.emplace_back(false, ls);
                idx[0][ls] = nodes.size() - 1;
            }

            const auto& rs = rv.substr(r.size() - sz);
            if (idx[1].count(rs) == 0) {
                nodes.emplace_back(true, rs);
                idx[1][rs] = nodes.size() - 1;
            }
        }
    }

    int64_t num_nodes = nodes.size();
    vector adjs(num_nodes, vector(0L, make_pair(0L, 0L)));

    for (const auto& [s, r, c] : m) {
        adjs[0].emplace_back(idx[0].at(string_view(s)), c);
        adjs[0].emplace_back(idx[1].at(string_view(r)), c);
    }

    for (auto i : irange(1L, num_nodes)) {
        int is_right = nodes[i].first;
        int opposite = (is_right + 1) % 2;
        const auto& sv = nodes[i].second;

        string tmp(sv);
        reverse(tmp.begin(), tmp.end());
        if (tmp == sv) {
            adjs[i].emplace_back(0, 0);
            continue;
        }

        for (const auto& [s, r, c] : m) {
            string_view rr = is_right ? s : r;
            if (rr.size() < sv.size()) {
                if (rr == sv.substr(0, rr.size())) {
                    auto sv2 = sv.substr(rr.size());
                    adjs[i].emplace_back(idx[is_right].at(sv2), c);
                }
            } else if (rr.size() == sv.size()) {
                if (rr == sv) {
                    adjs[i].emplace_back(0, c);
                }
            } else {
                if (rr.substr(0, sv.size()) == sv) {
                    auto sv2 = rr.substr(sv.size());
                    adjs[i].emplace_back(idx[opposite].at(sv2), c);
                }
            }
        }
    }

    // dump
    // for (auto i : irange(0L, num_nodes)) {
    //     cerr << i << " = (" << (int)nodes[i].first << "," << nodes[i].second
    //          << ")" << endl;
    //     for (auto [next, cost] : adjs[i]) {
    //         cerr << " to " << next << ", " << cost << endl;
    //     }
    // }

    priority_queue<pair<int64_t, int64_t>, std::vector<pair<int64_t, int64_t>>,
                   std::greater<>>
            q;
    vector costs(num_nodes, numeric_limits<int64_t>::max());

    q.emplace(0L, 0L);
    // costs[0] = 0;
    while (!q.empty()) {
        auto [cost, v] = q.top();
        q.pop();

        if (cost > 0 && v == 0) {
            cout << cost << endl;
            return 0;
        }

        if (cost > costs[v]) {
            continue;
        }

        for (auto [next, edge_cost] : adjs[v]) {
            auto next_cost = cost + edge_cost;

            if (next_cost < costs[next]) {
                costs[next] = next_cost;
                q.emplace(next_cost, next);
            }
        }
    }

    cout << -1 << endl;
}