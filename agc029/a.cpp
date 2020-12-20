#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    reverse(s.begin(), s.end());
    int32_t len_w = 0;
    int64_t ans = 0;
    for (auto c : s) {
        if (c == 'B') {
            if (len_w > 0) {
                ans += len_w;
            }
        } else {
            ++len_w;
        }
    }

    cout << ans << endl;
}