#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, k;
    cin >> h >> w >> k;

    vector<bitset<1000>> bs(h);
    for (auto i : irange(0L, h)) {
        cin >> bs[i];
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto s : irange(0uL, 1uL << (h - 1))) {
        bitset<10> d(s);

        int64_t tmp = d.count();
        vector<int64_t> nums(d.count() + 1);
        for (auto i : irange(0L, w)) {
            int64_t idx = 0;
            bool over_k = false;
            for (auto j : irange(0L, h)) {
                nums[idx] += bs[j][i];

                if (nums[idx] > k) {
                    over_k = true;
                    break;
                }

                if (d[j]) {
                    ++idx;
                }
            }

            if (!over_k) {
                continue;
            }

            ++tmp;

            idx = 0;
            nums.assign(nums.size(), 0);
            over_k = false;
            for (auto j : irange(0L, h)) {
                nums[idx] += bs[j][i];

                if (nums[idx] > k) {
                    over_k = true;
                    break;
                }

                if (d[j]) {
                    ++idx;
                }
            }

            if (over_k) {
                tmp = numeric_limits<int64_t>::max();
                break;
            }
        }

        ans = min(ans, tmp);
    }

    cout << ans << endl;
}