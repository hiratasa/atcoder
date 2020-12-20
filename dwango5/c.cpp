#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int n, q;
    string s;
    cin >> n >> s >> q;

    for (auto i : irange(0, q)) {
        int k;
        cin >> k;

        int64_t num_d = 0;
        int64_t num_m = 0;
        int64_t num_dm = 0;
        int64_t ans = 0;
        for (auto pos : irange(0, n)) {
            auto c = s[pos];
            if (c == 'D') {
                ++num_d;
            } else if (c == 'M') {
                ++num_m;
                num_dm += num_d;
            } else if (c == 'C') {
                ans += num_dm;
            }

            if (pos - k + 1 < 0) {
                continue;
            }
            auto prev = s[pos - k + 1];
            if (prev == 'D') {
                --num_d;
                num_dm -= num_m;
            } else if (prev == 'M') {
                --num_m;
            }
        }

        cout << ans << endl;
    }
}