#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    if (k == 1) {
        cout << 0 << endl;
        return 0;
    }

    n -= k;

    cout << n << endl;
}