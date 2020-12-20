#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    int64_t ans = 0;
    int64_t len_a = 0;
    for (int64_t i = 0; i < s.size(); ++i) {
        if (s[i] == 'A') {
            ++len_a;
        } else if (s[i] == 'B') {
            if (i + 1 < s.size() && s[i + 1] == 'C') {
                ans += len_a;
                ++i;
            } else {
                len_a = 0;
            }
        } else {
            len_a = 0;
        }
    }

    cout << ans << endl;
}