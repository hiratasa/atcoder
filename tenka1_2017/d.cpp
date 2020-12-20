#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    uint64_t k;
    cin >> n >> k;

    vector<pair<uint64_t, int64_t>> nums(n);
    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        cin >> nums[i].first >> nums[i].second;

        if ((nums[i].first & k) == nums[i].first) {
            ans += nums[i].second;
        }
    }

    for (auto i : irange(0L, 30L)) {
        if ((k & (1uL << i)) == 0) {
            continue;
        }
        auto l = ((k & ~(1uL << i)) | ((1uL << i) - 1));

        int64_t t = 0;
        for (auto j : irange(0L, n)) {
            if ((nums[j].first & l) == nums[j].first) {
                t += nums[j].second;
            }
        }

        ans = max(ans, t);
    }

    cout << ans << endl;
}