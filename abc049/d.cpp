#include <bits/stdc++.h>

#include <boost/functional/hash.hpp>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<int64_t>>& adjs, vector<int64_t>& g, int64_t v) {
    for (auto u : adjs[v]) {
        if (g[u] >= 0) {
            continue;
        }

        g[u] = g[v];
        dfs(adjs, g, u);
    }
}

int main() {
    int64_t n, k, l;
    cin >> n >> k >> l;

    vector<vector<int64_t>> road(n), rail(n);
    for (auto i : irange(0L, k)) {
        int64_t p, q;
        cin >> p >> q;
        --p;
        --q;
        road[p].push_back(q);
        road[q].push_back(p);
    }
    for (auto i : irange(0L, l)) {
        int64_t p, q;
        cin >> p >> q;
        --p;
        --q;
        rail[p].push_back(q);
        rail[q].push_back(p);
    }

    vector<int64_t> g1(n, -1), g2(n, -1);
    for (auto i : irange(0L, n)) {
        if (g1[i] < 0) {
            g1[i] = i;
            dfs(road, g1, i);
        }
        if (g2[i] < 0) {
            g2[i] = i;
            dfs(rail, g2, i);
        }
    }

    unordered_map<pair<int64_t, int64_t>, int64_t,
                  boost::hash<std::pair<int64_t, int64_t>>>
            m;
    for (auto i : irange(0L, n)) {
        m[make_pair(g1[i], g2[i])]++;
    }

    for (auto i : irange(0L, n)) {
        cout << m[make_pair(g1[i], g2[i])] << " ";
    }
    cout << endl;
}