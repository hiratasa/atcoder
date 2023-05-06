#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    string s;
    cin >> s;

    int64_t n = s.size();
    int64_t ans = 0;
    for (auto i : irange(0L, n / 2)) {
        if (s[i] != s[n - i - 1]) {
            ++ans;
        }
    }

    cout << ans << endl;
}