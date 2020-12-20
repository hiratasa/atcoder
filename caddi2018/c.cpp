#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, p;

    cin >> n >> p;

    if (n == 1) {
        cout << p << endl;
        return 0;
    }

    int64_t ans = 1;
    for (int64_t x = pow(p, 1.0/n) + 1; x >= 1; --x) {
        if (p % (int64_t)(pow(x, n) + 0.5) == 0) {
            cout << x << endl;
            break;
        }
    }
}