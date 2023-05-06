#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

double dijkstra(const vector<vector<int64_t>>& l, int64_t si, int64_t sj,
                int64_t gi, int64_t gj) {
    const int64_t di[]{-1, 1, 0, 0};
    const int64_t dj[]{0, 0, -1, 1};

    int64_t n = l.size();
    int64_t m = l[0].size();

    vector<vector<double>> c(n + 2, vector<double>(m + 2, 0));
    priority_queue<pair<double, pair<int64_t, int64_t>>> q;
    c[gi][gj] = 10;
    q.emplace(10, make_pair(gi, gj));

    while (!q.empty()) {
        auto cc = q.top().first;
        auto i = q.top().second.first;
        auto j = q.top().second.second;
        q.pop();

        if (i == si && j == sj) {
            return cc;
        }

        for (auto t : irange(0L, 4L)) {
            auto ni = i + di[t];
            auto nj = j + dj[t];

            if (l[ni][nj] == 0) {
                continue;
            }

            auto nc = min(cc * 0.99, (double)l[ni][nj]);

            if (nc > c[ni][nj]) {
                c[ni][nj] = nc;
                q.emplace(nc, make_pair(ni, nj));
            }
        }
    }

    return -1;
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> l(n + 2, vector<int64_t>(m + 2));
    int64_t si, sj, gi, gj;
    for (auto i : irange(1L, n + 1)) {
        string s;
        cin >> s;
        for (auto j : irange(1L, m + 1)) {
            switch (s[j - 1]) {
                case 's': {
                    si = i;
                    sj = j;
                    l[i][j] = 10;
                    break;
                }
                case 'g': {
                    gi = i;
                    gj = j;
                    l[i][j] = 10;
                    break;
                }
                case '#': {
                    l[i][j] = 0;
                    break;
                }
                default: {
                    l[i][j] = s[j - 1] - '0';
                    break;
                }
            }
        }
    }

    double ans = dijkstra(l, si, sj, gi, gj);
    if (ans < 0) {
        cout << -1 << endl;
    } else {
        cout << setprecision(20) << ans << endl;
    }
}