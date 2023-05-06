#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    int64_t ch, cw, dh, dw;
    cin >> ch >> cw >> dh >> dw;

    vector maze(h + 2, vector(w + 2, false));
    for (auto i : irange(0L, h)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, w)) {
            if (s[j] == '.') {
                maze[i + 1][j + 1] = true;
            }
        }
    }

    vector costs(h + 2, vector(w + 2, numeric_limits<int64_t>::max()));
    list<pair<int64_t, int64_t>> q;
    costs[ch][cw] = 0;
    q.emplace_back(ch, cw);
    while (!q.empty()) {
        auto [eh, ew] = q.front();
        q.pop_front();

        if (eh == dh && ew == dw) {
            cout << costs[dh][dw] << endl;
            return 0;
        }

        // cerr << eh << "," << ew << " " << costs[eh][ew] << endl;

        for (auto dy : irange(-2L, 3L)) {
            for (auto dx : irange(-2L, 3L)) {
                auto nh = eh + dy;
                auto nw = ew + dx;
                if (nh <= 0 || nh > h || nw <= 0 || nw > w) {
                    continue;
                }
                if (!maze[nh][nw]) {
                    continue;
                }

                auto edge_cost = (abs(dy) + abs(dx) == 1 ? 0 : 1);
                auto cost = costs[eh][ew] + edge_cost;
                if (cost < costs[nh][nw]) {
                    costs[nh][nw] = cost;
                    if (edge_cost == 0) {
                        q.emplace_front(nh, nw);
                    } else {
                        q.emplace_back(nh, nw);
                    }
                }
            }
        }
    }

    cout << -1 << endl;
}