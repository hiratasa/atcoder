#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    if (n == 100000) {
        n = 99999;
    }
    if (n >= 10000) {
        cout << n - 9999 + 999 - 99 + 9 << endl;
        return 0;
    }
    if (n >= 1000) {
        n = 999;
    }
    if (n >= 100) {
        cout << n - 99 + 9 << endl;
        return 0;
    }
    if (n >= 10) {
        n = 9;
    }
    cout << n << endl;
    return 0;
}