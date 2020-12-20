#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<pair<pair<int64_t, int64_t>, int64_t>> links;
    vector<vector<int64_t>> w(n, vector<int64_t>(n, 1L << 30));
    for (auto _ : irange(0L, m)) {
        int64_t a, b, c;
        cin >> a >> b >> c;
        --a;
        --b;
        w[a][b] = c;
        w[b][a] = c;
        links.emplace_back(make_pair(a, b), c);
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n)) {
            for (auto k : irange(0L, n)) {
                w[j][k] = min(w[j][k], w[j][i] + w[i][k]);
            }
        }
    }

    int64_t ans = 0;
    for (const auto& link : links) {
        if (w[link.first.first][link.first.second] != link.second) {
            ++ans;
        }
    }

    cout << ans << endl;
}