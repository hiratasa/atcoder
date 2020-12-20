#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

array<int64_t, 2> dfs(const vector<vector<bool>>& connected, vector<int64_t>& g,
                      int64_t v) {
    int64_t n = connected.size();

    array<int64_t, 2> ret{};
    ++ret[g[v]];
    for (auto u : irange(0L, n)) {
        if (connected[v][u] || u == v) {
            continue;
        }

        if (g[u] >= 0) {
            if (g[u] + g[v] != 1) {
                return {-1L, -1L};
            }
        } else {
            g[u] = (g[v] + 1) % 2;
            auto tmp = dfs(connected, g, u);
            if (tmp[0] < 0) {
                return {-1L, -1L};
            }
            ret[0] += tmp[0];
            ret[1] += tmp[1];
        }
    }

    return ret;
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<bool>> connected(n, vector<bool>(n));
    for (auto _ : irange(0L, m)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        connected[a][b] = true;
        connected[b][a] = true;
    }

    vector<int64_t> g(n, -1L);
    int64_t s = 0;
    vector<int64_t> p;
    for (auto i : irange(0L, n)) {
        if (g[i] < 0) {
            g[i] = 0;
            auto t = dfs(connected, g, i);
            if (t[0] < 0) {
                cout << -1 << endl;
                return 0;
            }
            if (t[0] > t[1]) {
                swap(t[0], t[1]);
            }
            s += t[0];
            p.push_back(t[1] - t[0]);
        }
    }

    bitset<701> bs;
    bs[s] = 1;
    for (auto pp : p) {
        bs |= (bs << pp);
    }

    for (auto i : irange(0L, n / 2 + 1) | reversed) {
        if (bs[i]) {
            cout << i * (i - 1) / 2 + (n - i) * (n - i - 1) / 2 << endl;
            return 0;
        }
    }
}