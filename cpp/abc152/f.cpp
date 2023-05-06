#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool dfs(const vector<vector<pair<int64_t, int64_t>>>& links,
         vector<bitset<20>>& bs, int64_t current, int64_t parent, int64_t goal,
         int64_t idx) {
    if (current == goal) {
        return true;
    }

    for (auto link : links[current]) {
        auto v = link.first;
        if (v == parent) {
            continue;
        }

        auto tmp = dfs(links, bs, v, current, goal, idx);
        if (tmp) {
            bs[link.second][idx] = true;
            return true;
        }
    }

    return false;
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<pair<int64_t, int64_t>>> links(n);
    for (auto i : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        links[a].emplace_back(b, i);
        links[b].emplace_back(a, i);
    }

    int64_t m;
    cin >> m;

    vector<bitset<20>> paths(n - 1);
    for (auto i : irange(0L, m)) {
        int64_t u, v;
        cin >> u >> v;
        --u;
        --v;

        dfs(links, paths, u, -1, v, i);
    }

    int64_t ans = 1L << (n - 1);
    for (auto k : irange(1uL, 1uL << m)) {
        bitset<20> bs(k);

        int64_t num_white = 0;
        for (auto i : irange(0L, n - 1)) {
            if ((paths[i] & bs).any()) {
                ++num_white;
            }
        }

        int64_t tmp = 1L << (n - 1 - num_white);
        // cerr << bs.to_string() << ":" << tmp << "\n";
        if (bs.count() % 2 == 0) {
            ans += tmp;
        } else {
            ans -= tmp;
        }
    }

    cout << ans << endl;
}