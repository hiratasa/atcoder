#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t dfs(vector<vector<pair<int64_t, pair<int64_t, int64_t>>>>& edges,
            vector<bool>& visited, int64_t v, int64_t goal, int64_t c) {
    visited[v] = true;

    if (v == goal) {
        return c;
    }

    for (auto&& edge : edges[v]) {
        auto u = edge.first;
        if (visited[u]) {
            continue;
        }

        auto&& cap = edge.second.first;

        if (cap == 0) {
            continue;
        }

        c = min(c, cap);

        auto ret = dfs(edges, visited, u, goal, c);
        if (ret > 0) {
            cap -= ret;
            edges[u][edge.second.second].second.first += ret;
            return ret;
        }
    }

    return 0L;
}

int main() {
    int64_t h, w;
    cin >> h >> w;

    vector<string> table(h);
    int64_t s, t, ss, tt;
    for (auto i : irange(0L, h)) {
        string line;
        cin >> line;
        table[i] = line;
        for (auto j : irange(0L, w)) {
            if (line[j] == 'S') {
                s = i;
                ss = j + h;
            } else if (line[j] == 'T') {
                t = i;
                tt = j + h;
            }
        }
    }

    if (s == t || ss == tt) {
        cout << -1 << endl;
        return 0;
    }

    int64_t n = h + w;
    vector<vector<pair<int64_t, pair<int64_t, int64_t>>>> edges(n);
    for (auto i : irange(0L, h)) {
        for (auto j : irange(0L, w)) {
            if (table[i][j] == '.') {
                continue;
            }

            auto v = i;
            auto u = h + j;
            if (u == ss) {
                u = s;
            } else if (u == tt) {
                u = t;
            }
            if (v == u) {
                continue;
            }

            edges[v].emplace_back(u, make_pair(1L, edges[u].size()));
            edges[u].emplace_back(v, make_pair(1L, edges[v].size() - 1));
        }
    }

    // max flow
    // edmons karp?
    int64_t ans = 0;
    while (true) {
        vector<bool> visited(n);
        auto f = dfs(edges, visited, s, t, 100L);
        ans += f;
        if (f == 0) {
            break;
        }
    }

    cout << ans << endl;
}