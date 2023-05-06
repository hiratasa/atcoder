#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    char prev = -1;
    int64_t ans = 0;
    for (auto c : s) {
        if (c != prev) {
            ++ans;
        }

        prev = c;
    }

    cout << ans << endl;
}