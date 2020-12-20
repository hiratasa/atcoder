#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, k;
    cin >> h >> w >> k;

    vector<vector<bool>> a(h, vector<bool>(w, true));
    int64_t si, sj;
    for (auto i : irange(0L, h)) {
        string s;
        cin >> s;

        for (auto j : irange(0L, w)) {
            if (s[j] == '#') {
                a[i][j] = false;
            } else if (s[j] == 'S') {
                si = i;
                sj = j;
            }
        }
    }

    const int64_t di[4] = {-1L, 1L, 0L, 0L};
    const int64_t dj[4] = {0L, 0L, -1L, 1L};

    vector<vector<int64_t>> dist(
            h, vector<int64_t>(w, numeric_limits<int64_t>::max()));
    queue<pair<int64_t, int64_t>> q;
    dist[si][sj] = 0;
    q.emplace(si, sj);

    int64_t ans = numeric_limits<int64_t>::max();
    while (!q.empty()) {
        auto i = q.front().first;
        auto j = q.front().second;
        q.pop();

        int64_t r = (min({i, j, h - i - 1, w - j - 1}) + k - 1) / k;

        ans = min(ans, r + 1);

        if (r == 0 || dist[i][j] == k) {
            continue;
        }

        for (auto t : irange(0L, 4L)) {
            auto ni = i + di[t];
            auto nj = j + dj[t];

            if (!a[ni][nj]) {
                continue;
            }

            if (dist[ni][nj] > dist[i][j] + 1) {
                dist[ni][nj] = dist[i][j] + 1;
                q.emplace(ni, nj);
            }
        }
    }

    cout << ans << endl;
}