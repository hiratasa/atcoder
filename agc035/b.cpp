#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

bool dfs(const vector<vector<pair<int64_t, int64_t>>>& links,
         vector<bool>& visited, vector<bool>& used,
         vector<pair<int64_t, int64_t>>& out, int64_t current) {
    if (visited[current]) {
        return true;
    }

    visited[current] = true;

    int64_t c = 0;
    for (auto kv : links[current]) {
        auto next = kv.first;
        auto idx = kv.second;
        if (used[idx]) {
            continue;
        }

        used[idx] = true;
        if (dfs(links, visited, used, out, next)) {
            ++c;
            out.emplace_back(current, next);
        } else {
            out.emplace_back(next, current);
        }
    }

    return c % 2 == 0;
}

main() {
    int64_t n, m;
    cin >> n >> m;

    if (m % 2 > 0) {
        cout << -1 << endl;
        return 0;
    }

    vector<vector<pair<int64_t, int64_t>>> links(n);
    for (auto i : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        links[a].emplace_back(b, i);
        links[b].emplace_back(a, i);
    }

    vector<bool> visited(n), used(m);
    vector<pair<int64_t, int64_t>> out;
    out.reserve(m);
    dfs(links, visited, used, out, 0);

    for (const auto& link : out) {
        cout << link.first + 1 << " " << link.second + 1 << "\n";
    }
}