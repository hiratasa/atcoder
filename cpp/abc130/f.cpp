#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> r, l, u, d;
    int64_t maxr_x = numeric_limits<int64_t>::min(), maxl_x = numeric_limits<int64_t>::min(), maxud_x = numeric_limits<int64_t>::min();
    int64_t minr_x = numeric_limits<int64_t>::max(), minl_x = numeric_limits<int64_t>::max(), minud_x = numeric_limits<int64_t>::max();
    int64_t maxu_y = numeric_limits<int64_t>::min(), maxd_y = numeric_limits<int64_t>::min(), maxrl_y = numeric_limits<int64_t>::min();
    int64_t minu_y = numeric_limits<int64_t>::max(), mind_y = numeric_limits<int64_t>::max(), minrl_y = numeric_limits<int64_t>::max();
    for (auto _ : irange(0L, n)) {
        int64_t x, y;
        char dir;
        cin >> x >> y >> dir;

        if (dir == 'R') {
            r.emplace_back(x, y);
            maxr_x = max(maxr_x, x);
            maxrl_y = max(maxrl_y, x);
            minr_x = min(minr_x, x);
            minrl_y = min(minrl_y, x);
        } else if (dir == 'L') {
            l.emplace_back(x, y);
            maxl_x = max(maxl_x, x);
            maxrl_y = max(maxrl_y, x);
            minl_x = min(minl_x, x);
            minrl_y = min(minrl_y, x);
        } else if (dir == 'U') {
            u.emplace_back(x, y);
            maxud_x = max(maxud_x, x);
            maxu_y = max(maxu_y, x);
            minud_x = min(minud_x, x);
            minu_y = min(minu_y, x);
        } else if (dir == 'D') {
            d.emplace_back(x, y);
            maxud_x = max(maxud_x, x);
            maxd_y = max(maxd_y, x);
            minud_x = min(minud_x, x);
            mind_y = min(mind_y, x);
        }
    }

    int64_t posmaxx1, posmaxx2;
    if (maxr_x >= maxl_x && maxr_x >= maxud_x) {
        posmaxx1 = posmaxx2 = 0;
    } else if (maxud_x > maxr_x && maxud_x >= maxl_x) {
        posmaxx1 = 0;
        posmaxx2 = maxud_x - maxr_x;
    } else if (maxl_x - maxud_x <= maxud_x - maxr_x) {
        posmaxx1 = maxl_x - maxud_x;
        posmaxx2 = maxud_x - maxr_x;
    } else {


    }

}