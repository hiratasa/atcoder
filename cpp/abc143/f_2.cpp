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

    unordered_map<int64_t, int64_t> nums;
    for (auto kv : counts) {
        ++nums[kv.second];
    }

    int64_t t = counts.size();
    // v[i] == 各グループの数i個以下の部分の合計
    vector<int64_t> v(n + 1);
    for (auto i : irange(0L, n)) {
        v[i + 1] = v[i] + t;
        t -= nums[i + 1];
    }

    for (auto k : irange(1L, n + 1)) {
        if (k > nums.size()) {
            cout << 0 << "\n";
            continue;
        }
        auto r = irange(0L, n + 1);
        auto it = partition_point(r.begin(), r.end(),
                                  [&](int64_t i) { return k * i <= v[i]; });
        auto ans = it - r.begin() - 1;

        cout << ans << "\n";
    }
}