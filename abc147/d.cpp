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
    for (auto&& aa : a) {
        cin >> aa;
    }

    constexpr auto M = 1000000007L;
    int64_t ans = 0;
    int64_t s = 0;
    vector<int64_t> nums(60);
    for (auto i : irange(0L, n)) {
        auto aa = a[i];
        auto ss = s;
        for (auto j : irange(0L, 60L)) {
            if ((aa >> j) & 1uL) {
                ss += ((1L << j) % M) * ((i + (M - 2) * nums[j]) % M);
                ss %= M;
            }

            nums[j] += ((aa >> j) & 1uL);
        }

        ans += ss;
        ans %= M;

        s += aa;
        s %= M;
    }

    cout << ans << endl;
}