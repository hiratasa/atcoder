#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t calc(const vector<pair<int64_t, int64_t>>& v,
             vector<pair<int64_t, int64_t>>& vy,
             vector<pair<int64_t, int64_t>>& vx) {
    int64_t x = 0, y = 0;

    for (auto xy : v) {
        x += xy.first;
        y += xy.second;
    }

    sort(vy.begin(), vy.end(), [](const auto& xy0, const auto& xy1) {
        return xy0.second * xy1.first < xy1.second * xy0.first;
    });
    sort(vx.begin(), vx.end(), [](const auto& xy0, const auto& xy1) {
        return xy0.second * xy1.first > xy1.second * xy0.first;
    });

    int64_t dist = x * x + y * y;

    int64_t dx2 = 0, dy2 = 0;
    for (const auto& xy1 : vx) {
        dx2 -= xy1.first;
        dy2 += xy1.second;

        int64_t tmp_dist = (x + dx2) * (x + dx2) + (y + dy2) * (y + dy2);
        if (dist < tmp_dist) {
            dist = tmp_dist;
        }
    }

    int64_t dx = 0, dy = 0;
    for (const auto& xy0 : vy) {
        dx += xy0.first;
        dy -= xy0.second;

        int64_t tmp_dist0 = (x + dx) * (x + dx) + (y + dy) * (y + dy);
        if (dist < tmp_dist0) {
            dist = tmp_dist0;
        }

        int64_t dx2 = dx, dy2 = dy;
        for (const auto& xy1 : vx) {
            dx2 -= xy1.first;
            dy2 += xy1.second;

            int64_t tmp_dist = (x + dx2) * (x + dx2) + (y + dy2) * (y + dy2);
            if (dist < tmp_dist) {
                dist = tmp_dist;
            }
        }
    }

    return dist;
}

main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> pp, pn, np, nn;
    for (auto i : irange(0L, n)) {
        int64_t x, y;
        cin >> x >> y;

        if (x == 0 && y == 0) {
            continue;
        }

        if (x >= 0) {
            if (y >= 0) {
                pp.emplace_back(x, y);
            } else {
                pn.emplace_back(x, -y);
            }
        } else {
            if (y >= 0) {
                np.emplace_back(-x, y);
            } else {
                nn.emplace_back(-x, -y);
            }
        }
    }

    int64_t dist = 0;
    dist = max(dist, calc(pp, pn, np));
    cerr << dist << endl;
    dist = max(dist, calc(pn, pp, nn));
    cerr << dist << endl;
    dist = max(dist, calc(np, nn, pp));
    cerr << dist << endl;
    dist = max(dist, calc(nn, np, pn));

    cout << setprecision(15) << sqrt(dist) << endl;
}