#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    int64_t n = s.size();
    vector<int64_t> ans(n);

    int64_t last_r = 0;
    for (auto i : irange(0L, n)) {
        auto c = s[i];

        if (c == 'R') {
            last_r = i;
        } else {
            if ((i - last_r) % 2 == 0) {
                ++ans[last_r];
            } else {
                ++ans[last_r + 1];
            }
        }
    }

    int64_t last_l = n - 1;
    for (auto i : irange(n - 1, -1L, -1L)) {
        auto c = s[i];

        if (c == 'L') {
            last_l = i;
        } else {
            if ((last_l - i) % 2 == 0) {
                ++ans[last_l];
            } else {
                ++ans[last_l - 1];
            }
        }
    }

    const auto* delim = "";
    for (auto a : ans) {
        cout << delim << a;
        delim = " ";
    }
    cout << "\n";
}