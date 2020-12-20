#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(const vector<vector<pair<int64_t, int64_t>>>& links,
         vector<int64_t>& sz, vector<int64_t>& parents, int64_t v, int64_t p) {
    sz[v] = 1;
    parents[v] = p;

    for (auto [u, _] : links[v]) {
        if (u == p) {
            continue;
        }

        dfs(links, sz, parents, u, v);
        sz[v] += sz[u];
    }
}

void dfs2(const vector<vector<pair<int64_t, int64_t>>>& links,
          vector<int64_t>& depth, int64_t v, int64_t p, int64_t d) {
    for (auto [u, m] : links[v]) {
        if (u == p) {
            continue;
        }

        depth.push_back(d + m);
        dfs2(links, depth, u, v, d + m);
    }
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<pair<int64_t, int64_t>>> links(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b, m;
        cin >> a >> b >> m;
        --a;
        --b;

        links[a].emplace_back(b, m);
        links[b].emplace_back(a, m);
    }

    vector<int64_t> sz(n, 0L), parents(n, 0L);
    dfs(links, sz, parents, 0, -1);
    assert(sz[0] == n);

    int64_t v0 = -1, v1 = -1;
    for (auto i : irange(0L, n)) {
        if (sz[i] == n / 2) {
            v0 = i;
            v1 = parents[i];
            break;
        }
    }

    if (v0 == -1) {
        cout << 0 << endl;
        return 0;
    }

    vector<int64_t> depth1, depth2;
    depth1.push_back(0);
    depth2.push_back(0);
    dfs2(links, depth1, v0, v1, 0);
    dfs2(links, depth2, v1, v0, 0);

    sort(depth1.begin(), depth1.end());
    sort(depth2.begin(), depth2.end());

    int64_t d0 = -1;
    for (auto [u, m] : links[v0]) {
        if (u == v1) {
            d0 = m;
            break;
        }
    }
    assert(d0 > 0);

    int64_t ans = 0;

    for (auto d : depth1) {
        auto it = upper_bound(depth2.begin(), depth2.end(), d - d0);
        auto it2 = upper_bound(depth2.begin(), depth2.end(), d + d0);

        ans += it2 - it;
    }

    for (auto d : depth2) {
        auto it = upper_bound(depth1.begin(), depth1.end(), d - d0);
        auto it2 = upper_bound(depth1.begin(), depth1.end(), d + d0);

        ans += it2 - it;
    }

    cout << ans << endl;
}