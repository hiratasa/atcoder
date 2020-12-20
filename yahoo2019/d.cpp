#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t l;
    cin >> l;

    vector<int64_t> a(l);
    int64_t sum = 0;
    for (auto&& aa : a) {
        cin >> aa;
        sum += aa;
    }

    int64_t unreached = 0;
    int64_t ended = 0;
    int64_t startended = 0;
    int64_t turned = 0;
    int64_t ans = sum;
    int64_t rsum = sum;
    for (auto i : irange(0L, l)) {
        auto is_zero = (a[i] == 0);
        auto is_even = (a[i] % 2 == 0);
        auto next_unreached = min({unreached, ended, startended, turned}) + a[i];
        auto next_ended = min({ended, startended, sum - rsum}) + (is_even ? 1 : 0);
        auto next_startended = min({startended, sum - rsum}) + !is_even + is_zero * 2;
        auto next_turned = min({ended, startended, turned, sum - rsum}) + !is_even + is_zero * 2;

        rsum -= a[i];
        auto next_min = min({next_unreached, next_ended, next_startended, next_turned}) + rsum;
        ans = min(ans, next_min);

        unreached = next_unreached;
        ended = next_ended;
        startended = next_startended;
        turned = next_turned;
    }

    cout << ans << endl;
}