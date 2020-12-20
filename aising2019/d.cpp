#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, q;
    cin >> n >> q;

    vector<int64_t> a(n);
    vector<int64_t> sum;
    vector<int64_t> psum;
    for (auto&& aa : a) {
        cin >> aa;

        sum.push_back(sum.empty() ? 0 : sum.back());
        sum.back() += aa;
        psum.push_back(psum.size() >= 2 ? psum[psum.size() - 2] : 0);
        psum.back() += aa;
    }

    for (auto _ : irange(0L, q)) {
        int64_t x;
        cin >> x;

        auto r = irange(0L, n);
        auto i = *partition_point(r.begin(), r.end(), [x, n, &a](int64_t i) {
            // i ～ n-1 番目までを全て先手がとるときにfalse
            auto k = n - i;

            auto j = n - 2 * k + 1;
            if (j < 0) {
                return true;
            }

            auto c = a[j];
            auto d = a[i];

            if (abs(c - x) <= abs(d - x)) {
                return false;
            }
            return true;
        });


        int64_t ans = 0;
        ans += sum.back() - sum[i - 1];

        auto k = n - i;
        auto j = n - 2 * k - 1;
        if (j >= 0) {
            ans += psum[j];
        }
        cout << ans << endl;
    }
}