#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w;
    cin >> h >> w;

    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<vector<int64_t>> ans(h, vector<int64_t>(w));
    int64_t y = 0, x = 0, dir = 1;
    for (auto i : irange(0L, n)) {
        for (auto _ : irange(0L, a[i])) {
            ans[y][x] = i + 1;
            x += dir;
            if (x == w) {
                --x;
                ++y;
                dir = -1;
            } else if (x == -1) {
                ++x;
                ++y;
                dir = 1;
            }
        }
    }

    for (auto i : irange(0L, h)) {
        cout << ans[i][0];
        for (auto j : irange(1L, w)) {
            cout << " " << ans[i][j];
        }
        cout << endl;
    }
}