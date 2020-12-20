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

    set<int64_t> remains;
    for (auto i : irange(0L, n)) {
        remains.insert(i);
    }
    int64_t l = 0, r = n - 1;
    for (auto c : s) {
        if (r - l < 0) {
            auto x = *remains.begin();
            if (c == 'L') {
            } else {
                x = *remains.rbegin();
            }
            cout << x + 1 << "\n";
            remains.erase(x);
        } else {
            int64_t x = l;
            if (c == 'L') {
                x = l;
                l += 2;
            } else {
                x = r;
                r -= 2;
            }

            cout << x + 1 << "\n";
            remains.erase(x);
        }
    }
}