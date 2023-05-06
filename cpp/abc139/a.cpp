#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s, t;
    cin >> s >> t;

    int64_t ans = 0;
    for (auto i : irange(0L, 3L)) {
        if (s[i] == t[i]) {
            ++ans;
        }
    }

    cout << ans << endl;
}