#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t x, y;
    cin >> x >> y;

    int64_t ans = 0;
    for (auto r : {x, y}) {
        switch (r) {
            case 1:
                ans += 300000;
                break;
            case 2:
                ans += 200000;
                break;
            case 3:
                ans += 100000;
                break;
        }
    }

    if (x == y && x == 1) {
        ans += 400000;
    }

    cout << ans << endl;
}