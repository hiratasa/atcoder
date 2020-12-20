#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t m, k;
    cin >> m >> k;

    if (m == 0) {
        if (k == 0) {
            cout << "0 0" << endl;
        } else {
            cout << "-1" << endl;
        }

        return 0;
    }

    if (m == 1) {
        if (k == 0) {
            cout << "0 0 1 1" << endl;
        } else {
            cout << "-1" << endl;
        }

        return 0;
    }

    auto n = (1 << m);
    if (k >= n) {
        cout << "-1" << endl;
        return 0;
    }

    for (int i = 0; i < n; ++i) {
        if (i == k) {
            continue;
        }
        cout << i << " ";
    }

    cout << k << " ";

    for (int i = n - 1; i >= 0; --i) {
        if (i == k) {
            continue;
        }
        cout << i << " ";
    }

    cout << k << endl;
}