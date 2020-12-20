#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    constexpr auto M = 1000000007;
    int64_t num = 0;
    int64_t ans = 1;
    for (auto c : s) {
        if (c == 'W') {
            if (num % 2 == 0) {
                ans *= num;
                ans %= M;
                --num;
            } else {
                ++num;
            }
        } else {
            if (num % 2 == 0) {
                ++num;
            } else {
                ans *= num;
                ans %= M;
                --num;
            }
        }
    }

    for (auto i : irange(1L, n + 1)) {
        ans *= i;
        ans %= M;
    }

    if (num > 0) {
        cout << 0 << endl;
    } else {
        cout << ans << endl;
    }
}