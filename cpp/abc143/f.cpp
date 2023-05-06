#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    unordered_map<int64_t, int64_t> counts;
    for (auto&& aa : a) {
        cin >> aa;
        ++counts[aa];
    }

    vector<int64_t> nums;
    nums.reserve(counts.size());
    for (auto kv : counts) {
        nums.push_back(kv.second);
    }

    sort(nums.rbegin(), nums.rend());

    vector<int64_t> cum(nums.size() + 1);
    for (auto i : irange(0uL, nums.size())) {
        cum[i + 1] = cum[i] + nums[i];
    }

    for (auto k : irange(1L, n + 1)) {
        if (k > nums.size()) {
            cout << 0 << "\n";
            continue;
        }
        auto r = irange(0L, k - 1);
        auto it = partition_point(r.begin(), r.end(), [&](int64_t i) {
            return cum.back() - cum[i + 1] < (k - i - 1) * nums[i];
        });
        auto idx = it - r.begin();

        auto ans = (cum.back() - cum[idx]) / (k - idx);
        cout << ans << "\n";
    }
}