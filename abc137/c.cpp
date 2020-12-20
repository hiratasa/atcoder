#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    unordered_map<string, int64_t> c;
    for (auto i : irange(0L, n)) {
        string s;
        cin >> s;
        sort(s.begin(), s.end());
        ++c[s];
    }

    int64_t ans = 0;
    for (const auto& kv : c) {
        ans += kv.second * (kv.second - 1) / 2;
    }

    cout << ans << endl;
}