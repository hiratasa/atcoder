#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t h, w, k;
    cin >> h >> w >> k;

    vector<vector<bool>> m(h, vector<bool>(w));
    vector<int64_t> nums(h);
    for (auto i : irange(0L, h)) {
        string s;
        cin >> s;
        for (auto j : irange(0L, w)) {
            m[i][j] = (s[j] == '#');
            nums[i] += (m[i][j] ? 1 : 0);
        }
    }

    int64_t idx = 1;
    vector<vector<int64_t>> ans(h, vector<int64_t>(w));
    int64_t i0 = 0;
    while (nums[i0] == 0) {
        ++i0;
    }
    for (auto i : irange(i0, h)) {
        if (nums[i] > 0) {
            for (auto j : irange(0L, w)) {
                ans[i][j] = idx;
                if (m[i][j] && idx < ans[i][0] + nums[i] - 1) {
                    ++idx;
                }
                ++j;
            }
            ++idx;
        } else {
            for (auto j : irange(0L, w)) {
                ans[i][j] = ans[i - 1][j];
            }
        }
    }

    for (auto i : irange(0L, i0)) {
        for (auto j : irange(0L, w)) {
            ans[i][j] = ans[i0][j];
        }
    }

    for (auto i : irange(0L, h)) {
        const auto* delim = "";
        for (auto j : irange(0L, w)) {
            cout << delim << ans[i][j];
            delim = " ";
        }
        cout << "\n";
    }
}